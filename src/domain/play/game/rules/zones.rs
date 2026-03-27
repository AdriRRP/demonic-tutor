//! Supports game rules zones.

use {
    super::super::model::Player,
    crate::domain::play::{
        errors::{CardError, DomainError, GameError},
        events::{CardExiled, ZoneType},
        ids::{CardInstanceId, GameId, PlayerCardHandle, PlayerId},
    },
};

fn owner_index_for_battlefield_handle(
    players: &[Player],
    controller_index: usize,
    handle: PlayerCardHandle,
) -> Result<usize, DomainError> {
    let controller = players.get(controller_index).ok_or_else(|| {
        DomainError::Game(GameError::InternalInvariantViolation(format!(
            "player index {controller_index} must exist during battlefield ownership lookup"
        )))
    })?;
    let card = controller.card_by_handle(handle).ok_or_else(|| {
        DomainError::Game(GameError::InternalInvariantViolation(format!(
            "missing card handle {} during battlefield ownership lookup",
            handle.index()
        )))
    })?;
    let Some(owner_id) = card.owner_id() else {
        return Ok(controller_index);
    };

    players
        .iter()
        .position(|player| player.id() == owner_id)
        .ok_or_else(|| DomainError::Game(GameError::PlayerNotFound(owner_id.clone())))
}

fn remove_attached_aura_effects_for_battlefield_handle(
    players: &mut [Player],
    controller_index: usize,
    handle: PlayerCardHandle,
) {
    let Some(card) = players[controller_index].card_by_handle(handle) else {
        return;
    };
    let Some(attached_to) = card.attached_to().cloned() else {
        return;
    };

    let attached_stat_boost = card.attached_stat_boost();
    let attached_combat_restriction = card.attached_combat_restriction();

    if attached_stat_boost.is_none() && attached_combat_restriction.is_none() {
        return;
    }

    let Some(target) = players
        .iter_mut()
        .find_map(|player| player.battlefield_card_mut(&attached_to))
    else {
        return;
    };

    if let Some(attached_stat_boost) = attached_stat_boost {
        target.remove_attached_stat_bonus(
            attached_stat_boost.power(),
            attached_stat_boost.toughness(),
        );
    }
    if attached_combat_restriction.is_some() {
        target.remove_attached_cant_attack_or_block();
    }
}

pub(crate) fn move_battlefield_handle_to_owner_graveyard_by_index(
    players: &mut [Player],
    controller_index: usize,
    handle: PlayerCardHandle,
) -> Result<(PlayerId, CardInstanceId), DomainError> {
    let owner_index = owner_index_for_battlefield_handle(players, controller_index, handle)?;
    remove_attached_aura_effects_for_battlefield_handle(players, controller_index, handle);
    let card_id = players[controller_index]
        .card_by_handle(handle)
        .map(|card| card.id().clone())
        .ok_or_else(|| {
            DomainError::Game(GameError::InternalInvariantViolation(format!(
                "missing card handle {} during battlefield-to-graveyard move",
                handle.index()
            )))
        })?;

    if owner_index == controller_index {
        let owner = players.get_mut(owner_index).ok_or_else(|| {
            DomainError::Game(GameError::InternalInvariantViolation(format!(
                "player index {owner_index} must exist during battlefield-to-graveyard move"
            )))
        })?;
        owner
            .move_battlefield_handle_to_graveyard(handle)
            .ok_or_else(|| {
                DomainError::Card(CardError::NotOnBattlefield {
                    player: owner.id().clone(),
                    card: card_id.clone(),
                })
            })?;
        return Ok((owner.id().clone(), card_id));
    }

    let card = players[controller_index]
        .take_battlefield_handle(handle)
        .ok_or_else(|| {
            DomainError::Card(CardError::NotOnBattlefield {
                player: players[controller_index].id().clone(),
                card: card_id.clone(),
            })
        })?;
    let owner = players.get_mut(owner_index).ok_or_else(|| {
        DomainError::Game(GameError::InternalInvariantViolation(format!(
            "player index {owner_index} must exist during battlefield-to-graveyard transfer"
        )))
    })?;
    let owner_id = owner.id().clone();
    owner.receive_graveyard_card(card);
    Ok((owner_id, card_id))
}

pub(crate) fn move_battlefield_handle_to_owner_hand_by_index(
    players: &mut [Player],
    controller_index: usize,
    handle: PlayerCardHandle,
) -> Result<(PlayerId, CardInstanceId), DomainError> {
    let owner_index = owner_index_for_battlefield_handle(players, controller_index, handle)?;
    remove_attached_aura_effects_for_battlefield_handle(players, controller_index, handle);
    let card_id = players[controller_index]
        .card_by_handle(handle)
        .map(|card| card.id().clone())
        .ok_or_else(|| {
            DomainError::Game(GameError::InternalInvariantViolation(format!(
                "missing card handle {} during battlefield-to-hand move",
                handle.index()
            )))
        })?;

    if owner_index == controller_index {
        let owner = players.get_mut(owner_index).ok_or_else(|| {
            DomainError::Game(GameError::InternalInvariantViolation(format!(
                "player index {owner_index} must exist during battlefield-to-hand move"
            )))
        })?;
        owner
            .move_battlefield_handle_to_hand(handle)
            .ok_or_else(|| {
                DomainError::Card(CardError::NotOnBattlefield {
                    player: owner.id().clone(),
                    card: card_id.clone(),
                })
            })?;
        return Ok((owner.id().clone(), card_id));
    }

    let card = players[controller_index]
        .take_battlefield_handle(handle)
        .ok_or_else(|| {
            DomainError::Card(CardError::NotOnBattlefield {
                player: players[controller_index].id().clone(),
                card: card_id.clone(),
            })
        })?;
    let owner = players.get_mut(owner_index).ok_or_else(|| {
        DomainError::Game(GameError::InternalInvariantViolation(format!(
            "player index {owner_index} must exist during battlefield-to-hand transfer"
        )))
    })?;
    let owner_id = owner.id().clone();
    owner.receive_hand_cards(vec![card]);
    Ok((owner_id, card_id))
}

fn exile_card_from_player_zone_handle_by_index(
    game_id: &GameId,
    players: &mut [Player],
    player_index: usize,
    handle: PlayerCardHandle,
    source_zone: ZoneType,
    move_card: impl FnOnce(&mut Player, PlayerCardHandle) -> Option<()>,
) -> Result<CardExiled, DomainError> {
    let player = players.get_mut(player_index).ok_or_else(|| {
        DomainError::Game(GameError::InternalInvariantViolation(format!(
            "player index {player_index} must exist during zone transition"
        )))
    })?;
    let player_id = player.id().clone();
    let card_id = player
        .card_by_handle(handle)
        .map(|card| card.id().clone())
        .ok_or_else(|| {
            DomainError::Game(GameError::InternalInvariantViolation(format!(
                "missing card handle {} during zone transition",
                handle.index()
            )))
        })?;

    move_card(player, handle).ok_or_else(|| {
        DomainError::Card(CardError::NotOnBattlefield {
            player: player_id.clone(),
            card: card_id.clone(),
        })
    })?;

    Ok(CardExiled::new(
        game_id.clone(),
        player_id,
        card_id,
        source_zone,
    ))
}

fn exile_card_from_player_zone_by_index(
    game_id: &GameId,
    players: &mut [Player],
    player_index: usize,
    card_id: &CardInstanceId,
    source_zone: ZoneType,
    move_card: impl FnOnce(&mut Player, &CardInstanceId) -> Option<()>,
) -> Result<CardExiled, DomainError> {
    let player = players.get_mut(player_index).ok_or_else(|| {
        DomainError::Game(GameError::InternalInvariantViolation(format!(
            "player index {player_index} must exist during zone transition"
        )))
    })?;
    let player_id = player.id().clone();

    move_card(player, card_id).ok_or_else(|| {
        DomainError::Card(CardError::NotOnBattlefield {
            player: player_id.clone(),
            card: card_id.clone(),
        })
    })?;

    Ok(CardExiled::new(
        game_id.clone(),
        player_id,
        card_id.clone(),
        source_zone,
    ))
}

/// Exiles a card from the battlefield.
///
/// # Errors
/// Returns `DomainError::Game::PlayerNotFound` if the player is not found.
/// Returns `DomainError::Card::NotOnBattlefield` if the card is not on the battlefield.
pub fn exile_card_from_battlefield_by_index(
    game_id: &GameId,
    players: &mut [Player],
    player_index: usize,
    card_id: &CardInstanceId,
) -> Result<CardExiled, DomainError> {
    exile_card_from_player_zone_by_index(
        game_id,
        players,
        player_index,
        card_id,
        ZoneType::Battlefield,
        Player::move_battlefield_card_to_exile,
    )
}

/// Exiles a card from the battlefield using an already resolved internal handle.
///
/// # Errors
/// Returns an invariant error if the player index or handle do not match the current
/// aggregate state.
pub fn exile_card_from_battlefield_handle_by_index(
    game_id: &GameId,
    players: &mut [Player],
    player_index: usize,
    handle: PlayerCardHandle,
) -> Result<CardExiled, DomainError> {
    let owner_index = owner_index_for_battlefield_handle(players, player_index, handle)?;
    remove_attached_aura_effects_for_battlefield_handle(players, player_index, handle);
    let card_id = players[player_index]
        .card_by_handle(handle)
        .map(|card| card.id().clone())
        .ok_or_else(|| {
            DomainError::Game(GameError::InternalInvariantViolation(format!(
                "missing card handle {} during battlefield-to-exile move",
                handle.index()
            )))
        })?;

    if owner_index == player_index {
        let owner = players.get_mut(owner_index).ok_or_else(|| {
            DomainError::Game(GameError::InternalInvariantViolation(format!(
                "player index {owner_index} must exist during battlefield-to-exile move"
            )))
        })?;
        let owner_id = owner.id().clone();
        owner
            .move_battlefield_handle_to_exile(handle)
            .ok_or_else(|| {
                DomainError::Card(CardError::NotOnBattlefield {
                    player: owner_id.clone(),
                    card: card_id.clone(),
                })
            })?;
        return Ok(CardExiled::new(
            game_id.clone(),
            owner_id,
            card_id,
            ZoneType::Battlefield,
        ));
    }

    let card = players[player_index]
        .take_battlefield_handle(handle)
        .ok_or_else(|| {
            DomainError::Card(CardError::NotOnBattlefield {
                player: players[player_index].id().clone(),
                card: card_id.clone(),
            })
        })?;
    let owner = players.get_mut(owner_index).ok_or_else(|| {
        DomainError::Game(GameError::InternalInvariantViolation(format!(
            "player index {owner_index} must exist during battlefield-to-exile transfer"
        )))
    })?;
    let owner_id = owner.id().clone();
    owner.receive_exile_card(card);

    Ok(CardExiled::new(
        game_id.clone(),
        owner_id,
        card_id,
        ZoneType::Battlefield,
    ))
}

/// Exiles a card from the battlefield.
///
/// # Errors
/// Returns `DomainError::Game::PlayerNotFound` if the player is not found.
/// Returns `DomainError::Card::NotOnBattlefield` if the card is not on the battlefield.
pub fn exile_card_from_battlefield(
    game_id: &GameId,
    players: &mut [Player],
    player_id: &PlayerId,
    card_id: &CardInstanceId,
) -> Result<CardExiled, DomainError> {
    let player_index = players
        .iter()
        .position(|p| p.id() == player_id)
        .ok_or_else(|| DomainError::Game(GameError::PlayerNotFound(player_id.clone())))?;
    exile_card_from_battlefield_by_index(game_id, players, player_index, card_id)
}

/// Exiles a card from the graveyard.
///
/// # Errors
/// Returns `DomainError::Game::PlayerNotFound` if the player is not found.
/// Returns `DomainError::Card::NotOnBattlefield` if the card is not in the graveyard.
pub fn exile_card_from_graveyard_by_index(
    game_id: &GameId,
    players: &mut [Player],
    player_index: usize,
    card_id: &CardInstanceId,
) -> Result<CardExiled, DomainError> {
    exile_card_from_player_zone_by_index(
        game_id,
        players,
        player_index,
        card_id,
        ZoneType::Graveyard,
        Player::move_graveyard_card_to_exile,
    )
}

/// Exiles a card from the graveyard using an already resolved internal handle.
///
/// # Errors
/// Returns an invariant error if the player index or handle do not match the current
/// aggregate state.
pub fn exile_card_from_graveyard_handle_by_index(
    game_id: &GameId,
    players: &mut [Player],
    player_index: usize,
    handle: PlayerCardHandle,
) -> Result<CardExiled, DomainError> {
    exile_card_from_player_zone_handle_by_index(
        game_id,
        players,
        player_index,
        handle,
        ZoneType::Graveyard,
        Player::move_graveyard_handle_to_exile,
    )
}

/// Exiles a card from the graveyard.
///
/// # Errors
/// Returns `DomainError::Game::PlayerNotFound` if the player is not found.
/// Returns `DomainError::Card::NotOnBattlefield` if the card is not in the graveyard.
pub fn exile_card_from_graveyard(
    game_id: &GameId,
    players: &mut [Player],
    player_id: &PlayerId,
    card_id: &CardInstanceId,
) -> Result<CardExiled, DomainError> {
    let player_index = players
        .iter()
        .position(|p| p.id() == player_id)
        .ok_or_else(|| DomainError::Game(GameError::PlayerNotFound(player_id.clone())))?;
    exile_card_from_graveyard_by_index(game_id, players, player_index, card_id)
}

#[cfg(test)]
mod tests {
    #![allow(clippy::expect_used)]

    use super::*;
    use crate::domain::play::{
        cards::CardInstance,
        ids::{CardDefinitionId, CardInstanceId},
    };

    #[test]
    fn exile_from_controller_battlefield_moves_foreign_owned_card_to_owner_exile() {
        let game_id = GameId::new("game-owner-exile");
        let mut players = vec![
            Player::new(PlayerId::new("p1")),
            Player::new(PlayerId::new("p2")),
        ];
        let card_id = CardInstanceId::new("borrowed-bear");

        players[1].receive_graveyard_card(CardInstance::new_creature(
            card_id.clone(),
            CardDefinitionId::new("borrowed-bear"),
            0,
            2,
            2,
        ));
        let card = players[1]
            .remove_graveyard_card(&card_id)
            .expect("owner graveyard should contain the card");
        players[0].receive_battlefield_card(card);
        let handle = players[0]
            .battlefield_handle(&card_id)
            .expect("controller battlefield should contain the foreign-owned card");

        let event = exile_card_from_battlefield_handle_by_index(&game_id, &mut players, 0, handle)
            .expect("battlefield exile should succeed");

        assert_eq!(event.player_id, PlayerId::new("p2"));
        assert!(players[0].battlefield_card(&card_id).is_none());
        assert!(players[1].exile_card(&card_id).is_some());
    }
}
