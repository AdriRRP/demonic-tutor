use super::player::Player;
use crate::domain::{
    commands::SetLifeCommand, errors::DomainError, events::LifeChanged, ids::GameId,
};

/// Sets a player's life total.
///
/// # Errors
/// Returns an error if the player is not found.
pub fn set_life(
    game_id: &GameId,
    players: &mut [Player],
    cmd: SetLifeCommand,
) -> Result<LifeChanged, DomainError> {
    let player_idx = players
        .iter()
        .position(|p| p.id() == &cmd.player_id)
        .ok_or_else(|| {
            DomainError::Game(super::GameError::PlayerNotFound(cmd.player_id.clone()))
        })?;

    let player = &mut players[player_idx];

    let old_life = player.life();
    let change = cmd.life_change;
    if change >= 0 {
        player.gain_life(change.unsigned_abs());
    } else {
        player.lose_life(change.unsigned_abs());
    }
    let new_life = player.life();

    Ok(LifeChanged::new(
        game_id.clone(),
        cmd.player_id,
        old_life,
        new_life,
    ))
}
