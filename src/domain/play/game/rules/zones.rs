//! Supports game rules zones.

use {
    super::super::model::Player,
    crate::domain::play::{
        errors::{CardError, DomainError, GameError},
        events::{CardExiled, ZoneType},
        ids::{CardInstanceId, GameId, PlayerCardHandle, PlayerId},
    },
};

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
    exile_card_from_player_zone_handle_by_index(
        game_id,
        players,
        player_index,
        handle,
        ZoneType::Battlefield,
        Player::move_battlefield_handle_to_exile,
    )
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
