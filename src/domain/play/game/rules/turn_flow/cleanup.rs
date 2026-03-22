use crate::domain::play::game::{
    helpers, invariants,
    model::{Player, MAX_HAND_SIZE},
};
use crate::domain::play::{
    commands::DiscardForCleanupCommand,
    errors::{DomainError, GameError, PhaseError},
    events::{CardDiscarded, DiscardKind},
    ids::{GameId, PlayerId},
    phase::Phase,
};

/// Discards one card from hand to graveyard as an explicit cleanup action.
///
/// # Errors
/// Returns an error if:
/// - The player is not the active player
/// - The phase is not `EndStep`
/// - The player is not above the maximum hand size
/// - The card is not in the player's hand
pub fn discard_for_cleanup(
    game_id: &GameId,
    players: &mut [Player],
    active_player: &PlayerId,
    phase: &Phase,
    cmd: DiscardForCleanupCommand,
) -> Result<CardDiscarded, DomainError> {
    invariants::require_active_player(active_player, &cmd.player_id)?;

    if !matches!(phase, Phase::EndStep) {
        return Err(DomainError::Phase(PhaseError::InvalidForDiscard {
            phase: *phase,
        }));
    }

    let player = helpers::find_player_mut(players, &cmd.player_id)?;
    let hand_size = player.hand_size();
    if hand_size <= MAX_HAND_SIZE {
        return Err(DomainError::Game(GameError::DiscardNotRequired {
            player: cmd.player_id.clone(),
            hand_size,
            max_hand_size: MAX_HAND_SIZE,
        }));
    }

    let card = helpers::remove_card_from_hand(player, &cmd.player_id, &cmd.card_id)?;
    let card_id = card.id().clone();
    player.receive_graveyard_card(card);

    Ok(CardDiscarded::new(
        game_id.clone(),
        cmd.player_id,
        card_id,
        DiscardKind::CleanupHandSize,
    ))
}
