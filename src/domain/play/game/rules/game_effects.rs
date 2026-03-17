use super::super::{invariants, model::Player, TerminalState};
use crate::domain::play::{
    events::{GameEndReason, GameEnded, LifeChanged},
    ids::{GameId, PlayerId},
};

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
    let player = invariants::find_player_mut(players, player_id)?;
    let old_life = player.life();
    player.adjust_life(life_delta);
    let new_life = player.life();

    Ok(LifeChanged::new(
        game_id.clone(),
        player_id.clone(),
        old_life,
        new_life,
    ))
}

/// Ends the game because a player attempted to draw from an empty library.
///
/// # Errors
/// Returns an error if the losing player has no opposing player in the current game state.
pub fn end_game_for_empty_library_draw(
    game_id: &GameId,
    players: &[Player],
    terminal_state: &mut TerminalState,
    losing_player: &PlayerId,
) -> Result<GameEnded, crate::domain::play::errors::DomainError> {
    let winning_player = invariants::opposing_player_id(players, losing_player)?;
    terminal_state.end(
        winning_player.clone(),
        losing_player.clone(),
        GameEndReason::EmptyLibraryDraw,
    );

    Ok(GameEnded::new(
        game_id.clone(),
        winning_player,
        losing_player.clone(),
        GameEndReason::EmptyLibraryDraw,
    ))
}
