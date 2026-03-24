//! Supports game rules game effects.

use {
    super::super::{helpers, model::Player, TerminalState},
    crate::domain::play::{
        events::{GameEndReason, GameEnded, LifeChanged},
        ids::{GameId, PlayerId},
    },
};

/// Applies a life delta without resolving further automatic gameplay consequences.
///
/// # Errors
/// Returns an error if the target player is not found.
pub fn adjust_player_life_by_index(
    game_id: &GameId,
    players: &mut [Player],
    player_index: usize,
    life_delta: i32,
) -> Result<LifeChanged, crate::domain::play::errors::DomainError> {
    let player = players.get_mut(player_index).ok_or_else(|| {
        crate::domain::play::errors::DomainError::Game(
            crate::domain::play::errors::GameError::InternalInvariantViolation(
                "target player index should point to an existing player".to_string(),
            ),
        )
    })?;
    let player_id = player.id().clone();
    let old_life = player.life();
    player.adjust_life(life_delta);
    let new_life = player.life();

    Ok(LifeChanged::new(
        game_id.clone(),
        player_id,
        old_life,
        new_life,
    ))
}

/// Applies a life delta without resolving further automatic gameplay consequences.
///
/// # Errors
/// Returns an error if the target player is not found.
pub fn adjust_player_life(
    game_id: &GameId,
    players: &mut [Player],
    player_id: &PlayerId,
    life_delta: i32,
) -> Result<LifeChanged, crate::domain::play::errors::DomainError> {
    let player_index = helpers::find_player_index(players, player_id)?;
    adjust_player_life_by_index(game_id, players, player_index, life_delta)
}

/// Ends the game because a player attempted to draw from an empty library.
///
/// # Errors
/// Returns an error if the losing player has no opposing player in the current game state.
pub fn end_game_for_empty_library_draw(
    game_id: &GameId,
    players: &[Player],
    terminal_state: &mut TerminalState,
    losing_player_index: usize,
) -> Result<GameEnded, crate::domain::play::errors::DomainError> {
    let losing_player = players.get(losing_player_index).ok_or_else(|| {
        crate::domain::play::errors::DomainError::Game(
            crate::domain::play::errors::GameError::InternalInvariantViolation(
                "losing player index should point to an existing player".to_string(),
            ),
        )
    })?;
    let losing_player_id = losing_player.id().clone();
    let winning_player = players[helpers::opposing_player_index(players, losing_player_index)?]
        .id()
        .clone();
    terminal_state.end(
        winning_player.clone(),
        losing_player_id.clone(),
        GameEndReason::EmptyLibraryDraw,
    );

    Ok(GameEnded::new(
        game_id.clone(),
        winning_player,
        losing_player_id,
        GameEndReason::EmptyLibraryDraw,
    ))
}
