//! Supports application game service stack.

use {
    super::{common::DomainEvents, GameService},
    crate::{
        application::{EventBus, EventStore},
        domain::play::{
            commands::{
                ActivateAbilityCommand, CastSpellCommand, PassPriorityCommand,
                ResolveOptionalEffectCommand, ResolvePendingHandChoiceCommand,
                ResolvePendingScryCommand, ResolvePendingSurveilCommand,
            },
            errors::DomainError,
            events::{
                CardDiscarded, CardDrawn, CardExiled, CardMovedZone, CreatureDied, DomainEvent,
                GameEnded, LifeChanged, SpellCast, StackTopResolved, TriggeredAbilityPutOnStack,
            },
            game::{
                ActivateAbilityOutcome, CastSpellOutcome, Game, PassPriorityOutcome,
                ResolveOptionalEffectOutcome, ResolvePendingHandChoiceOutcome,
                ResolvePendingScryOutcome, ResolvePendingSurveilOutcome,
            },
        },
    },
};

struct ResolutionEffectBatch<'a> {
    card_drawn: &'a [CardDrawn],
    card_discarded: Option<&'a CardDiscarded>,
    card_exiled: Option<&'a CardExiled>,
    zone_changes: &'a [CardMovedZone],
    life_changed: Option<&'a LifeChanged>,
    creatures_died: &'a [CreatureDied],
}

fn push_resolution_effect_batch(
    domain_events: &mut DomainEvents,
    batch: &ResolutionEffectBatch<'_>,
) {
    domain_events.extend(batch.card_drawn.iter().cloned());
    domain_events.push_optional(batch.card_discarded.cloned());
    domain_events.push_optional(batch.card_exiled.cloned());
    domain_events.extend(batch.zone_changes.iter().cloned());
    domain_events.push_optional(batch.life_changed.cloned());
    domain_events.extend(batch.creatures_died.iter().cloned());
}

fn push_resolution_effect_sequence(
    domain_events: &mut DomainEvents,
    batches: &[ResolutionEffectBatch<'_>],
) {
    for batch in batches {
        push_resolution_effect_batch(domain_events, batch);
    }
}

fn push_resolution_close_events(
    domain_events: &mut DomainEvents,
    stack_top_resolved: Option<&StackTopResolved>,
    spell_cast: Option<&SpellCast>,
) {
    domain_events.push_optional(stack_top_resolved.cloned());
    domain_events.push_optional(spell_cast.cloned());
}

fn push_resolution_follow_up_events(
    domain_events: &mut DomainEvents,
    triggered_abilities_put_on_stack: &[TriggeredAbilityPutOnStack],
    game_ended: Option<&GameEnded>,
) {
    domain_events.extend(triggered_abilities_put_on_stack.iter().cloned());
    domain_events.push_optional(game_ended.cloned());
}

pub fn domain_events_for_activate_ability(outcome: &ActivateAbilityOutcome) -> Vec<DomainEvent> {
    let mut domain_events = DomainEvents::default();
    domain_events.extend(outcome.zone_changes.iter().cloned());
    domain_events.extend(outcome.creatures_died.iter().cloned());
    domain_events.push(outcome.activated_ability_put_on_stack.clone());
    domain_events.into_vec()
}

pub fn domain_events_for_cast_spell(outcome: &CastSpellOutcome) -> Vec<DomainEvent> {
    vec![outcome.spell_put_on_stack.clone().into()]
}

pub fn domain_events_for_pass_priority(outcome: &PassPriorityOutcome) -> Vec<DomainEvent> {
    let mut domain_events = DomainEvents::with(outcome.priority_passed.clone());
    let draws_happen_before_resolution =
        !outcome.card_drawn.is_empty() && outcome.stack_top_resolved.is_some();
    let batches = if draws_happen_before_resolution {
        [
            ResolutionEffectBatch {
                card_drawn: &outcome.card_drawn,
                card_discarded: outcome.card_discarded.as_ref(),
                card_exiled: outcome.card_exiled.as_ref(),
                zone_changes: &outcome.zone_changes,
                life_changed: outcome.life_changed.as_ref(),
                creatures_died: &outcome.creatures_died,
            },
            ResolutionEffectBatch {
                card_drawn: &[],
                card_discarded: None,
                card_exiled: None,
                zone_changes: &[],
                life_changed: None,
                creatures_died: &[],
            },
        ]
    } else {
        [
            ResolutionEffectBatch {
                card_drawn: &[],
                card_discarded: outcome.card_discarded.as_ref(),
                card_exiled: outcome.card_exiled.as_ref(),
                zone_changes: &outcome.zone_changes,
                life_changed: outcome.life_changed.as_ref(),
                creatures_died: &outcome.creatures_died,
            },
            ResolutionEffectBatch {
                card_drawn: &outcome.card_drawn,
                card_discarded: None,
                card_exiled: None,
                zone_changes: &[],
                life_changed: None,
                creatures_died: &[],
            },
        ]
    };
    push_resolution_effect_sequence(&mut domain_events, &batches[..1]);
    push_resolution_close_events(
        &mut domain_events,
        outcome.stack_top_resolved.as_ref(),
        outcome.spell_cast.as_ref(),
    );
    domain_events.extend(outcome.triggered_abilities_put_on_stack.iter().cloned());
    push_resolution_effect_sequence(&mut domain_events, &batches[1..]);
    domain_events.push_optional(outcome.game_ended.clone());
    domain_events.into_vec()
}

pub fn domain_events_for_resolve_optional_effect(
    outcome: &ResolveOptionalEffectOutcome,
) -> Vec<DomainEvent> {
    let mut domain_events = DomainEvents::default();
    push_resolution_effect_sequence(
        &mut domain_events,
        &[ResolutionEffectBatch {
            card_drawn: &[],
            card_discarded: outcome.card_discarded.as_ref(),
            card_exiled: outcome.card_exiled.as_ref(),
            zone_changes: &outcome.zone_changes,
            life_changed: outcome.life_changed.as_ref(),
            creatures_died: &outcome.creatures_died,
        }],
    );
    push_resolution_close_events(
        &mut domain_events,
        outcome.stack_top_resolved.as_ref(),
        outcome.spell_cast.as_ref(),
    );
    push_resolution_follow_up_events(
        &mut domain_events,
        &outcome.triggered_abilities_put_on_stack,
        outcome.game_ended.as_ref(),
    );
    domain_events.into_vec()
}

pub fn domain_events_for_resolve_pending_hand_choice(
    outcome: &ResolvePendingHandChoiceOutcome,
) -> Vec<DomainEvent> {
    let mut domain_events = DomainEvents::default();
    push_resolution_effect_sequence(
        &mut domain_events,
        &[
            ResolutionEffectBatch {
                card_drawn: &[],
                card_discarded: outcome.card_discarded.as_ref(),
                card_exiled: None,
                zone_changes: &outcome.zone_changes,
                life_changed: None,
                creatures_died: &[],
            },
            ResolutionEffectBatch {
                card_drawn: &outcome.card_drawn,
                card_discarded: None,
                card_exiled: None,
                zone_changes: &[],
                life_changed: None,
                creatures_died: &[],
            },
        ],
    );
    push_resolution_close_events(
        &mut domain_events,
        outcome.stack_top_resolved.as_ref(),
        outcome.spell_cast.as_ref(),
    );
    push_resolution_follow_up_events(&mut domain_events, &[], outcome.game_ended.as_ref());
    domain_events.into_vec()
}

pub fn domain_events_for_resolve_pending_scry(
    outcome: &ResolvePendingScryOutcome,
) -> Vec<DomainEvent> {
    let mut domain_events = DomainEvents::default();
    push_resolution_effect_sequence(
        &mut domain_events,
        &[ResolutionEffectBatch {
            card_drawn: &[],
            card_discarded: None,
            card_exiled: None,
            zone_changes: &outcome.zone_changes,
            life_changed: None,
            creatures_died: &[],
        }],
    );
    push_resolution_close_events(
        &mut domain_events,
        outcome.stack_top_resolved.as_ref(),
        outcome.spell_cast.as_ref(),
    );
    push_resolution_follow_up_events(&mut domain_events, &[], outcome.game_ended.as_ref());
    domain_events.into_vec()
}

pub fn domain_events_for_resolve_pending_surveil(
    outcome: &ResolvePendingSurveilOutcome,
) -> Vec<DomainEvent> {
    let mut domain_events = DomainEvents::default();
    push_resolution_effect_sequence(
        &mut domain_events,
        &[ResolutionEffectBatch {
            card_drawn: &[],
            card_discarded: None,
            card_exiled: None,
            zone_changes: &outcome.zone_changes,
            life_changed: None,
            creatures_died: &[],
        }],
    );
    push_resolution_close_events(
        &mut domain_events,
        outcome.stack_top_resolved.as_ref(),
        outcome.spell_cast.as_ref(),
    );
    push_resolution_follow_up_events(&mut domain_events, &[], outcome.game_ended.as_ref());
    domain_events.into_vec()
}

impl<E, B> GameService<E, B>
where
    E: EventStore,
    B: EventBus,
{
    /// Casts a spell.
    ///
    /// # Errors
    ///
    /// Returns an error if the command is invalid.
    pub fn cast_spell(
        &self,
        game: &mut Game,
        cmd: CastSpellCommand,
    ) -> Result<CastSpellOutcome, DomainError> {
        let outcome = game.cast_spell(cmd)?;
        let domain_events = domain_events_for_cast_spell(&outcome);
        self.persist_and_publish_events(game.id().as_str(), &domain_events)?;

        Ok(outcome)
    }

    /// Activates a supported non-mana ability.
    ///
    /// # Errors
    ///
    /// Returns an error if the command is invalid.
    pub fn activate_ability(
        &self,
        game: &mut Game,
        cmd: ActivateAbilityCommand,
    ) -> Result<ActivateAbilityOutcome, DomainError> {
        let outcome = game.activate_ability(cmd)?;
        let domain_events = domain_events_for_activate_ability(&outcome);
        self.persist_and_publish_events(game.id().as_str(), &domain_events)?;

        Ok(outcome)
    }

    /// Passes priority in an open priority window.
    ///
    /// # Errors
    ///
    /// Returns an error if the command is invalid.
    pub fn pass_priority(
        &self,
        game: &mut Game,
        cmd: PassPriorityCommand,
    ) -> Result<PassPriorityOutcome, DomainError> {
        let outcome = game.pass_priority(cmd)?;
        let domain_events = domain_events_for_pass_priority(&outcome);
        self.persist_and_publish_events(game.id().as_str(), &domain_events)?;

        Ok(outcome)
    }

    /// Resolves a pending optional effect choice.
    ///
    /// # Errors
    ///
    /// Returns an error if the command is invalid.
    pub fn resolve_optional_effect(
        &self,
        game: &mut Game,
        cmd: ResolveOptionalEffectCommand,
    ) -> Result<ResolveOptionalEffectOutcome, DomainError> {
        let outcome = game.resolve_optional_effect(cmd)?;
        let domain_events = domain_events_for_resolve_optional_effect(&outcome);
        self.persist_and_publish_events(game.id().as_str(), &domain_events)?;

        Ok(outcome)
    }

    /// Resolves a pending hand-choice effect.
    ///
    /// # Errors
    ///
    /// Returns an error if the command is invalid.
    pub fn resolve_pending_hand_choice(
        &self,
        game: &mut Game,
        cmd: ResolvePendingHandChoiceCommand,
    ) -> Result<ResolvePendingHandChoiceOutcome, DomainError> {
        let outcome = game.resolve_pending_hand_choice(cmd)?;
        let domain_events = domain_events_for_resolve_pending_hand_choice(&outcome);
        self.persist_and_publish_events(game.id().as_str(), &domain_events)?;

        Ok(outcome)
    }

    /// Resolves a pending scry decision.
    ///
    /// # Errors
    ///
    /// Returns an error if the command is invalid.
    pub fn resolve_pending_scry(
        &self,
        game: &mut Game,
        cmd: ResolvePendingScryCommand,
    ) -> Result<ResolvePendingScryOutcome, DomainError> {
        let outcome = game.resolve_pending_scry(cmd)?;
        let domain_events = domain_events_for_resolve_pending_scry(&outcome);
        self.persist_and_publish_events(game.id().as_str(), &domain_events)?;

        Ok(outcome)
    }

    /// Resolves a pending surveil decision.
    ///
    /// # Errors
    ///
    /// Returns an error if the command is invalid.
    pub fn resolve_pending_surveil(
        &self,
        game: &mut Game,
        cmd: ResolvePendingSurveilCommand,
    ) -> Result<ResolvePendingSurveilOutcome, DomainError> {
        let outcome = game.resolve_pending_surveil(cmd)?;
        let domain_events = domain_events_for_resolve_pending_surveil(&outcome);
        self.persist_and_publish_events(game.id().as_str(), &domain_events)?;

        Ok(outcome)
    }
}
