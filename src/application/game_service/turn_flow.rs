use super::common::DomainEvents;
use crate::domain::play::{
    events::DomainEvent,
    game::{AdvanceTurnOutcome, DrawCardsEffectOutcome},
};

pub(super) fn domain_events_for_advance_turn(outcome: &AdvanceTurnOutcome) -> Vec<DomainEvent> {
    match outcome {
        AdvanceTurnOutcome::Progressed {
            turn_progressed,
            card_drawn,
        } => {
            let mut domain_events = DomainEvents::with(turn_progressed.clone());
            domain_events.push_optional(card_drawn.clone());
            domain_events.into_vec()
        }
        AdvanceTurnOutcome::GameEnded(game_ended) => vec![game_ended.clone().into()],
    }
}

pub(super) fn domain_events_for_draw_cards_effect(
    outcome: &DrawCardsEffectOutcome,
) -> Vec<DomainEvent> {
    let mut domain_events = DomainEvents::default();
    domain_events.extend(outcome.cards_drawn.iter().cloned());
    domain_events.push_optional(outcome.game_ended.clone());
    domain_events.into_vec()
}
