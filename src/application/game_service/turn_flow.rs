//! Supports application game service turn flow.

use {
    super::{common::DomainEvents, GameService},
    crate::{
        application::{EventBus, EventStore},
        domain::play::{
            commands::{AdvanceTurnCommand, DiscardForCleanupCommand, DrawCardsEffectCommand},
            errors::DomainError,
            events::{CardDiscarded, CardMovedZone, DomainEvent, ZoneType},
            game::{AdvanceTurnOutcome, DrawCardsEffectOutcome, Game},
        },
    },
};

pub fn domain_events_for_advance_turn(outcome: &AdvanceTurnOutcome) -> Vec<DomainEvent> {
    match outcome {
        AdvanceTurnOutcome::Progressed {
            turn_progressed,
            card_drawn,
            triggered_abilities_put_on_stack,
        } => {
            let mut domain_events = DomainEvents::with(turn_progressed.clone());
            domain_events.push_optional(card_drawn.clone());
            domain_events.extend(triggered_abilities_put_on_stack.iter().cloned());
            domain_events.into_vec()
        }
        AdvanceTurnOutcome::GameEnded(game_ended) => vec![game_ended.clone().into()],
    }
}

pub fn domain_events_for_draw_cards_effect(outcome: &DrawCardsEffectOutcome) -> Vec<DomainEvent> {
    let mut domain_events = DomainEvents::default();
    domain_events.extend(outcome.cards_drawn.iter().cloned());
    domain_events.push_optional(outcome.game_ended.clone());
    domain_events.into_vec()
}

pub fn domain_events_for_discard_for_cleanup(event: &CardDiscarded) -> Vec<DomainEvent> {
    let mut domain_events = DomainEvents::with(event.clone());
    domain_events.push(CardMovedZone::new(
        event.game_id.clone(),
        event.player_id.clone(),
        event.card_id.clone(),
        ZoneType::Hand,
        ZoneType::Graveyard,
    ));
    domain_events.into_vec()
}

impl<E, B> GameService<E, B>
where
    E: EventStore,
    B: EventBus,
{
    /// Advances the turn to the next player.
    ///
    /// # Errors
    ///
    /// Returns an error if the active player cannot be found or auto-draw fails.
    pub fn advance_turn(
        &self,
        game: &mut Game,
        cmd: AdvanceTurnCommand,
    ) -> Result<AdvanceTurnOutcome, DomainError> {
        let outcome = game.advance_turn(cmd)?;
        let domain_events = domain_events_for_advance_turn(&outcome);
        self.persist_and_publish_events(game.id().as_str(), &domain_events)?;

        Ok(outcome)
    }

    /// Resolves an explicit draw effect from the active player onto a target player.
    ///
    /// # Errors
    ///
    /// Returns an error if the command is invalid.
    pub fn draw_cards_effect(
        &self,
        game: &mut Game,
        cmd: &DrawCardsEffectCommand,
    ) -> Result<DrawCardsEffectOutcome, DomainError> {
        let outcome = game.draw_cards_effect(cmd)?;
        let domain_events = domain_events_for_draw_cards_effect(&outcome);
        self.persist_and_publish_events(game.id().as_str(), &domain_events)?;

        Ok(outcome)
    }

    /// Discards one card from hand during cleanup-related turn flow.
    ///
    /// # Errors
    ///
    /// Returns an error if the command is invalid.
    pub fn discard_for_cleanup(
        &self,
        game: &mut Game,
        cmd: DiscardForCleanupCommand,
    ) -> Result<CardDiscarded, DomainError> {
        let event = game.discard_for_cleanup(cmd)?;
        let domain_events = domain_events_for_discard_for_cleanup(&event);
        self.persist_and_publish_events(game.id().as_str(), &domain_events)?;

        Ok(event)
    }
}

#[cfg(test)]
mod tests {
    //! Tests application turn-flow event mapping.

    use super::domain_events_for_discard_for_cleanup;
    use crate::domain::play::{
        events::{CardDiscarded, DiscardKind, DomainEvent},
        ids::{CardInstanceId, GameId, PlayerId},
    };

    #[test]
    fn discard_for_cleanup_surfaces_zone_move_after_discard_event() {
        let event = CardDiscarded::new(
            GameId::new("game"),
            PlayerId::new("p1"),
            CardInstanceId::new("card-1"),
            DiscardKind::CleanupHandSize,
        );

        let events = domain_events_for_discard_for_cleanup(&event);

        assert!(matches!(
            events.as_slice(),
            [
                DomainEvent::CardDiscarded(discarded),
                DomainEvent::CardMovedZone(moved),
            ] if discarded.card_id == CardInstanceId::new("card-1")
                && moved.card_id == CardInstanceId::new("card-1")
                && moved.origin_zone.as_str() == "hand"
                && moved.destination_zone.as_str() == "graveyard"
        ));
    }
}
