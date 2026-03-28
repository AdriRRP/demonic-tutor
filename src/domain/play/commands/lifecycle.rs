//! Supports play commands lifecycle.

use crate::domain::play::{
    commands::setup::{PlayerDeck, PlayerLibrary},
    ids::{GameId, PlayerId},
};

#[derive(Debug, Clone)]
pub struct StartGameCommand {
    pub game_id: GameId,
    pub players: Vec<PlayerDeck>,
}

impl StartGameCommand {
    #[must_use]
    pub const fn new(game_id: GameId, players: Vec<PlayerDeck>) -> Self {
        Self { game_id, players }
    }
}

#[derive(Debug, Clone)]
pub struct DealOpeningHandsCommand {
    pub player_libraries: Vec<PlayerLibrary>,
}

impl DealOpeningHandsCommand {
    #[must_use]
    pub const fn new(player_libraries: Vec<PlayerLibrary>) -> Self {
        Self { player_libraries }
    }
}

#[derive(Debug, Clone)]
pub struct MulliganCommand {
    pub player_id: PlayerId,
}

impl MulliganCommand {
    #[must_use]
    pub const fn new(player_id: PlayerId) -> Self {
        Self { player_id }
    }
}

#[derive(Debug, Clone)]
pub struct ConcedeCommand {
    pub player_id: PlayerId,
}

impl ConcedeCommand {
    #[must_use]
    pub const fn new(player_id: PlayerId) -> Self {
        Self { player_id }
    }
}
