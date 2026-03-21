use super::super::{helpers, model::Player, TerminalState};
use crate::domain::play::{
    events::{CreatureDied, GameEndReason, GameEnded},
    ids::GameId,
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
    let winning_player = helpers::opposing_player_id(players, &losing_player_id)?;
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

fn destroy_zero_toughness_creatures(game_id: &GameId, players: &mut [Player]) -> Vec<CreatureDied> {
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

fn destroy_lethally_damaged_creatures(
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
/// State-based actions are resolved iteratively until no further changes occur.
///
/// # Errors
/// Returns an error if a derived opposing player cannot be determined while ending the game.
pub fn check_state_based_actions(
    game_id: &GameId,
    players: &mut [Player],
    terminal_state: &mut TerminalState,
) -> Result<StateBasedActionsResult, crate::domain::play::errors::DomainError> {
    let mut total_creatures_died = Vec::new();
    let mut final_game_ended = None;

    loop {
        let mut changes = false;

        let died = destroy_zero_toughness_creatures(game_id, players);
        if !died.is_empty() {
            changes = true;
            total_creatures_died.extend(died);
        }

        let destroyed = destroy_lethally_damaged_creatures(game_id, players);
        if !destroyed.is_empty() {
            changes = true;
            total_creatures_died.extend(destroyed);
        }

        let ended = end_game_for_zero_life(game_id, players, terminal_state)?;
        if let Some(event) = ended {
            changes = true;
            final_game_ended = Some(event);
        }

        if !changes || terminal_state.is_over() {
            break;
        }
    }

    Ok(StateBasedActionsResult::new(
        total_creatures_died,
        final_game_ended,
    ))
}
