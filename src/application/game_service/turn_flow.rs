//! Supports application game service turn flow.

use {
    super::{common::DomainEvents, GameService},
    crate::{
        application::{EventBus, EventStore},
        domain::play::{
            commands::{AdvanceTurnCommand, DiscardForCleanupCommand, DrawCardsEffectCommand},
            errors::DomainError,
            events::{CardDiscarded, DomainEvent},
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
        self.persist_and_publish_event(game.id().as_str(), &event)?;

        Ok(event)
    }
}
