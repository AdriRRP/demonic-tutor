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
                CardDiscarded, CardDrawn, CardMovedZone, CreatureDied, DomainEvent, GameEnded,
                LifeChanged, SpellCast, StackTopResolved, TriggeredAbilityPutOnStack,
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
    zone_changes: &'a [CardMovedZone],
    life_changed: Option<&'a LifeChanged>,
    creatures_died: &'a [CreatureDied],
}

struct ResolutionFollowUp<'a> {
    triggered_abilities_put_on_stack: &'a [TriggeredAbilityPutOnStack],
    game_ended: Option<&'a GameEnded>,
}

struct ResolutionEventPlan<'a> {
    effects_before_close: &'a [ResolutionEffectBatch<'a>],
    stack_top_resolved: Option<&'a StackTopResolved>,
    spell_cast: Option<&'a SpellCast>,
    effects_after_close: &'a [ResolutionEffectBatch<'a>],
    follow_up: ResolutionFollowUp<'a>,
}

fn push_resolution_effect_batch(
    domain_events: &mut DomainEvents,
    batch: &ResolutionEffectBatch<'_>,
) {
    domain_events.extend(batch.card_drawn.iter().cloned());
    domain_events.push_optional(batch.card_discarded.cloned());
    domain_events.extend(
        batch
            .zone_changes
            .iter()
            .filter(|event| {
                !matches!(
                    event.origin_zone,
                    crate::domain::play::events::ZoneType::Stack
                ) && !matches!(
                    (&event.origin_zone, &event.destination_zone),
                    (
                        crate::domain::play::events::ZoneType::Library,
                        crate::domain::play::events::ZoneType::Hand
                    )
                )
            })
            .cloned(),
    );
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

fn push_resolution_event_plan(domain_events: &mut DomainEvents, plan: &ResolutionEventPlan<'_>) {
    push_resolution_effect_sequence(domain_events, plan.effects_before_close);
    push_resolution_close_events(domain_events, plan.stack_top_resolved, plan.spell_cast);
    push_resolution_effect_sequence(domain_events, plan.effects_after_close);
    push_resolution_follow_up_events(
        domain_events,
        plan.follow_up.triggered_abilities_put_on_stack,
        plan.follow_up.game_ended,
    );
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
    let effects_with_draws = [ResolutionEffectBatch {
        card_drawn: &outcome.card_drawn,
        card_discarded: outcome.card_discarded.as_ref(),
        zone_changes: &outcome.zone_changes,
        life_changed: outcome.life_changed.as_ref(),
        creatures_died: &outcome.creatures_died,
    }];
    let effects_without_draws = [ResolutionEffectBatch {
        card_drawn: &[],
        card_discarded: outcome.card_discarded.as_ref(),
        zone_changes: &outcome.zone_changes,
        life_changed: outcome.life_changed.as_ref(),
        creatures_died: &outcome.creatures_died,
    }];
    let draws_only = [ResolutionEffectBatch {
        card_drawn: &outcome.card_drawn,
        card_discarded: None,
        zone_changes: &[],
        life_changed: None,
        creatures_died: &[],
    }];
    let empty_batches: [ResolutionEffectBatch<'_>; 0] = [];
    let draws_happen_before_resolution =
        !outcome.card_drawn.is_empty() && outcome.stack_top_resolved.is_some();
    let plan = if draws_happen_before_resolution {
        ResolutionEventPlan {
            effects_before_close: &effects_with_draws,
            stack_top_resolved: outcome.stack_top_resolved.as_ref(),
            spell_cast: outcome.spell_cast.as_ref(),
            effects_after_close: &empty_batches,
            follow_up: ResolutionFollowUp {
                triggered_abilities_put_on_stack: &outcome.triggered_abilities_put_on_stack,
                game_ended: outcome.game_ended.as_ref(),
            },
        }
    } else {
        ResolutionEventPlan {
            effects_before_close: &effects_without_draws,
            stack_top_resolved: outcome.stack_top_resolved.as_ref(),
            spell_cast: outcome.spell_cast.as_ref(),
            effects_after_close: &draws_only,
            follow_up: ResolutionFollowUp {
                triggered_abilities_put_on_stack: &outcome.triggered_abilities_put_on_stack,
                game_ended: outcome.game_ended.as_ref(),
            },
        }
    };
    push_resolution_event_plan(&mut domain_events, &plan);
    domain_events.into_vec()
}

pub fn domain_events_for_resolve_optional_effect(
    outcome: &ResolveOptionalEffectOutcome,
) -> Vec<DomainEvent> {
    let mut domain_events = DomainEvents::default();
    let effects = [ResolutionEffectBatch {
        card_drawn: &[],
        card_discarded: outcome.card_discarded.as_ref(),
        zone_changes: &outcome.zone_changes,
        life_changed: outcome.life_changed.as_ref(),
        creatures_died: &outcome.creatures_died,
    }];
    let empty_batches: [ResolutionEffectBatch<'_>; 0] = [];
    push_resolution_event_plan(
        &mut domain_events,
        &ResolutionEventPlan {
            effects_before_close: &effects,
            stack_top_resolved: outcome.stack_top_resolved.as_ref(),
            spell_cast: outcome.spell_cast.as_ref(),
            effects_after_close: &empty_batches,
            follow_up: ResolutionFollowUp {
                triggered_abilities_put_on_stack: &outcome.triggered_abilities_put_on_stack,
                game_ended: outcome.game_ended.as_ref(),
            },
        },
    );
    domain_events.into_vec()
}

pub fn domain_events_for_resolve_pending_hand_choice(
    outcome: &ResolvePendingHandChoiceOutcome,
) -> Vec<DomainEvent> {
    let mut domain_events = DomainEvents::default();
    let effects_before_close = [
        ResolutionEffectBatch {
            card_drawn: &[],
            card_discarded: outcome.card_discarded.as_ref(),
            zone_changes: &outcome.zone_changes,
            life_changed: None,
            creatures_died: &[],
        },
        ResolutionEffectBatch {
            card_drawn: &outcome.card_drawn,
            card_discarded: None,
            zone_changes: &[],
            life_changed: None,
            creatures_died: &[],
        },
    ];
    let empty_batches: [ResolutionEffectBatch<'_>; 0] = [];
    push_resolution_event_plan(
        &mut domain_events,
        &ResolutionEventPlan {
            effects_before_close: &effects_before_close,
            stack_top_resolved: outcome.stack_top_resolved.as_ref(),
            spell_cast: outcome.spell_cast.as_ref(),
            effects_after_close: &empty_batches,
            follow_up: ResolutionFollowUp {
                triggered_abilities_put_on_stack: &[],
                game_ended: outcome.game_ended.as_ref(),
            },
        },
    );
    domain_events.into_vec()
}

pub fn domain_events_for_resolve_pending_scry(
    outcome: &ResolvePendingScryOutcome,
) -> Vec<DomainEvent> {
    let mut domain_events = DomainEvents::default();
    let effects = [ResolutionEffectBatch {
        card_drawn: &[],
        card_discarded: None,
        zone_changes: &outcome.zone_changes,
        life_changed: None,
        creatures_died: &[],
    }];
    let empty_batches: [ResolutionEffectBatch<'_>; 0] = [];
    push_resolution_event_plan(
        &mut domain_events,
        &ResolutionEventPlan {
            effects_before_close: &effects,
            stack_top_resolved: outcome.stack_top_resolved.as_ref(),
            spell_cast: outcome.spell_cast.as_ref(),
            effects_after_close: &empty_batches,
            follow_up: ResolutionFollowUp {
                triggered_abilities_put_on_stack: &[],
                game_ended: outcome.game_ended.as_ref(),
            },
        },
    );
    domain_events.into_vec()
}

pub fn domain_events_for_resolve_pending_surveil(
    outcome: &ResolvePendingSurveilOutcome,
) -> Vec<DomainEvent> {
    let mut domain_events = DomainEvents::default();
    let effects = [ResolutionEffectBatch {
        card_drawn: &[],
        card_discarded: None,
        zone_changes: &outcome.zone_changes,
        life_changed: None,
        creatures_died: &[],
    }];
    let empty_batches: [ResolutionEffectBatch<'_>; 0] = [];
    push_resolution_event_plan(
        &mut domain_events,
        &ResolutionEventPlan {
            effects_before_close: &effects,
            stack_top_resolved: outcome.stack_top_resolved.as_ref(),
            spell_cast: outcome.spell_cast.as_ref(),
            effects_after_close: &empty_batches,
            follow_up: ResolutionFollowUp {
                triggered_abilities_put_on_stack: &[],
                game_ended: outcome.game_ended.as_ref(),
            },
        },
    );
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
        self.apply_persisted(
            game,
            |game| game.cast_spell(cmd),
            domain_events_for_cast_spell,
        )
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
        self.apply_persisted(
            game,
            |game| game.activate_ability(cmd),
            domain_events_for_activate_ability,
        )
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
        self.apply_persisted(
            game,
            |game| game.pass_priority(cmd),
            domain_events_for_pass_priority,
        )
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
        self.apply_persisted(
            game,
            |game| game.resolve_optional_effect(cmd),
            domain_events_for_resolve_optional_effect,
        )
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
        self.apply_persisted(
            game,
            |game| game.resolve_pending_hand_choice(cmd),
            domain_events_for_resolve_pending_hand_choice,
        )
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
        self.apply_persisted(
            game,
            |game| game.resolve_pending_scry(cmd),
            domain_events_for_resolve_pending_scry,
        )
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
        self.apply_persisted(
            game,
            |game| game.resolve_pending_surveil(cmd),
            domain_events_for_resolve_pending_surveil,
        )
    }
}
