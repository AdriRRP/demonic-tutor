//! Supports play game helpers.

use {
    super::model::{AggregateCardLocationIndex, Player, PlayerCardZone},
    crate::domain::play::{
        cards::{CardInstance, CardType},
        errors::{CardError, DomainError, GameError},
        ids::{CardInstanceId, PlayerId},
    },
};

pub(super) struct BattlefieldCardLocation<'a> {
    owner_index: usize,
    card: &'a CardInstance,
}

impl<'a> BattlefieldCardLocation<'a> {
    #[must_use]
    pub const fn owner_index(&self) -> usize {
        self.owner_index
    }

    #[must_use]
    pub const fn card(&self) -> &'a CardInstance {
        self.card
    }
}

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
    let player_index = find_player_index(players, player_id)?;
    let opposing_index = opposing_player_index(players, player_index)?;
    Ok(players[opposing_index].id().clone())
}

pub(super) fn opposing_player_index(
    players: &[Player],
    player_index: usize,
) -> Result<usize, DomainError> {
    players
        .iter()
        .enumerate()
        .find_map(|(index, _)| (index != player_index).then_some(index))
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

pub(super) fn player_by_index(
    players: &[Player],
    player_index: usize,
) -> Result<&Player, DomainError> {
    players.get(player_index).ok_or_else(|| {
        DomainError::Game(GameError::InternalInvariantViolation(format!(
            "missing player at index {player_index}"
        )))
    })
}

pub(super) fn player_mut_by_index(
    players: &mut [Player],
    player_index: usize,
) -> Result<&mut Player, DomainError> {
    players.get_mut(player_index).ok_or_else(|| {
        DomainError::Game(GameError::InternalInvariantViolation(format!(
            "missing player at index {player_index}"
        )))
    })
}

pub(super) fn hand_card_type(
    player: &Player,
    player_id: &PlayerId,
    card_id: &CardInstanceId,
) -> Result<CardType, DomainError> {
    hand_card(player, player_id, card_id).map(|card| *card.card_type())
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

pub(super) fn battlefield_card_location<'a>(
    players: &'a [Player],
    card_locations: &AggregateCardLocationIndex,
    card_id: &CardInstanceId,
) -> Option<BattlefieldCardLocation<'a>> {
    let location = card_locations.location(card_id)?;
    (location.zone() == PlayerCardZone::Battlefield).then_some(())?;
    let owner_index = location.owner_index();
    let player = players.get(owner_index)?;
    let card = player.card_by_handle(location.handle())?;
    Some(BattlefieldCardLocation { owner_index, card })
}

pub(super) fn battlefield_card_mut<'a>(
    players: &'a mut [Player],
    card_locations: &AggregateCardLocationIndex,
    card_id: &CardInstanceId,
) -> Option<&'a mut CardInstance> {
    let location = card_locations.location(card_id)?;
    (location.zone() == PlayerCardZone::Battlefield).then_some(())?;
    players
        .get_mut(location.owner_index())?
        .card_mut_by_handle(location.handle())
}
