//! Supports application game service stack.

use {
    super::{common::DomainEvents, GameService},
    crate::{
        application::{EventBus, EventStore},
        domain::play::{
            commands::{
                ActivateAbilityCommand, CastSpellCommand, PassPriorityCommand,
                ResolveOptionalEffectCommand, ResolvePendingHandChoiceCommand,
                ResolvePendingScryCommand,
            },
            errors::DomainError,
            events::DomainEvent,
            game::{
                ActivateAbilityOutcome, CastSpellOutcome, Game, PassPriorityOutcome,
                ResolveOptionalEffectOutcome, ResolvePendingHandChoiceOutcome,
                ResolvePendingScryOutcome,
            },
        },
    },
};

pub fn domain_events_for_activate_ability(outcome: &ActivateAbilityOutcome) -> Vec<DomainEvent> {
    let mut domain_events = DomainEvents::with(outcome.activated_ability_put_on_stack.clone());
    domain_events.extend(outcome.creatures_died.iter().cloned());
    domain_events.into_vec()
}

pub fn domain_events_for_cast_spell(outcome: &CastSpellOutcome) -> Vec<DomainEvent> {
    vec![outcome.spell_put_on_stack.clone().into()]
}

pub fn domain_events_for_pass_priority(outcome: &PassPriorityOutcome) -> Vec<DomainEvent> {
    let mut domain_events = DomainEvents::with(outcome.priority_passed.clone());
    domain_events.extend(outcome.triggered_abilities_put_on_stack.iter().cloned());
    domain_events.push_optional(outcome.stack_top_resolved.clone());
    domain_events.push_optional(outcome.spell_cast.clone());
    domain_events.push_optional(outcome.card_exiled.clone());
    domain_events.push_optional(outcome.life_changed.clone());
    domain_events.extend(outcome.creatures_died.iter().cloned());
    domain_events.push_optional(outcome.game_ended.clone());
    domain_events.into_vec()
}

pub fn domain_events_for_resolve_optional_effect(
    outcome: &ResolveOptionalEffectOutcome,
) -> Vec<DomainEvent> {
    let mut domain_events = DomainEvents::default();
    domain_events.extend(outcome.triggered_abilities_put_on_stack.iter().cloned());
    domain_events.push_optional(outcome.stack_top_resolved.clone());
    domain_events.push_optional(outcome.spell_cast.clone());
    domain_events.push_optional(outcome.card_exiled.clone());
    domain_events.push_optional(outcome.life_changed.clone());
    domain_events.extend(outcome.creatures_died.iter().cloned());
    domain_events.push_optional(outcome.game_ended.clone());
    domain_events.into_vec()
}

pub fn domain_events_for_resolve_pending_hand_choice(
    outcome: &ResolvePendingHandChoiceOutcome,
) -> Vec<DomainEvent> {
    let mut domain_events = DomainEvents::default();
    domain_events.push_optional(outcome.stack_top_resolved.clone());
    domain_events.push_optional(outcome.spell_cast.clone());
    domain_events.push_optional(outcome.card_discarded.clone());
    domain_events.push_optional(outcome.game_ended.clone());
    domain_events.into_vec()
}

pub fn domain_events_for_resolve_pending_scry(
    outcome: &ResolvePendingScryOutcome,
) -> Vec<DomainEvent> {
    let mut domain_events = DomainEvents::default();
    domain_events.push_optional(outcome.stack_top_resolved.clone());
    domain_events.push_optional(outcome.spell_cast.clone());
    domain_events.push_optional(outcome.game_ended.clone());
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
}
