//! Supports play events lifecycle.

use crate::domain::play::ids::{CardInstanceId, GameId, PlayerId};

#[derive(Debug, Clone)]
pub struct GameStarted {
    pub game_id: GameId,
    pub players: Vec<PlayerId>,
}

impl GameStarted {
    #[must_use]
    pub const fn new(game_id: GameId, players: Vec<PlayerId>) -> Self {
        Self { game_id, players }
    }
}

#[derive(Debug, Clone)]
pub struct OpeningHandDealt {
    pub game_id: GameId,
    pub player_id: PlayerId,
    pub cards: Vec<CardInstanceId>,
}

impl OpeningHandDealt {
    #[must_use]
    pub const fn new(game_id: GameId, player_id: PlayerId, cards: Vec<CardInstanceId>) -> Self {
        Self {
            game_id,
            player_id,
            cards,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameEndReason {
    EmptyLibraryDraw,
    ZeroLife,
    SimultaneousZeroLife,
    Conceded,
}

#[derive(Debug, Clone)]
pub struct GameEnded {
    pub game_id: GameId,
    pub winner_id: Option<PlayerId>,
    pub loser_id: Option<PlayerId>,
    pub reason: GameEndReason,
}

impl GameEnded {
    #[must_use]
    pub const fn new(
        game_id: GameId,
        winner_id: PlayerId,
        loser_id: PlayerId,
        reason: GameEndReason,
    ) -> Self {
        Self {
            game_id,
            winner_id: Some(winner_id),
            loser_id: Some(loser_id),
            reason,
        }
    }

    #[must_use]
    pub const fn draw(game_id: GameId, reason: GameEndReason) -> Self {
        Self {
            game_id,
            winner_id: None,
            loser_id: None,
            reason,
        }
    }
}
