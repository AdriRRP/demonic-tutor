use super::player::{Player, OPENING_HAND_SIZE};
use super::Phase;
use crate::domain::{
    commands::MulliganCommand,
    errors::{DomainError, GameError},
    events::MulliganTaken,
    ids::{GameId, PlayerId},
};

/// Performs a mulligan, shuffling hand back into library and drawing new hand.
///
/// # Errors
/// Returns an error if:
/// - The phase is not Setup
/// - The player has already used mulligan
/// - The player does not have enough cards in library
pub fn mulligan(
    game_id: &GameId,
    players: &mut [Player],
    _active_player: &PlayerId,
    phase: &Phase,
    cmd: MulliganCommand,
) -> Result<MulliganTaken, DomainError> {
    if !matches!(phase, Phase::Setup) {
        return Err(DomainError::Phase(super::PhaseError::InvalidForMulligan));
    }

    let player_idx = players
        .iter()
        .position(|p| p.id() == &cmd.player_id)
        .ok_or_else(|| DomainError::Game(GameError::PlayerNotFound(cmd.player_id.clone())))?;

    let player = &mut players[player_idx];

    if player.mulligan_used() {
        return Err(DomainError::Game(GameError::MulliganAlreadyUsed(
            cmd.player_id,
        )));
    }

    if player.library().len() < OPENING_HAND_SIZE {
        return Err(DomainError::Game(GameError::NotEnoughCardsInLibrary {
            player: cmd.player_id,
            available: player.library().len(),
            requested: OPENING_HAND_SIZE,
        }));
    }

    let hand_cards = player.hand_mut().drain_all();
    player.library_mut().receive(hand_cards);
    player.library_mut().shuffle();

    let drawn_cards = player
        .library_mut()
        .draw(OPENING_HAND_SIZE)
        .ok_or_else(|| {
            DomainError::Game(GameError::NotEnoughCardsInLibrary {
                player: cmd.player_id.clone(),
                available: player.library().len(),
                requested: OPENING_HAND_SIZE,
            })
        })?;

    player.hand_mut().receive(drawn_cards);
    player.use_mulligan();

    Ok(MulliganTaken::new(game_id.clone(), cmd.player_id))
}
