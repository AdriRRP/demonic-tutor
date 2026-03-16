use super::player::Player;
use crate::domain::{
    cards::CardType,
    commands::TapLandCommand,
    errors::{CardError, DomainError},
    events::{LandTapped, ManaAdded},
};

/// Taps a land to produce mana.
///
/// # Errors
/// Returns an error if:
/// - The player is not found
/// - The card is not on the battlefield
/// - The card is not a land
/// - The land is already tapped
pub fn tap_land(
    players: &mut [Player],
    cmd: TapLandCommand,
) -> Result<(LandTapped, ManaAdded), DomainError> {
    let player_idx = players
        .iter()
        .position(|p| p.id() == &cmd.player_id)
        .ok_or_else(|| {
            DomainError::Game(super::GameError::PlayerNotFound(cmd.player_id.clone()))
        })?;

    let player = &mut players[player_idx];

    let card = player
        .battlefield_mut()
        .card_mut(&cmd.card_id)
        .ok_or_else(|| {
            DomainError::Card(CardError::NotOnBattlefield {
                player: cmd.player_id.clone(),
                card: cmd.card_id.clone(),
            })
        })?;

    if card.is_tapped() {
        return Err(DomainError::Card(CardError::AlreadyTapped {
            player: cmd.player_id.clone(),
            card: cmd.card_id.clone(),
        }));
    }

    if !matches!(card.card_type(), CardType::Land) {
        return Err(DomainError::Card(CardError::NotALand(cmd.card_id.clone())));
    }

    card.tap();

    let old_mana = player.mana();
    let new_mana = old_mana + 1;
    *player.mana_mut() = new_mana;

    let game_id = super::Game::id_from_player_id(&cmd.player_id);

    Ok((
        LandTapped::new(game_id.clone(), cmd.player_id.clone(), cmd.card_id.clone()),
        ManaAdded::new(game_id, cmd.player_id, 1, new_mana),
    ))
}
