//! Supports aggregate-level card location indexing.

use {
    super::{Player, PlayerCardZone},
    crate::domain::play::ids::{CardInstanceId, PlayerCardHandle},
    std::collections::HashMap,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AggregateCardLocation {
    player_index: usize,
    handle: PlayerCardHandle,
    zone: PlayerCardZone,
}

impl AggregateCardLocation {
    #[must_use]
    pub const fn new(player_index: usize, handle: PlayerCardHandle, zone: PlayerCardZone) -> Self {
        Self {
            player_index,
            handle,
            zone,
        }
    }

    #[must_use]
    pub const fn player_index(self) -> usize {
        self.player_index
    }

    #[must_use]
    pub const fn handle(self) -> PlayerCardHandle {
        self.handle
    }

    #[must_use]
    pub const fn zone(self) -> PlayerCardZone {
        self.zone
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct AggregateCardLocationIndex {
    by_card_id: HashMap<CardInstanceId, AggregateCardLocation>,
}

impl AggregateCardLocationIndex {
    #[must_use]
    pub fn from_players(players: &[Player]) -> Self {
        let mut index = Self::default();
        for (player_index, player) in players.iter().enumerate() {
            for (card_id, handle, zone) in player.owned_card_locations() {
                index.by_card_id.insert(
                    card_id.clone(),
                    AggregateCardLocation::new(player_index, handle, zone),
                );
            }
        }
        index
    }

    pub fn upsert(
        &mut self,
        card_id: CardInstanceId,
        player_index: usize,
        handle: PlayerCardHandle,
        zone: PlayerCardZone,
    ) {
        self.by_card_id.insert(
            card_id,
            AggregateCardLocation::new(player_index, handle, zone),
        );
    }

    pub fn set_zone(&mut self, card_id: &CardInstanceId, zone: PlayerCardZone) -> Option<()> {
        let location = self.by_card_id.get_mut(card_id)?;
        location.zone = zone;
        Some(())
    }

    pub fn remove(&mut self, card_id: &CardInstanceId) -> Option<AggregateCardLocation> {
        self.by_card_id.remove(card_id)
    }

    #[must_use]
    pub fn location(&self, card_id: &CardInstanceId) -> Option<AggregateCardLocation> {
        self.by_card_id.get(card_id).copied()
    }
}
