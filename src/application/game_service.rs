use crate::domain::{
    commands::{
        AdvanceTurnCommand, DealOpeningHandsCommand, DrawCardCommand, PlayLandCommand,
        StartGameCommand,
    },
    errors::DomainError,
    events::{CardDrawn, LandPlayed, OpeningHandDealt, TurnAdvanced},
    game::Game,
};

pub struct GameService;

impl GameService {
    /// Starts a new game.
    ///
    /// # Errors
    ///
    /// Returns an error if the command is invalid.
    pub fn start_game(
        cmd: StartGameCommand,
    ) -> Result<(Game, crate::domain::events::GameStarted), DomainError> {
        Game::start(cmd)
    }

    /// Deals opening hands to all players.
    ///
    /// # Errors
    ///
    /// Returns an error if the command is invalid.
    pub fn deal_opening_hands(
        game: &mut Game,
        cmd: &DealOpeningHandsCommand,
    ) -> Result<Vec<OpeningHandDealt>, DomainError> {
        game.deal_opening_hands(cmd)
    }

    /// Plays a land card.
    ///
    /// # Errors
    ///
    /// Returns an error if the command is invalid.
    pub fn play_land(game: &mut Game, cmd: PlayLandCommand) -> Result<LandPlayed, DomainError> {
        game.play_land(cmd)
    }

    /// Advances the turn to the next player.
    ///
    /// # Errors
    ///
    /// Returns an error if the active player cannot be found.
    pub fn advance_turn(
        game: &mut Game,
        cmd: AdvanceTurnCommand,
    ) -> Result<TurnAdvanced, DomainError> {
        game.advance_turn(cmd)
    }

    /// Draws a card from the player's library.
    ///
    /// # Errors
    ///
    /// Returns an error if the command is invalid.
    pub fn draw_card(game: &mut Game, cmd: DrawCardCommand) -> Result<CardDrawn, DomainError> {
        game.draw_card(cmd)
    }
}
