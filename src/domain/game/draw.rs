use super::player::Player;
use super::Phase;
use crate::domain::{
    commands::DrawCardCommand,
    errors::{DomainError, GameError},
    events::CardDrawn,
    ids::PlayerId,
};

/// Draws a card from the player's library to their hand.
///
/// # Errors
/// Returns an error if:
/// - The player is not the active player
/// - The phase is not Main or Setup
/// - The player has no cards in their library
pub fn draw_card(
    players: &mut [Player],
    active_player: &PlayerId,
    phase: &Phase,
    cmd: DrawCardCommand,
) -> Result<CardDrawn, DomainError> {
    if *active_player != cmd.player_id {
        return Err(DomainError::Game(GameError::NotYourTurn {
            current: active_player.clone(),
            requested: cmd.player_id,
        }));
    }

    if !matches!(phase, Phase::Main | Phase::Setup) {
        return Err(DomainError::Phase(super::PhaseError::InvalidForDraw {
            phase: *phase,
        }));
    }

    let player_idx = players
        .iter()
        .position(|p| p.id() == &cmd.player_id)
        .ok_or_else(|| DomainError::Game(GameError::PlayerNotFound(cmd.player_id.clone())))?;

    let player = &mut players[player_idx];

    let drawn_cards = player.library_mut().draw(1).ok_or_else(|| {
        DomainError::Game(GameError::NotEnoughCardsInLibrary {
            player: cmd.player_id.clone(),
            available: player.library().len(),
            requested: 1,
        })
    })?;

    let card = drawn_cards.into_iter().next().ok_or_else(|| {
        DomainError::Game(GameError::InternalInvariantViolation(
            "draw(1) should return exactly one card".to_string(),
        ))
    })?;

    let card_id = card.id().clone();
    player.hand_mut().receive(vec![card]);

    Ok(CardDrawn::new(
        super::Game::id_from_player_id(&cmd.player_id),
        cmd.player_id,
        card_id,
    ))
}
