use super::player::Player;
use super::Phase;
use crate::domain::{
    commands::MulliganCommand,
    errors::{DomainError, GameError},
    events::MulliganTaken,
    ids::PlayerId,
};

/// Performs a mulligan, shuffling hand back into library and drawing new hand.
///
/// # Errors
/// Returns an error if:
/// - The phase is not Setup
/// - The player has already used mulligan
/// - The player does not have enough cards in library
pub fn mulligan(
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

    let hand_size = 7;
    if player.library().len() < hand_size {
        return Err(DomainError::Game(GameError::NotEnoughCardsInLibrary {
            player: cmd.player_id,
            available: player.library().len(),
            requested: hand_size,
        }));
    }

    let hand_cards: Vec<_> = player.hand().cards().to_vec();
    player.hand_mut().receive(Vec::new());
    player.library_mut().receive(hand_cards);
    player.library_mut().shuffle();

    let drawn_cards = player.library_mut().draw(hand_size).ok_or_else(|| {
        DomainError::Game(GameError::NotEnoughCardsInLibrary {
            player: cmd.player_id.clone(),
            available: player.library().len(),
            requested: hand_size,
        })
    })?;

    player.hand_mut().receive(drawn_cards);
    *player.mulligan_used_mut() = true;

    Ok(MulliganTaken::new(
        super::Game::id_from_player_id(&cmd.player_id),
        cmd.player_id,
    ))
}
