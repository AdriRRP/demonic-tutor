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
            events::{BlockersDeclared, CardMovedZone, DomainEvent, ZoneType},
            game::{DeclareAttackersOutcome, Game, ResolveCombatDamageOutcome},
        },
    },
};

fn zone_change_for_creature_died(
    event: &crate::domain::play::events::CreatureDied,
) -> CardMovedZone {
    CardMovedZone::new(
        event.game_id.clone(),
        event.player_id.clone(),
        event.card_id.clone(),
        ZoneType::Battlefield,
        ZoneType::Graveyard,
    )
}

pub fn domain_events_for_declare_attackers(outcome: &DeclareAttackersOutcome) -> Vec<DomainEvent> {
    let mut domain_events = DomainEvents::with(outcome.attackers_declared.clone());
    domain_events.extend(outcome.triggered_abilities_put_on_stack.iter().cloned());
    domain_events.into_vec()
}

pub fn domain_events_for_resolve_combat_damage(
    outcome: &ResolveCombatDamageOutcome,
) -> Vec<DomainEvent> {
    let mut domain_events = DomainEvents::default();
    domain_events.extend(outcome.life_changed.iter().cloned());
    domain_events.extend(outcome.creatures_died.iter().cloned());
    domain_events.extend(
        outcome
            .creatures_died
            .iter()
            .map(zone_change_for_creature_died),
    );
    domain_events.push(outcome.combat_damage_resolved.clone());
    domain_events.extend(outcome.triggered_abilities_put_on_stack.iter().cloned());
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
    ) -> Result<DeclareAttackersOutcome, DomainError> {
        let outcome = game.declare_attackers(cmd)?;
        let domain_events = domain_events_for_declare_attackers(&outcome);
        self.persist_and_publish_events(game.id().as_str(), &domain_events)?;
        Ok(outcome)
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
