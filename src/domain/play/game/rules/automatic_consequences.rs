use super::super::{invariants, model::Player, TerminalState};
use crate::domain::play::{
    events::{CreatureDied, GameEndReason, GameEnded, LifeChanged},
    ids::{GameId, PlayerId},
};

#[derive(Debug, Clone)]
pub struct LifeAdjustmentResult {
    pub life_changed: LifeChanged,
    pub game_ended: Option<GameEnded>,
}

impl LifeAdjustmentResult {
    #[must_use]
    pub const fn new(life_changed: LifeChanged, game_ended: Option<GameEnded>) -> Self {
        Self {
            life_changed,
            game_ended,
        }
    }
}

/// Applies a life delta and resolves the automatic zero-life game loss when needed.
///
/// # Errors
/// Returns an error if the target player is not found or the opposing player cannot be derived.
pub fn adjust_player_life(
    game_id: &GameId,
    players: &mut [Player],
    terminal_state: &mut TerminalState,
    player_id: &PlayerId,
    life_delta: i32,
) -> Result<LifeAdjustmentResult, crate::domain::play::errors::DomainError> {
    let player = invariants::find_player_mut(players, player_id)?;
    let old_life = player.life();
    player.adjust_life(life_delta);
    let new_life = player.life();

    let life_changed = LifeChanged::new(game_id.clone(), player_id.clone(), old_life, new_life);

    let game_ended = if new_life == 0 {
        let winner = invariants::opposing_player_id(players, player_id)?;
        terminal_state.end(winner.clone(), player_id.clone(), GameEndReason::ZeroLife);
        Some(GameEnded::new(
            game_id.clone(),
            winner,
            player_id.clone(),
            GameEndReason::ZeroLife,
        ))
    } else {
        None
    };

    Ok(LifeAdjustmentResult::new(life_changed, game_ended))
}

/// Ends the game because a player attempted to draw from an empty library.
///
/// # Errors
/// Returns an error if the losing player has no opposing player in the current game state.
pub fn end_game_for_empty_library_draw(
    game_id: &GameId,
    players: &[Player],
    terminal_state: &mut TerminalState,
    losing_player: &PlayerId,
) -> Result<GameEnded, crate::domain::play::errors::DomainError> {
    let winning_player = invariants::opposing_player_id(players, losing_player)?;
    terminal_state.end(
        winning_player.clone(),
        losing_player.clone(),
        GameEndReason::EmptyLibraryDraw,
    );

    Ok(GameEnded::new(
        game_id.clone(),
        winning_player,
        losing_player.clone(),
        GameEndReason::EmptyLibraryDraw,
    ))
}

pub fn destroy_zero_toughness_creatures(
    game_id: &GameId,
    players: &mut [Player],
) -> Vec<CreatureDied> {
    let mut died = Vec::new();

    for player in players.iter_mut() {
        let zero_toughness_ids = player
            .battlefield()
            .cards()
            .iter()
            .filter(|card| card.has_zero_toughness())
            .map(|card| card.id().clone())
            .collect::<Vec<_>>();

        for card_id in zero_toughness_ids {
            if let Some(card) = player.battlefield_mut().remove(&card_id) {
                player.graveyard_mut().add(card);
                died.push(CreatureDied::new(
                    game_id.clone(),
                    player.id().clone(),
                    card_id,
                ));
            }
        }
    }

    died
}

pub fn destroy_lethally_damaged_creatures(
    game_id: &GameId,
    players: &mut [Player],
) -> Vec<CreatureDied> {
    let mut destroyed = Vec::new();

    for player in players.iter_mut() {
        let destroyed_ids = player
            .battlefield()
            .cards()
            .iter()
            .filter(|card| card.has_lethal_damage())
            .map(|card| card.id().clone())
            .collect::<Vec<_>>();

        for card_id in destroyed_ids {
            if let Some(card) = player.battlefield_mut().remove(&card_id) {
                player.graveyard_mut().add(card);
                destroyed.push(CreatureDied::new(
                    game_id.clone(),
                    player.id().clone(),
                    card_id,
                ));
            }
        }
    }

    destroyed
}
