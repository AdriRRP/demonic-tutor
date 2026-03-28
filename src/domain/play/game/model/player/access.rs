//! Supports player card access, lookup, and zone inspection.

use super::{
    CardDefinitionId, CardInstance, CardInstanceId, Player, PlayerCardHandle, PlayerCardZone,
};

#[allow(clippy::missing_const_for_fn)]
impl Player {
    pub(super) fn resolve_handle(&self, card_id: &CardInstanceId) -> Option<PlayerCardHandle> {
        self.cards.find_handle(card_id)
    }

    pub(crate) fn resolve_public_card_handle(
        &self,
        card_id: &CardInstanceId,
    ) -> Option<PlayerCardHandle> {
        self.resolve_handle(card_id)
    }

    pub(super) fn handle_in_zone(
        &self,
        card_id: &CardInstanceId,
        zone: PlayerCardZone,
    ) -> Option<PlayerCardHandle> {
        let handle = self.resolve_handle(card_id)?;
        (self.cards.zone_by_handle(handle) == Some(zone)).then_some(handle)
    }

    pub(super) fn handle_is_in_zone(&self, handle: PlayerCardHandle, zone: PlayerCardZone) -> bool {
        self.cards.zone_by_handle(handle) == Some(zone)
    }

    fn card_by_handle_in_zone(
        &self,
        handle: PlayerCardHandle,
        zone: PlayerCardZone,
    ) -> Option<&CardInstance> {
        self.handle_is_in_zone(handle, zone)
            .then(|| self.cards.get_by_handle(handle))
            .flatten()
    }

    fn card_in_zone(
        &self,
        card_id: &CardInstanceId,
        zone: PlayerCardZone,
    ) -> Option<&CardInstance> {
        let handle = self.handle_in_zone(card_id, zone)?;
        self.cards.get_by_handle(handle)
    }

    fn card_in_zone_mut(
        &mut self,
        card_id: &CardInstanceId,
        zone: PlayerCardZone,
    ) -> Option<&mut CardInstance> {
        let handle = self.handle_in_zone(card_id, zone)?;
        self.cards.get_mut_by_handle(handle)
    }

    #[must_use]
    pub fn hand_size(&self) -> usize {
        self.hand.len()
    }

    #[must_use]
    pub fn library_size(&self) -> usize {
        self.library.len()
    }

    #[must_use]
    pub fn top_library_card_id(&self) -> Option<CardInstanceId> {
        let handle = self.library.peek_one()?;
        self.cards
            .get_by_handle(handle)
            .map(|card| card.id().clone())
    }

    #[must_use]
    pub fn hand_is_empty(&self) -> bool {
        self.hand.is_empty()
    }

    #[must_use]
    pub fn battlefield_size(&self) -> usize {
        self.battlefield.len()
    }

    #[must_use]
    pub fn battlefield_is_empty(&self) -> bool {
        self.battlefield.is_empty()
    }

    #[must_use]
    pub fn graveyard_size(&self) -> usize {
        self.graveyard.len()
    }

    #[must_use]
    pub fn graveyard_is_empty(&self) -> bool {
        self.graveyard.is_empty()
    }

    #[must_use]
    pub fn exile_size(&self) -> usize {
        self.exile.len()
    }

    #[must_use]
    pub fn exile_is_empty(&self) -> bool {
        self.exile.is_empty()
    }

    #[must_use]
    pub fn hand_contains(&self, card_id: &CardInstanceId) -> bool {
        self.handle_in_zone(card_id, PlayerCardZone::Hand).is_some()
    }

    #[must_use]
    pub fn library_contains(&self, card_id: &CardInstanceId) -> bool {
        self.handle_in_zone(card_id, PlayerCardZone::Library)
            .is_some()
    }

    #[must_use]
    pub fn battlefield_contains(&self, card_id: &CardInstanceId) -> bool {
        self.handle_in_zone(card_id, PlayerCardZone::Battlefield)
            .is_some()
    }

    #[must_use]
    pub fn graveyard_contains(&self, card_id: &CardInstanceId) -> bool {
        self.handle_in_zone(card_id, PlayerCardZone::Graveyard)
            .is_some()
    }

    #[must_use]
    pub fn exile_contains(&self, card_id: &CardInstanceId) -> bool {
        self.handle_in_zone(card_id, PlayerCardZone::Exile)
            .is_some()
    }

    #[must_use]
    pub fn hand_card(&self, card_id: &CardInstanceId) -> Option<&CardInstance> {
        self.card_in_zone(card_id, PlayerCardZone::Hand)
    }

    #[must_use]
    pub fn library_card(&self, card_id: &CardInstanceId) -> Option<&CardInstance> {
        self.card_in_zone(card_id, PlayerCardZone::Library)
    }

    #[must_use]
    pub fn hand_card_at(&self, index: usize) -> Option<&CardInstance> {
        let handle = self.hand.handle_at(index)?;
        self.cards.get_by_handle(handle)
    }

    #[must_use]
    pub fn battlefield_card(&self, card_id: &CardInstanceId) -> Option<&CardInstance> {
        self.card_in_zone(card_id, PlayerCardZone::Battlefield)
    }

    pub(crate) fn battlefield_handle(&self, card_id: &CardInstanceId) -> Option<PlayerCardHandle> {
        self.handle_in_zone(card_id, PlayerCardZone::Battlefield)
    }

    pub fn battlefield_card_mut(&mut self, card_id: &CardInstanceId) -> Option<&mut CardInstance> {
        self.card_in_zone_mut(card_id, PlayerCardZone::Battlefield)
    }

    pub(crate) fn battlefield_card_by_handle(
        &self,
        handle: PlayerCardHandle,
    ) -> Option<&CardInstance> {
        self.card_by_handle_in_zone(handle, PlayerCardZone::Battlefield)
    }

    #[must_use]
    pub fn battlefield_card_at(&self, index: usize) -> Option<&CardInstance> {
        let handle = self.battlefield.handle_at(index)?;
        self.cards.get_by_handle(handle)
    }

    #[must_use]
    pub fn graveyard_card(&self, card_id: &CardInstanceId) -> Option<&CardInstance> {
        self.card_in_zone(card_id, PlayerCardZone::Graveyard)
    }

    #[must_use]
    pub fn graveyard_card_at(&self, index: usize) -> Option<&CardInstance> {
        let handle = self.graveyard.handle_at(index)?;
        self.cards.get_by_handle(handle)
    }

    #[must_use]
    pub fn exile_card(&self, card_id: &CardInstanceId) -> Option<&CardInstance> {
        self.card_in_zone(card_id, PlayerCardZone::Exile)
    }

    #[must_use]
    pub fn exile_card_at(&self, index: usize) -> Option<&CardInstance> {
        let handle = self.exile.handle_at(index)?;
        self.cards.get_by_handle(handle)
    }

    #[must_use]
    pub fn hand_card_by_definition(
        &self,
        definition_id: &CardDefinitionId,
    ) -> Option<&CardInstance> {
        self.hand
            .iter()
            .filter_map(|handle| self.cards.get_by_handle(*handle))
            .find(|card| card.definition_id() == definition_id)
    }

    #[must_use]
    pub fn hand_card_ids(&self) -> Vec<CardInstanceId> {
        self.hand
            .iter()
            .filter_map(|handle| self.cards.get_by_handle(*handle))
            .map(|card| card.id().clone())
            .collect()
    }

    #[must_use]
    pub fn battlefield_card_by_definition(
        &self,
        definition_id: &CardDefinitionId,
    ) -> Option<&CardInstance> {
        self.battlefield
            .iter()
            .filter_map(|handle| self.cards.get_by_handle(*handle))
            .find(|card| card.definition_id() == definition_id)
    }

    pub fn battlefield_cards(&self) -> impl Iterator<Item = &CardInstance> {
        self.battlefield
            .iter()
            .filter_map(|handle| self.cards.get_by_handle(*handle))
    }

    pub(crate) fn owned_card_locations(
        &self,
    ) -> impl Iterator<Item = (&CardInstanceId, PlayerCardHandle, PlayerCardZone)> {
        self.cards
            .cards
            .iter()
            .enumerate()
            .filter_map(|(index, slot)| {
                slot.as_ref()
                    .map(|owned| (owned.card.id(), PlayerCardHandle::new(index), owned.zone))
            })
    }

    pub(crate) fn card_by_handle(&self, handle: PlayerCardHandle) -> Option<&CardInstance> {
        self.cards.get_by_handle(handle)
    }

    pub(crate) fn card_mut_by_handle(
        &mut self,
        handle: PlayerCardHandle,
    ) -> Option<&mut CardInstance> {
        self.cards.get_mut_by_handle(handle)
    }

    pub fn battlefield_card_ids(&self) -> impl Iterator<Item = &CardInstanceId> {
        self.battlefield
            .iter()
            .filter_map(|handle| self.cards.get_by_handle(*handle).map(CardInstance::id))
    }

    pub(crate) fn battlefield_handles(&self) -> impl Iterator<Item = PlayerCardHandle> + '_ {
        self.battlefield.iter().copied()
    }

    pub(crate) fn first_instant_or_sorcery_graveyard_handle(&self) -> Option<PlayerCardHandle> {
        self.graveyard.iter().copied().find(|handle| {
            self.cards.get_by_handle(*handle).is_some_and(|card| {
                matches!(
                    card.card_type(),
                    crate::domain::play::cards::CardType::Instant
                        | crate::domain::play::cards::CardType::Sorcery
                )
            })
        })
    }

    pub fn for_each_battlefield_card_mut<F>(&mut self, mut f: F)
    where
        F: FnMut(&mut CardInstance),
    {
        for index in 0..self.battlefield.len() {
            let Some(handle) = self.battlefield.handle_at(index) else {
                continue;
            };
            if let Some(card) = self.cards.get_mut_by_handle(handle) {
                f(card);
            }
        }
    }

    #[must_use]
    pub fn card_zone(&self, card_id: &CardInstanceId) -> Option<PlayerCardZone> {
        let handle = self.resolve_handle(card_id)?;
        self.cards.zone_by_handle(handle)
    }

    #[must_use]
    pub fn owns_card(&self, card_id: &CardInstanceId) -> bool {
        self.card_zone(card_id).is_some()
    }
}
