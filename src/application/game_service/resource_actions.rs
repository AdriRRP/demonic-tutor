use super::common::DomainEvents;
use crate::domain::play::{events::DomainEvent, game::AdjustPlayerLifeEffectOutcome};

pub(super) fn domain_events_for_adjust_player_life_effect(
    outcome: &AdjustPlayerLifeEffectOutcome,
) -> Vec<DomainEvent> {
    let mut domain_events = DomainEvents::with(outcome.life_changed.clone());
    domain_events.extend(outcome.creatures_died.iter().cloned());
    domain_events.push_optional(outcome.game_ended.clone());
    domain_events.into_vec()
}
