use super::player::Player;
use crate::domain::{commands::SetLifeCommand, errors::DomainError, events::LifeChanged};

/// Sets a player's life total.
///
/// # Errors
/// Returns an error if the player is not found.
pub fn set_life(players: &mut [Player], cmd: SetLifeCommand) -> Result<LifeChanged, DomainError> {
    let player_idx = players
        .iter()
        .position(|p| p.id() == &cmd.player_id)
        .ok_or_else(|| {
            DomainError::Game(super::GameError::PlayerNotFound(cmd.player_id.clone()))
        })?;

    let player = &mut players[player_idx];

    let old_life = player.life();
    let new_life = old_life
        .saturating_add(cmd.life_change.max(0).cast_unsigned())
        .saturating_sub((-cmd.life_change).max(0).cast_unsigned());
    *player.life_mut() = new_life;

    Ok(LifeChanged::new(
        super::Game::id_from_player_id(&cmd.player_id),
        cmd.player_id,
        old_life,
        new_life,
    ))
}
