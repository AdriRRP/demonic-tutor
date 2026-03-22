use super::model::Player;
use crate::domain::play::{
    cards::{CardInstance, CardType},
    errors::{CardError, DomainError, GameError},
    ids::{CardInstanceId, PlayerId},
};

pub(super) fn find_player_index(
    players: &[Player],
    player_id: &PlayerId,
) -> Result<usize, DomainError> {
    players
        .iter()
        .position(|player| player.id() == player_id)
        .ok_or_else(|| DomainError::Game(GameError::PlayerNotFound(player_id.clone())))
}

pub(super) fn opposing_player_id(
    players: &[Player],
    player_id: &PlayerId,
) -> Result<PlayerId, DomainError> {
    players
        .iter()
        .find(|player| player.id() != player_id)
        .map(|player| player.id().clone())
        .ok_or_else(|| {
            DomainError::Game(GameError::InternalInvariantViolation(
                "a two-player game should always produce a winner when one player loses"
                    .to_string(),
            ))
        })
}

pub(super) fn find_player_mut<'a>(
    players: &'a mut [Player],
    player_id: &PlayerId,
) -> Result<&'a mut Player, DomainError> {
    let player_index = find_player_index(players, player_id)?;
    Ok(&mut players[player_index])
}

pub(super) fn hand_card_type(
    player: &Player,
    player_id: &PlayerId,
    card_id: &CardInstanceId,
) -> Result<CardType, DomainError> {
    hand_card(player, player_id, card_id).map(|card| card.card_type().clone())
}

pub(super) fn hand_card<'a>(
    player: &'a Player,
    player_id: &PlayerId,
    card_id: &CardInstanceId,
) -> Result<&'a CardInstance, DomainError> {
    player.hand_card(card_id).ok_or_else(|| {
        DomainError::Card(CardError::NotInHand {
            player: player_id.clone(),
            card: card_id.clone(),
        })
    })
}

pub(super) fn remove_card_from_hand(
    player: &mut Player,
    player_id: &PlayerId,
    card_id: &CardInstanceId,
) -> Result<CardInstance, DomainError> {
    player.remove_hand_card(card_id).ok_or_else(|| {
        DomainError::Card(CardError::NotInHand {
            player: player_id.clone(),
            card: card_id.clone(),
        })
    })
}

pub(super) fn battlefield_card_owner<'a>(
    players: &'a [Player],
    card_id: &CardInstanceId,
) -> Option<(&'a PlayerId, &'a CardInstance)> {
    players.iter().find_map(|player| {
        player
            .battlefield_card(card_id)
            .map(|card| (player.id(), card))
    })
}

pub(super) fn battlefield_card_mut<'a>(
    players: &'a mut [Player],
    card_id: &CardInstanceId,
) -> Option<&'a mut CardInstance> {
    for player in players.iter_mut() {
        if let Some(card) = player.battlefield_card_mut(card_id) {
            return Some(card);
        }
    }

    None
}
