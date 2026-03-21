use super::common::DomainEvents;
use crate::domain::play::{events::DomainEvent, game::ResolveCombatDamageOutcome};

pub(super) fn domain_events_for_resolve_combat_damage(
    outcome: &ResolveCombatDamageOutcome,
) -> Vec<DomainEvent> {
    let mut domain_events = DomainEvents::with(outcome.combat_damage_resolved.clone());
    domain_events.push_optional(outcome.life_changed.clone());
    domain_events.extend(outcome.creatures_died.iter().cloned());
    domain_events.push_optional(outcome.game_ended.clone());
    domain_events.into_vec()
}
