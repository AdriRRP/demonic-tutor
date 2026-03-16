use super::player::Player;
use super::Phase;
use crate::domain::{
    cards::CardType,
    commands::PlayLandCommand,
    errors::{CardError, DomainError, PhaseError},
    events::LandPlayed,
    ids::PlayerId,
};

/// Plays a land card from hand to battlefield.
///
/// # Errors
/// Returns an error if:
/// - The player is not the active player
/// - The phase is not Main
/// - The player has already played a land this turn
/// - The card is not in the player's hand
/// - The card is not a land
pub fn play_land(
    players: &mut [Player],
    active_player: &PlayerId,
    phase: &Phase,
    cmd: PlayLandCommand,
) -> Result<LandPlayed, DomainError> {
    if *active_player != cmd.player_id {
        return Err(DomainError::Game(super::GameError::NotYourTurn {
            current: active_player.clone(),
            requested: cmd.player_id,
        }));
    }

    if !matches!(phase, Phase::Main) {
        return Err(DomainError::Phase(PhaseError::InvalidForLand));
    }

    let player_idx = players
        .iter()
        .position(|p| p.id() == &cmd.player_id)
        .ok_or_else(|| {
            DomainError::Game(super::GameError::PlayerNotFound(cmd.player_id.clone()))
        })?;

    let player = &mut players[player_idx];

    if player.lands_played_this_turn() > 0 {
        return Err(DomainError::Phase(PhaseError::AlreadyPlayedLandThisTurn(
            cmd.player_id,
        )));
    }

    let card_id = cmd.card_id.clone();

    let card = player.hand_mut().remove(&card_id).ok_or_else(|| {
        DomainError::Card(CardError::NotInHand {
            player: cmd.player_id.clone(),
            card: card_id.clone(),
        })
    })?;

    if !matches!(card.card_type(), CardType::Land) {
        return Err(DomainError::Card(CardError::NotALand(card_id)));
    }

    player.battlefield_mut().add(card);
    *player.lands_played_this_turn_mut() += 1;

    Ok(LandPlayed::new(
        super::Game::id_from_player_id(&cmd.player_id),
        cmd.player_id,
        card_id,
    ))
}
