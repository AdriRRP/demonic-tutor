//! Supports game rules state based actions.

use {
    super::super::{
        helpers,
        model::{AggregateCardLocationIndex, Player},
        rules::zones,
        TerminalState,
    },
    crate::domain::play::{
        events::{CardMovedZone, CreatureDied, GameEndReason, GameEnded},
        ids::GameId,
    },
};

#[derive(Debug, Clone)]
struct StateBasedActionCheckResult {
    creatures_died: Vec<CreatureDied>,
    zone_changes: Vec<CardMovedZone>,
    game_ended: Option<GameEnded>,
}

impl StateBasedActionCheckResult {
    #[must_use]
    const fn changed(&self) -> bool {
        !self.creatures_died.is_empty() || !self.zone_changes.is_empty() || self.game_ended.is_some()
    }
}

#[derive(Debug, Clone)]
pub struct StateBasedActionsResult {
    pub creatures_died: Vec<CreatureDied>,
    pub zone_changes: Vec<CardMovedZone>,
    pub game_ended: Option<GameEnded>,
}

impl StateBasedActionsResult {
    #[must_use]
    pub const fn new(
        creatures_died: Vec<CreatureDied>,
        zone_changes: Vec<CardMovedZone>,
        game_ended: Option<GameEnded>,
    ) -> Self {
        Self {
            creatures_died,
            zone_changes,
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

    let zero_life_players = players
        .iter()
        .filter(|player| player.life() == 0)
        .collect::<Vec<_>>();

    let Some(losing_player) = zero_life_players.first() else {
        return Ok(None);
    };

    if zero_life_players.len() > 1 {
        terminal_state.end_draw(GameEndReason::SimultaneousZeroLife);
        return Ok(Some(GameEnded::draw(
            game_id.clone(),
            GameEndReason::SimultaneousZeroLife,
        )));
    }

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
    card_locations: &AggregateCardLocationIndex,
) -> Result<StateBasedActionCheckResult, crate::domain::play::errors::DomainError> {
    let mut creatures_died = Vec::new();
    let mut zone_changes = Vec::new();

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
            let zone_change = zones::move_battlefield_handle_to_owner_graveyard_by_index(
                game_id,
                players,
                card_locations,
                player_index,
                handle,
            )?;
            creatures_died.push(CreatureDied::new(
                game_id.clone(),
                zone_change.zone_owner_id.clone(),
                zone_change.card_id.clone(),
            ));
            zone_changes.push(zone_change);
        }
    }

    Ok(StateBasedActionCheckResult {
        creatures_died,
        zone_changes,
        game_ended: None,
    })
}

fn review_attached_aura_state_based_actions(
    game_id: &GameId,
    players: &mut [Player],
    card_locations: &AggregateCardLocationIndex,
) -> Result<StateBasedActionCheckResult, crate::domain::play::errors::DomainError> {
    let mut zone_changes = Vec::new();

    for player_index in 0..players.len() {
        let doomed_handles = players[player_index]
            .battlefield_handles()
            .filter(|handle| {
                players[player_index]
                    .battlefield_card_by_handle(*handle)
                    .is_some_and(|card| {
                        matches!(
                            card.attachment_profile(),
                            Some(crate::domain::play::cards::AttachmentProfile::EnchantCreature)
                        ) && !card.attached_to().is_some_and(|target_id| {
                            card_locations.location(target_id).is_some_and(|location| {
                                location.zone()
                                    == crate::domain::play::game::PlayerCardZone::Battlefield
                            })
                        })
                    })
            })
            .collect::<Vec<_>>();

        for handle in doomed_handles {
            let zone_change = zones::move_battlefield_handle_to_owner_graveyard_by_index(
                game_id,
                players,
                card_locations,
                player_index,
                handle,
            )?;
            zone_changes.push(zone_change);
        }
    }

    Ok(StateBasedActionCheckResult {
        creatures_died: Vec::new(),
        zone_changes,
        game_ended: None,
    })
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
    let mut total_zone_changes = Vec::new();
    let mut final_game_ended = None;

    loop {
        let mut changes = false;

        let current_locations = AggregateCardLocationIndex::from_players(players);

        let creature_result =
            review_supported_creature_state_based_actions(game_id, players, &current_locations)?;
        if creature_result.changed() {
            changes = true;
        }
        total_creatures_died.extend(creature_result.creatures_died);
        total_zone_changes.extend(creature_result.zone_changes);

        let attached_aura_result =
            review_attached_aura_state_based_actions(game_id, players, &current_locations)?;
        if attached_aura_result.changed() {
            changes = true;
        }
        total_zone_changes.extend(attached_aura_result.zone_changes);

        if terminal_state.is_over() {
            break;
        }

        let zero_life_result = StateBasedActionCheckResult {
            creatures_died: Vec::new(),
            zone_changes: Vec::new(),
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
        total_zone_changes,
        final_game_ended,
    ))
}

#[cfg(test)]
mod tests {
    #![allow(clippy::expect_used, clippy::unwrap_used)]

    use super::*;
    use crate::domain::play::{
        cards::{AttachmentProfile, CardDefinition, CardInstance, CardType},
        events::ZoneType,
        ids::{CardDefinitionId, CardInstanceId, PlayerId},
    };

    #[test]
    fn simultaneous_zero_life_ends_the_game_as_a_draw() {
        let game_id = GameId::new("game-simultaneous-zero-life");
        let mut players = vec![
            Player::new(PlayerId::new("p1")),
            Player::new(PlayerId::new("p2")),
        ];
        let mut terminal_state = TerminalState::active();
        players[0].adjust_life(-20);
        players[1].adjust_life(-20);

        let result = check_state_based_actions(&game_id, &mut players, &mut terminal_state)
            .expect("state-based actions should resolve");
        let game_ended = result
            .game_ended
            .expect("simultaneous zero life should end the game");

        assert_eq!(game_ended.reason, GameEndReason::SimultaneousZeroLife);
        assert_eq!(game_ended.winner_id, None);
        assert_eq!(game_ended.loser_id, None);
        assert_eq!(terminal_state.winner(), None);
        assert_eq!(terminal_state.loser(), None);
        assert_eq!(
            terminal_state.end_reason(),
            Some(GameEndReason::SimultaneousZeroLife)
        );
    }

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
        assert!(players[0].receive_battlefield_card(card).is_some());

        let result =
            check_state_based_actions(&game_id, &mut players, &mut terminal_state).unwrap();

        assert_eq!(result.creatures_died.len(), 1);
        assert_eq!(result.creatures_died[0].player_id, PlayerId::new("p2"));
        assert!(players[0].battlefield_card(&card_id).is_none());
        assert!(players[1].graveyard_card(&card_id).is_some());
    }

    #[test]
    fn stale_external_location_index_does_not_keep_orphaned_aura_on_battlefield() {
        let game_id = GameId::new("game-stale-sba-locations");
        let mut players = vec![
            Player::new(PlayerId::new("p1")),
            Player::new(PlayerId::new("p2")),
        ];
        let mut terminal_state = TerminalState::default();
        let creature_id = CardInstanceId::new("enchanted-bear");
        let aura_id = CardInstanceId::new("holy-strength");

        assert!(players[0]
            .receive_battlefield_card(CardInstance::new_creature(
                creature_id.clone(),
                CardDefinitionId::new("enchanted-bear"),
                0,
                2,
                2,
            ))
            .is_some());

        let aura_definition = CardDefinition::for_card_type(
            CardDefinitionId::new("holy-strength"),
            0,
            &CardType::Enchantment,
        )
        .with_attachment_profile(AttachmentProfile::EnchantCreature);
        let mut aura =
            CardInstance::from_definition(aura_id.clone(), aura_definition, CardType::Enchantment);
        aura.attach_to(creature_id.clone());
        assert!(players[0].receive_battlefield_card(aura).is_some());

        players[0]
            .battlefield_card_mut(&creature_id)
            .expect("creature should be on battlefield")
            .add_damage(2);

        let result = check_state_based_actions(&game_id, &mut players, &mut terminal_state).unwrap();

        assert!(players[0].battlefield_card(&creature_id).is_none());
        assert!(players[0].graveyard_card(&creature_id).is_some());
        assert!(players[0].battlefield_card(&aura_id).is_none());
        assert!(players[0].graveyard_card(&aura_id).is_some());
        assert!(
            result
                .zone_changes
                .iter()
                .any(|event| event.card_id == aura_id
                    && matches!(event.origin_zone, ZoneType::Battlefield)
                    && matches!(event.destination_zone, ZoneType::Graveyard))
        );
    }

    #[test]
    fn token_that_dies_reports_departure_to_created_zone() {
        let game_id = GameId::new("game-token-dies-zone-change");
        let mut players = vec![Player::new(PlayerId::new("p1")), Player::new(PlayerId::new("p2"))];
        let mut terminal_state = TerminalState::default();
        let token_id = CardInstanceId::new("token-1");

        assert!(players[0]
            .receive_battlefield_card(CardInstance::new_vanilla_creature_token(
                token_id.clone(),
                CardDefinitionId::new("token-definition"),
                1,
                1,
            ))
            .is_some());
        players[0]
            .battlefield_card_mut(&token_id)
            .expect("token should be on battlefield")
            .add_damage(1);

        let result =
            check_state_based_actions(&game_id, &mut players, &mut terminal_state).unwrap();

        assert!(
            result.zone_changes.iter().any(|event| event.card_id == token_id
                && matches!(event.origin_zone, ZoneType::Battlefield)
                && matches!(event.destination_zone, ZoneType::Created))
        );
        assert!(players[0].battlefield_card(&token_id).is_none());
        assert!(!players[0].owns_card(&token_id));
    }
}
