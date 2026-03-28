//! Supports application game service lifecycle.

use {
    super::GameService,
    crate::{
        application::{EventBus, EventStore},
        domain::play::{
            commands::{
                ConcedeCommand, DealOpeningHandsCommand, MulliganCommand, StartGameCommand,
            },
            errors::DomainError,
            events::{GameEnded, GameStarted, MulliganTaken, OpeningHandDealt},
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

    /// Starts a new game and deals opening hands as one persisted lifecycle batch.
    ///
    /// # Errors
    ///
    /// Returns an error if either lifecycle step is invalid.
    pub(crate) fn start_game_with_opening_hands(
        &self,
        start_cmd: StartGameCommand,
        opening_cmd: &DealOpeningHandsCommand,
    ) -> Result<(Game, GameStarted, Vec<OpeningHandDealt>), DomainError> {
        let (mut game, game_started) = Game::start(start_cmd)?;
        let opening_hands = game.deal_opening_hands(opening_cmd)?;

        let mut domain_events = Vec::with_capacity(1 + opening_hands.len());
        domain_events.push(game_started.clone().into());
        domain_events.extend(opening_hands.iter().cloned().map(Into::into));
        self.persist_and_publish_events(game.id().as_str(), &domain_events)?;

        Ok((game, game_started, opening_hands))
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
        self.apply_persisted(
            game,
            |game| game.deal_opening_hands(cmd),
            |events| events.iter().cloned().map(Into::into).collect(),
        )
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
        self.apply_persisted_event(game, |game| game.mulligan(cmd))
    }

    /// Concedes an active game for one player.
    ///
    /// # Errors
    ///
    /// Returns an error if the command is invalid.
    pub fn concede(&self, game: &mut Game, cmd: ConcedeCommand) -> Result<GameEnded, DomainError> {
        self.apply_persisted_event(game, |game| game.concede(cmd))
    }
}
