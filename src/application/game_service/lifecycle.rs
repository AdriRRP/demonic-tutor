//! Supports application game service lifecycle.

use {
    super::GameService,
    crate::{
        application::{EventBus, EventStore},
        domain::play::{
            commands::{DealOpeningHandsCommand, MulliganCommand, StartGameCommand},
            errors::DomainError,
            events::{GameStarted, MulliganTaken, OpeningHandDealt},
            game::Game,
        },
    },
};

impl<E, B> GameService<E, B>
where
    E: EventStore,
    B: EventBus,
{
    /// Starts a new game.
    ///
    /// # Errors
    ///
    /// Returns an error if the command is invalid.
    pub fn start_game(&self, cmd: StartGameCommand) -> Result<(Game, GameStarted), DomainError> {
        let (game, event) = Game::start(cmd)?;
        self.persist_and_publish_event(game.id().as_str(), &event)?;

        Ok((game, event))
    }

    /// Deals opening hands to all players.
    ///
    /// # Errors
    ///
    /// Returns an error if the command is invalid.
    pub fn deal_opening_hands(
        &self,
        game: &mut Game,
        cmd: &DealOpeningHandsCommand,
    ) -> Result<Vec<OpeningHandDealt>, DomainError> {
        let events = game.deal_opening_hands(cmd)?;
        self.persist_and_publish_event_batch(game.id().as_str(), &events)?;

        Ok(events)
    }

    /// Performs a mulligan for a player.
    ///
    /// # Errors
    ///
    /// Returns an error if the command is invalid.
    pub fn mulligan(
        &self,
        game: &mut Game,
        cmd: MulliganCommand,
    ) -> Result<MulliganTaken, DomainError> {
        let event = game.mulligan(cmd)?;
        self.persist_and_publish_event(game.id().as_str(), &event)?;

        Ok(event)
    }
}
