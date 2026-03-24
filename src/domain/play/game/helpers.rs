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

pub(super) struct GraveyardCardLocation {
    owner_index: usize,
}

impl GraveyardCardLocation {
    #[must_use]
    pub const fn owner_index(&self) -> usize {
        self.owner_index
    }
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
    let card = player.battlefield_card(card_id)?;
    Some(BattlefieldCardLocation { owner_index, card })
}

pub(super) fn battlefield_card_mut<'a>(
    players: &'a mut [Player],
    card_locations: &AggregateCardLocationIndex,
    card_id: &CardInstanceId,
) -> Option<&'a mut CardInstance> {
    let owner_index = battlefield_card_location(players, card_locations, card_id)?.owner_index();
    players[owner_index].battlefield_card_mut(card_id)
}

pub(super) fn graveyard_card_location(
    players: &[Player],
    card_locations: &AggregateCardLocationIndex,
    card_id: &CardInstanceId,
) -> Option<GraveyardCardLocation> {
    let location = card_locations.location(card_id)?;
    (location.zone() == PlayerCardZone::Graveyard).then_some(())?;
    players
        .get(location.owner_index())?
        .graveyard_card(card_id)?;
    Some(GraveyardCardLocation {
        owner_index: location.owner_index(),
    })
}
