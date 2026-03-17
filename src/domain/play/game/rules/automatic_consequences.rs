use super::super::{invariants, model::Player, TerminalState};
use crate::domain::play::{
    events::{CreatureDied, GameEndReason, GameEnded, LifeChanged},
    ids::{GameId, PlayerId},
};

#[derive(Debug, Clone)]
pub struct StateBasedActionsResult {
    pub creatures_died: Vec<CreatureDied>,
    pub game_ended: Option<GameEnded>,
}

impl StateBasedActionsResult {
    #[must_use]
    pub const fn new(creatures_died: Vec<CreatureDied>, game_ended: Option<GameEnded>) -> Self {
        Self {
            creatures_died,
            game_ended,
        }
    }
}

/// Applies a life delta without resolving further automatic gameplay consequences.
///
/// # Errors
/// Returns an error if the target player is not found.
pub fn adjust_player_life(
    game_id: &GameId,
    players: &mut [Player],
    player_id: &PlayerId,
    life_delta: i32,
) -> Result<LifeChanged, crate::domain::play::errors::DomainError> {
    let player = invariants::find_player_mut(players, player_id)?;
    let old_life = player.life();
    player.adjust_life(life_delta);
    let new_life = player.life();

    Ok(LifeChanged::new(
        game_id.clone(),
        player_id.clone(),
        old_life,
        new_life,
    ))
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

fn end_game_for_zero_life(
    game_id: &GameId,
    players: &[Player],
    terminal_state: &mut TerminalState,
) -> Result<Option<GameEnded>, crate::domain::play::errors::DomainError> {
    if terminal_state.is_over() {
        return Ok(None);
    }

    let Some(losing_player) = players.iter().find(|player| player.life() == 0) else {
        return Ok(None);
    };

    let losing_player_id = losing_player.id().clone();
    let winning_player = invariants::opposing_player_id(players, &losing_player_id)?;
    terminal_state.end(
        winning_player.clone(),
        losing_player_id.clone(),
        GameEndReason::ZeroLife,
    );

    Ok(Some(GameEnded::new(
        game_id.clone(),
        winning_player,
        losing_player_id,
        GameEndReason::ZeroLife,
    )))
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

/// Resolves the currently supported state-based actions after a relevant gameplay action.
///
/// The current review covers:
/// - creatures with zero toughness
/// - creatures with lethal marked damage
/// - players at zero life
///
/// # Errors
/// Returns an error if a derived opposing player cannot be determined while ending the game.
pub fn check_state_based_actions(
    game_id: &GameId,
    players: &mut [Player],
    terminal_state: &mut TerminalState,
) -> Result<StateBasedActionsResult, crate::domain::play::errors::DomainError> {
    let mut creatures_died = destroy_zero_toughness_creatures(game_id, players);
    creatures_died.extend(destroy_lethally_damaged_creatures(game_id, players));
    let game_ended = end_game_for_zero_life(game_id, players, terminal_state)?;

    Ok(StateBasedActionsResult::new(creatures_died, game_ended))
}
