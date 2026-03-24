//! Supports aggregate-level card location indexing.

use {
    super::{Player, PlayerCardZone},
    crate::domain::play::ids::{CardInstanceId, PlayerCardHandle},
    std::collections::HashMap,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AggregateCardLocation {
    owner_index: usize,
    handle: PlayerCardHandle,
    zone: PlayerCardZone,
}

impl AggregateCardLocation {
    #[must_use]
    pub const fn new(owner_index: usize, handle: PlayerCardHandle, zone: PlayerCardZone) -> Self {
        Self {
            owner_index,
            handle,
            zone,
        }
    }

    #[must_use]
    pub const fn owner_index(self) -> usize {
        self.owner_index
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
        index.refresh(players);
        index
    }

    pub fn refresh(&mut self, players: &[Player]) {
        self.by_card_id.clear();
        for (owner_index, player) in players.iter().enumerate() {
            for (card_id, handle, zone) in player.owned_card_locations() {
                self.by_card_id.insert(
                    card_id.clone(),
                    AggregateCardLocation::new(owner_index, handle, zone),
                );
            }
        }
    }

    #[must_use]
    pub fn location(&self, card_id: &CardInstanceId) -> Option<AggregateCardLocation> {
        self.by_card_id.get(card_id).copied()
    }
}
