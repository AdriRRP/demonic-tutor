//! Supports application game service combat.

use {
    super::{common::DomainEvents, GameService},
    crate::{
        application::{EventBus, EventStore},
        domain::play::{
            commands::{
                DeclareAttackersCommand, DeclareBlockersCommand, ResolveCombatDamageCommand,
            },
            errors::DomainError,
            events::{AttackersDeclared, BlockersDeclared, DomainEvent},
            game::{Game, ResolveCombatDamageOutcome},
        },
    },
};

pub(super) fn domain_events_for_resolve_combat_damage(
    outcome: &ResolveCombatDamageOutcome,
) -> Vec<DomainEvent> {
    let mut domain_events = DomainEvents::with(outcome.combat_damage_resolved.clone());
    domain_events.extend(outcome.life_changed.iter().cloned());
    domain_events.extend(outcome.creatures_died.iter().cloned());
    domain_events.push_optional(outcome.game_ended.clone());
    domain_events.into_vec()
}

impl<E, B> GameService<E, B>
where
    E: EventStore,
    B: EventBus,
{
    /// Declares attacking creatures.
    ///
    /// # Errors
    ///
    /// Returns an error if the command is invalid.
    pub fn declare_attackers(
        &self,
        game: &mut Game,
        cmd: DeclareAttackersCommand,
    ) -> Result<AttackersDeclared, DomainError> {
        let event = game.declare_attackers(cmd)?;
        self.persist_and_publish_event(game.id().as_str(), &event)?;

        Ok(event)
    }

    /// Declares blocking creatures.
    ///
    /// # Errors
    ///
    /// Returns an error if the command is invalid.
    pub fn declare_blockers(
        &self,
        game: &mut Game,
        cmd: DeclareBlockersCommand,
    ) -> Result<BlockersDeclared, DomainError> {
        let event = game.declare_blockers(cmd)?;
        self.persist_and_publish_event(game.id().as_str(), &event)?;

        Ok(event)
    }

    /// Resolves combat damage.
    ///
    /// # Errors
    ///
    /// Returns an error if the command is invalid.
    pub fn resolve_combat_damage(
        &self,
        game: &mut Game,
        cmd: ResolveCombatDamageCommand,
    ) -> Result<ResolveCombatDamageOutcome, DomainError> {
        let outcome = game.resolve_combat_damage(cmd)?;
        let domain_events = domain_events_for_resolve_combat_damage(&outcome);
        self.persist_and_publish_events(game.id().as_str(), &domain_events)?;

        Ok(outcome)
    }
}
