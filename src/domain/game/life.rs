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
    let new_life = if change >= 0 {
        old_life.saturating_add(change.unsigned_abs())
    } else {
        old_life.saturating_sub(change.unsigned_abs())
    };
    *player.life_mut() = new_life;

    Ok(LifeChanged::new(
        game_id.clone(),
        cmd.player_id,
        old_life,
        new_life,
    ))
}
