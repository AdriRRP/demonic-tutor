use super::player::Player;
use super::Phase;
use crate::domain::{
    commands::PlayLandCommand,
    errors::{CardError, DomainError, PhaseError},
    events::LandPlayed,
    ids::{GameId, PlayerId},
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
    game_id: &GameId,
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

    if !matches!(phase, Phase::FirstMain | Phase::SecondMain) {
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

    // Validate card type before removing from hand to avoid losing the card
    // if the type check fails.
    let card_type = player
        .hand()
        .cards()
        .iter()
        .find(|c| c.id() == &card_id)
        .map(|c| c.card_type().clone())
        .ok_or_else(|| {
            DomainError::Card(CardError::NotInHand {
                player: cmd.player_id.clone(),
                card: card_id.clone(),
            })
        })?;

    if !card_type.is_land() {
        return Err(DomainError::Card(CardError::NotALand(card_id)));
    }

    let card = player.hand_mut().remove(&card_id).ok_or_else(|| {
        DomainError::Card(CardError::NotInHand {
            player: cmd.player_id.clone(),
            card: card_id.clone(),
        })
    })?;

    player.battlefield_mut().add(card);
    *player.lands_played_this_turn_mut() += 1;

    Ok(LandPlayed::new(game_id.clone(), cmd.player_id, card_id))
}
