//! Supports game rules state based actions.

use {
    super::super::{helpers, model::Player, rules::zones, TerminalState},
    crate::domain::play::{
        events::{CreatureDied, GameEndReason, GameEnded},
        ids::GameId,
    },
};

#[derive(Debug, Clone)]
struct StateBasedActionCheckResult {
    creatures_died: Vec<CreatureDied>,
    game_ended: Option<GameEnded>,
}

impl StateBasedActionCheckResult {
    #[must_use]
    const fn changed(&self) -> bool {
        !self.creatures_died.is_empty() || self.game_ended.is_some()
    }
}

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

fn review_supported_creature_state_based_actions(
    game_id: &GameId,
    players: &mut [Player],
) -> StateBasedActionCheckResult {
    let mut creatures_died = Vec::new();

    for player_index in 0..players.len() {
        let doomed_handles = players[player_index]
            .battlefield_handles()
            .filter(|handle| {
                players[player_index]
                    .battlefield_card_by_handle(*handle)
                    .is_some_and(|card| {
                        card.has_zero_toughness()
                            || (card.has_lethal_damage() && !card.has_indestructible())
                    })
            })
            .collect::<Vec<_>>();

        for handle in doomed_handles {
            if let Ok((owner_id, card_id)) =
                zones::move_battlefield_handle_to_owner_graveyard_by_index(
                    players,
                    player_index,
                    handle,
                )
            {
                creatures_died.push(CreatureDied::new(game_id.clone(), owner_id, card_id));
            }
        }
    }

    StateBasedActionCheckResult {
        creatures_died,
        game_ended: None,
    }
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

        let creature_result = review_supported_creature_state_based_actions(game_id, players);
        if creature_result.changed() {
            changes = true;
        }
        total_creatures_died.extend(creature_result.creatures_died);

        if terminal_state.is_over() {
            break;
        }

        let zero_life_result = StateBasedActionCheckResult {
            creatures_died: Vec::new(),
            game_ended: end_game_for_zero_life(game_id, players, terminal_state)?,
        };
        if zero_life_result.changed() {
            changes = true;
        }
        if let Some(event) = zero_life_result.game_ended {
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

#[cfg(test)]
mod tests {
    #![allow(clippy::expect_used, clippy::unwrap_used)]

    use super::*;
    use crate::domain::play::{
        cards::CardInstance,
        ids::{CardDefinitionId, CardInstanceId, PlayerId},
    };

    #[test]
    fn lethal_damage_moves_foreign_owned_creature_to_owners_graveyard() {
        let game_id = GameId::new("game-owner-sba");
        let mut players = vec![
            Player::new(PlayerId::new("p1")),
            Player::new(PlayerId::new("p2")),
        ];
        let mut terminal_state = TerminalState::default();
        let card_id = CardInstanceId::new("borrowed-bear");

        players[1].receive_graveyard_card(CardInstance::new_creature(
            card_id.clone(),
            CardDefinitionId::new("borrowed-bear"),
            0,
            2,
            2,
        ));
        let mut card = players[1]
            .remove_graveyard_card(&card_id)
            .expect("owner graveyard should contain the card");
        card.add_damage(2);
        players[0].receive_battlefield_card(card);

        let result =
            check_state_based_actions(&game_id, &mut players, &mut terminal_state).unwrap();

        assert_eq!(result.creatures_died.len(), 1);
        assert_eq!(result.creatures_died[0].player_id, PlayerId::new("p2"));
        assert!(players[0].battlefield_card(&card_id).is_none());
        assert!(players[1].graveyard_card(&card_id).is_some());
    }
}
