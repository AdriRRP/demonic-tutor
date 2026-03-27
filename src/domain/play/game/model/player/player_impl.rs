//! Supports player aggregate operations.

use super::{
    Battlefield, CardDefinitionId, CardInstance, CardInstanceId, Exile, Graveyard, Hand, Library,
    ManaColor, ManaCost, ManaPool, Player, PlayerCardArena, PlayerCardHandle, PlayerCardZone,
    PlayerId, PrepareHandSpellCastError, PreparedHandSpellCast, DEFAULT_STARTING_LIFE,
};
#[allow(clippy::missing_const_for_fn)]
impl Player {
    fn resolve_handle(&self, card_id: &CardInstanceId) -> Option<PlayerCardHandle> {
        self.cards.find_handle(card_id)
    }

    pub(crate) fn resolve_public_card_handle(
        &self,
        card_id: &CardInstanceId,
    ) -> Option<PlayerCardHandle> {
        self.resolve_handle(card_id)
    }

    fn handle_in_zone(
        &self,
        card_id: &CardInstanceId,
        zone: PlayerCardZone,
    ) -> Option<PlayerCardHandle> {
        let handle = self.resolve_handle(card_id)?;
        (self.cards.zone_by_handle(handle) == Some(zone)).then_some(handle)
    }

    fn handle_is_in_zone(&self, handle: PlayerCardHandle, zone: PlayerCardZone) -> bool {
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

    fn remove_hand_handle(&mut self, handle: PlayerCardHandle) -> Option<CardInstance> {
        self.hand.remove(handle)?;
        self.cards.remove_by_handle(handle)
    }

    fn remove_battlefield_handle(&mut self, handle: PlayerCardHandle) -> Option<CardInstance> {
        self.battlefield.remove(handle)?;
        self.cards.remove_by_handle(handle)
    }

    pub(crate) fn take_battlefield_handle(
        &mut self,
        handle: PlayerCardHandle,
    ) -> Option<CardInstance> {
        self.remove_battlefield_handle(handle)
    }

    fn remove_graveyard_handle(&mut self, handle: PlayerCardHandle) -> Option<CardInstance> {
        self.graveyard.remove(handle)?;
        self.cards.remove_by_handle(handle)
    }

    fn remove_handle_from_zone(
        &mut self,
        handle: PlayerCardHandle,
        zone: PlayerCardZone,
    ) -> Option<PlayerCardHandle> {
        match zone {
            PlayerCardZone::Library => None,
            PlayerCardZone::Hand => self.hand.remove(handle),
            PlayerCardZone::Battlefield => self.battlefield.remove(handle),
            PlayerCardZone::Graveyard => self.graveyard.remove(handle),
            PlayerCardZone::Exile => self.exile.remove(handle),
        }
    }

    fn add_handle_to_zone(&mut self, handle: PlayerCardHandle, zone: PlayerCardZone) -> Option<()> {
        match zone {
            PlayerCardZone::Library => return None,
            PlayerCardZone::Hand => self.hand.receive(vec![handle]),
            PlayerCardZone::Battlefield => self.battlefield.add(handle),
            PlayerCardZone::Graveyard => self.graveyard.add(handle),
            PlayerCardZone::Exile => self.exile.add(handle),
        }
        Some(())
    }

    fn apply_battlefield_static_effects_for_entering_handle(
        &mut self,
        handle: PlayerCardHandle,
    ) -> Option<()> {
        let entering_card = self.cards.get_by_handle(handle)?;
        let entering_is_creature = entering_card.creature_stats().is_some();
        let entering_is_anthem = matches!(
            entering_card.controller_static_effect(),
            Some(
                crate::domain::play::cards::ControllerStaticEffectProfile::CreaturesYouControlPlusOnePlusOne
            )
        );

        if entering_is_anthem {
            let creature_handles = self
                .battlefield
                .iter()
                .copied()
                .filter(|current_handle| *current_handle != handle)
                .filter(|current_handle| {
                    self.cards
                        .get_by_handle(*current_handle)
                        .is_some_and(|card| card.creature_stats().is_some())
                })
                .collect::<Vec<_>>();
            for creature_handle in creature_handles {
                self.cards
                    .get_mut_by_handle(creature_handle)
                    .map(|card| card.add_controller_static_stat_bonus(1, 1))?;
            }
        }

        if entering_is_creature {
            let anthem_count = u32::try_from(
                self
                .battlefield
                .iter()
                .copied()
                .filter(|current_handle| {
                    self.cards.get_by_handle(*current_handle).is_some_and(|card| {
                        matches!(
                            card.controller_static_effect(),
                            Some(
                                crate::domain::play::cards::ControllerStaticEffectProfile::CreaturesYouControlPlusOnePlusOne
                            )
                        )
                    })
                })
                .count(),
            )
            .ok()?;
            if anthem_count > 0 {
                self.cards.get_mut_by_handle(handle).map(|card| {
                    card.add_controller_static_stat_bonus(anthem_count, anthem_count);
                })?;
            }
        }

        Some(())
    }

    fn remove_battlefield_static_effects_for_departing_handle(
        &mut self,
        handle: PlayerCardHandle,
    ) -> Option<()> {
        if !matches!(
            self.cards.get_by_handle(handle)?.controller_static_effect(),
            Some(
                crate::domain::play::cards::ControllerStaticEffectProfile::CreaturesYouControlPlusOnePlusOne
            )
        ) {
            return Some(());
        }

        let creature_handles = self
            .battlefield
            .iter()
            .copied()
            .filter(|current_handle| *current_handle != handle)
            .filter(|current_handle| {
                self.cards
                    .get_by_handle(*current_handle)
                    .is_some_and(|card| card.creature_stats().is_some())
            })
            .collect::<Vec<_>>();
        for creature_handle in creature_handles {
            self.cards
                .get_mut_by_handle(creature_handle)
                .map(|card| card.remove_controller_static_stat_bonus(1, 1))?;
        }
        Some(())
    }

    fn move_handle_between_zones(
        &mut self,
        handle: PlayerCardHandle,
        from: PlayerCardZone,
        to: PlayerCardZone,
    ) -> Option<()> {
        self.cards.zone_by_handle(handle)?;
        if from == PlayerCardZone::Battlefield && to != PlayerCardZone::Battlefield {
            self.remove_battlefield_static_effects_for_departing_handle(handle)?;
            self.cards.get_mut_by_handle(handle)?.clear_attachment();
        }
        if from == PlayerCardZone::Battlefield
            && to != PlayerCardZone::Battlefield
            && self.cards.get_by_handle(handle)?.is_token()
        {
            self.remove_handle_from_zone(handle, from)?;
            self.cards.remove_by_handle(handle)?;
            return Some(());
        }
        self.remove_handle_from_zone(handle, from)?;
        self.add_handle_to_zone(handle, to)?;
        if self.cards.set_zone(handle, to).is_none() {
            let _ = self.remove_handle_from_zone(handle, to);
            let _ = self.add_handle_to_zone(handle, from);
            return None;
        }
        if to == PlayerCardZone::Battlefield {
            self.apply_battlefield_static_effects_for_entering_handle(handle)?;
        }
        Some(())
    }

    #[must_use]
    pub fn new(id: PlayerId) -> Self {
        Self {
            id,
            library: Library::new(Vec::new()),
            hand: Hand::new(),
            battlefield: Battlefield::new(),
            graveyard: Graveyard::new(),
            exile: Exile::new(),
            cards: PlayerCardArena::default(),
            life: DEFAULT_STARTING_LIFE,
            mana: ManaPool::empty(),
            lands_played_this_turn: 0,
            mulligan_used: false,
        }
    }

    #[must_use]
    pub const fn id(&self) -> &PlayerId {
        &self.id
    }

    #[must_use]
    pub const fn hand(&self) -> &Hand {
        &self.hand
    }

    #[must_use]
    pub const fn library(&self) -> &Library {
        &self.library
    }

    #[must_use]
    pub const fn battlefield(&self) -> &Battlefield {
        &self.battlefield
    }

    #[must_use]
    pub const fn graveyard(&self) -> &Graveyard {
        &self.graveyard
    }

    #[must_use]
    pub const fn exile(&self) -> &Exile {
        &self.exile
    }

    #[must_use]
    pub const fn life(&self) -> u32 {
        self.life
    }

    #[must_use]
    pub fn mana(&self) -> u32 {
        self.mana.total()
    }

    #[must_use]
    pub const fn mana_pool(&self) -> &ManaPool {
        &self.mana
    }

    #[must_use]
    pub const fn lands_played_this_turn(&self) -> usize {
        self.lands_played_this_turn
    }

    #[must_use]
    pub const fn mulligan_used(&self) -> bool {
        self.mulligan_used
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

    pub fn remove_hand_card(&mut self, card_id: &CardInstanceId) -> Option<CardInstance> {
        let handle = self.handle_in_zone(card_id, PlayerCardZone::Hand)?;
        self.remove_hand_handle(handle)
    }

    pub fn remove_battlefield_card(&mut self, card_id: &CardInstanceId) -> Option<CardInstance> {
        let handle = self.handle_in_zone(card_id, PlayerCardZone::Battlefield)?;
        self.remove_battlefield_handle(handle)
    }

    pub fn remove_graveyard_card(&mut self, card_id: &CardInstanceId) -> Option<CardInstance> {
        let handle = self.handle_in_zone(card_id, PlayerCardZone::Graveyard)?;
        self.remove_graveyard_handle(handle)
    }

    pub fn move_hand_card_to_battlefield(&mut self, card_id: &CardInstanceId) -> Option<()> {
        let handle = self.handle_in_zone(card_id, PlayerCardZone::Hand)?;
        self.move_handle_between_zones(handle, PlayerCardZone::Hand, PlayerCardZone::Battlefield)
    }

    pub fn move_battlefield_card_to_graveyard(&mut self, card_id: &CardInstanceId) -> Option<()> {
        let handle = self.handle_in_zone(card_id, PlayerCardZone::Battlefield)?;
        self.move_handle_between_zones(
            handle,
            PlayerCardZone::Battlefield,
            PlayerCardZone::Graveyard,
        )
    }

    pub fn move_battlefield_card_to_hand(&mut self, card_id: &CardInstanceId) -> Option<()> {
        let handle = self.handle_in_zone(card_id, PlayerCardZone::Battlefield)?;
        self.move_handle_between_zones(handle, PlayerCardZone::Battlefield, PlayerCardZone::Hand)
    }

    pub fn move_hand_card_to_graveyard(&mut self, card_id: &CardInstanceId) -> Option<()> {
        let handle = self.handle_in_zone(card_id, PlayerCardZone::Hand)?;
        self.move_handle_between_zones(handle, PlayerCardZone::Hand, PlayerCardZone::Graveyard)
    }

    pub(crate) fn move_battlefield_handle_to_graveyard(
        &mut self,
        handle: PlayerCardHandle,
    ) -> Option<()> {
        self.handle_is_in_zone(handle, PlayerCardZone::Battlefield)
            .then_some(())
            .and_then(|()| {
                self.move_handle_between_zones(
                    handle,
                    PlayerCardZone::Battlefield,
                    PlayerCardZone::Graveyard,
                )
            })
    }

    pub(crate) fn move_battlefield_handle_to_hand(
        &mut self,
        handle: PlayerCardHandle,
    ) -> Option<()> {
        self.handle_is_in_zone(handle, PlayerCardZone::Battlefield)
            .then_some(())
            .and_then(|()| {
                self.move_handle_between_zones(
                    handle,
                    PlayerCardZone::Battlefield,
                    PlayerCardZone::Hand,
                )
            })
    }

    pub(crate) fn move_hand_handle_to_graveyard(&mut self, handle: PlayerCardHandle) -> Option<()> {
        self.handle_is_in_zone(handle, PlayerCardZone::Hand)
            .then_some(())
            .and_then(|()| {
                self.move_handle_between_zones(
                    handle,
                    PlayerCardZone::Hand,
                    PlayerCardZone::Graveyard,
                )
            })
    }

    pub fn move_battlefield_card_to_exile(&mut self, card_id: &CardInstanceId) -> Option<()> {
        let handle = self.handle_in_zone(card_id, PlayerCardZone::Battlefield)?;
        self.move_handle_between_zones(handle, PlayerCardZone::Battlefield, PlayerCardZone::Exile)
    }

    pub(crate) fn move_battlefield_handle_to_exile(
        &mut self,
        handle: PlayerCardHandle,
    ) -> Option<()> {
        self.handle_is_in_zone(handle, PlayerCardZone::Battlefield)
            .then_some(())
            .and_then(|()| {
                self.move_handle_between_zones(
                    handle,
                    PlayerCardZone::Battlefield,
                    PlayerCardZone::Exile,
                )
            })
    }

    pub fn move_graveyard_card_to_exile(&mut self, card_id: &CardInstanceId) -> Option<()> {
        let handle = self.handle_in_zone(card_id, PlayerCardZone::Graveyard)?;
        self.move_handle_between_zones(handle, PlayerCardZone::Graveyard, PlayerCardZone::Exile)
    }

    pub(crate) fn move_graveyard_handle_to_exile(
        &mut self,
        handle: PlayerCardHandle,
    ) -> Option<()> {
        self.handle_is_in_zone(handle, PlayerCardZone::Graveyard)
            .then_some(())
            .and_then(|()| {
                self.move_handle_between_zones(
                    handle,
                    PlayerCardZone::Graveyard,
                    PlayerCardZone::Exile,
                )
            })
    }

    pub(crate) fn move_graveyard_handle_to_hand(&mut self, handle: PlayerCardHandle) -> Option<()> {
        self.handle_is_in_zone(handle, PlayerCardZone::Graveyard)
            .then_some(())
            .and_then(|()| {
                self.move_handle_between_zones(
                    handle,
                    PlayerCardZone::Graveyard,
                    PlayerCardZone::Hand,
                )
            })
    }

    pub(crate) fn move_graveyard_handle_to_battlefield(
        &mut self,
        handle: PlayerCardHandle,
    ) -> Option<()> {
        self.handle_is_in_zone(handle, PlayerCardZone::Graveyard)
            .then_some(())
            .and_then(|()| {
                self.move_handle_between_zones(
                    handle,
                    PlayerCardZone::Graveyard,
                    PlayerCardZone::Battlefield,
                )
            })
    }

    pub fn receive_hand_cards(&mut self, cards: Vec<CardInstance>) {
        let mut handles = Vec::with_capacity(cards.len());
        for mut card in cards {
            card.ensure_owner(&self.id);
            let handle = self.cards.insert(card, PlayerCardZone::Hand);
            handles.push(handle);
        }
        self.hand.receive(handles);
    }

    pub fn receive_library_cards(&mut self, cards: Vec<CardInstance>) {
        let mut handles = Vec::with_capacity(cards.len());
        for mut card in cards {
            card.ensure_owner(&self.id);
            let handle = self.cards.insert(card, PlayerCardZone::Library);
            handles.push(handle);
        }
        self.library.receive(handles);
    }

    pub fn receive_battlefield_card(&mut self, mut card: CardInstance) -> Option<()> {
        card.ensure_owner(&self.id);
        let handle = self.cards.insert(card, PlayerCardZone::Battlefield);
        self.battlefield.add(handle);
        if self
            .apply_battlefield_static_effects_for_entering_handle(handle)
            .is_none()
        {
            let _ = self.battlefield.remove(handle);
            let _ = self.cards.remove_by_handle(handle);
            return None;
        }
        Some(())
    }

    pub fn receive_graveyard_card(&mut self, mut card: CardInstance) {
        card.ensure_owner(&self.id);
        let handle = self.cards.insert(card, PlayerCardZone::Graveyard);
        self.graveyard.add(handle);
    }

    pub fn receive_exile_card(&mut self, mut card: CardInstance) {
        card.ensure_owner(&self.id);
        let handle = self.cards.insert(card, PlayerCardZone::Exile);
        self.exile.add(handle);
    }

    pub fn draw_cards_into_hand(&mut self, count: usize) -> Option<()> {
        let handles = self.library.draw(count)?;
        for handle in handles.iter().copied() {
            self.cards.set_zone(handle, PlayerCardZone::Hand)?;
        }
        self.hand.receive(handles);
        Some(())
    }

    pub fn draw_one_into_hand(&mut self) -> Option<CardInstanceId> {
        let handle = self.library.draw_one()?;
        let card_id = if let Some(card) = self.cards.get_by_handle(handle) {
            card.id().clone()
        } else {
            return None;
        };
        self.cards.set_zone(handle, PlayerCardZone::Hand)?;
        self.hand.receive(vec![handle]);
        Some(card_id)
    }

    pub fn mill_cards_to_graveyard(&mut self, count: usize) -> Option<Vec<CardInstanceId>> {
        let mut handles = Vec::with_capacity(count);
        for _ in 0..count {
            let Some(handle) = self.library.draw_one() else {
                break;
            };
            handles.push(handle);
        }
        if handles.is_empty() {
            return Some(Vec::new());
        }

        let mut card_ids = Vec::with_capacity(handles.len());
        for handle in handles {
            let card_id = self.cards.get_by_handle(handle)?.id().clone();
            self.cards.set_zone(handle, PlayerCardZone::Graveyard)?;
            self.graveyard.add(handle);
            card_ids.push(card_id);
        }
        Some(card_ids)
    }

    pub fn recycle_hand_into_library(&mut self) {
        let handles = self.hand.drain_all();
        for handle in handles.iter().copied() {
            let _ = self.cards.set_zone(handle, PlayerCardZone::Library);
        }
        self.library.receive(handles);
    }

    pub fn shuffle_library(&mut self) {
        self.library.shuffle();
    }

    pub fn move_top_library_card_to_bottom(&mut self) -> Option<CardInstanceId> {
        let handle = self.library.move_top_to_bottom()?;
        self.cards
            .get_by_handle(handle)
            .map(|card| card.id().clone())
    }

    /// Prepares a spell cast atomically from the player's hand.
    ///
    /// # Errors
    /// Returns `MissingCard` if the card is not still in hand, or
    /// `InsufficientMana` if the player cannot currently pay the provided cost.
    pub fn prepare_hand_spell_cast(
        &mut self,
        card_id: &CardInstanceId,
        mana_cost: u32,
        mana_cost_profile: ManaCost,
    ) -> Result<PreparedHandSpellCast, PrepareHandSpellCastError> {
        let handle = self
            .handle_in_zone(card_id, PlayerCardZone::Hand)
            .ok_or(PrepareHandSpellCastError::MissingCard)?;
        let available = self.mana();
        let mut next_mana = self.mana.clone();
        if !next_mana.spend(mana_cost_profile) {
            return Err(PrepareHandSpellCastError::InsufficientMana { available });
        }

        let owned = self
            .cards
            .begin_remove_by_handle(handle)
            .ok_or(PrepareHandSpellCastError::MissingCard)?;
        if self.hand.remove(handle).is_none() {
            let _ = self.cards.rollback_remove(handle, owned);
            return Err(PrepareHandSpellCastError::MissingCard);
        }
        self.cards.commit_removed(handle, owned.card.id());
        let payload = owned.card.into_spell_payload();
        self.mana = next_mana;

        Ok(PreparedHandSpellCast {
            mana_cost_paid: mana_cost,
            payload,
        })
    }

    /// Prepares a spell cast atomically from the player's graveyard when an
    /// explicit casting permission allows it.
    ///
    /// # Errors
    /// Returns `MissingCard` if the card is not still in graveyard, or
    /// `InsufficientMana` if the player cannot currently pay the provided cost.
    pub fn prepare_graveyard_spell_cast(
        &mut self,
        card_id: &CardInstanceId,
        mana_cost: u32,
        mana_cost_profile: ManaCost,
    ) -> Result<PreparedHandSpellCast, PrepareHandSpellCastError> {
        let handle = self
            .handle_in_zone(card_id, PlayerCardZone::Graveyard)
            .ok_or(PrepareHandSpellCastError::MissingCard)?;
        let available = self.mana();
        let mut next_mana = self.mana.clone();
        if !next_mana.spend(mana_cost_profile) {
            return Err(PrepareHandSpellCastError::InsufficientMana { available });
        }

        let owned = self
            .cards
            .begin_remove_by_handle(handle)
            .ok_or(PrepareHandSpellCastError::MissingCard)?;
        if self.graveyard.remove(handle).is_none() {
            let _ = self.cards.rollback_remove(handle, owned);
            return Err(PrepareHandSpellCastError::MissingCard);
        }
        self.cards.commit_removed(handle, owned.card.id());
        let payload = owned.card.into_spell_payload();
        self.mana = next_mana;

        Ok(PreparedHandSpellCast {
            mana_cost_paid: mana_cost,
            payload,
        })
    }
    pub const fn adjust_life(&mut self, delta: i32) {
        self.life = self.life.saturating_add_signed(delta);
    }

    pub const fn add_mana(&mut self, amount: u32) {
        self.mana.add_generic(amount);
    }

    pub const fn add_colored_mana(&mut self, color: ManaColor, amount: u32) {
        self.mana.add_colored(color, amount);
    }

    pub const fn clear_mana(&mut self) {
        self.mana.clear();
    }

    pub fn spend_mana(&mut self, amount: u32) -> bool {
        self.mana.spend(ManaCost::generic(amount))
    }

    pub fn spend_mana_cost(&mut self, cost: ManaCost) -> bool {
        self.mana.spend(cost)
    }

    pub const fn record_land_played(&mut self) {
        self.lands_played_this_turn += 1;
    }

    pub const fn reset_lands_played(&mut self) {
        self.lands_played_this_turn = 0;
    }

    pub const fn use_mulligan(&mut self) {
        self.mulligan_used = true;
    }

    pub const fn reset_mulligan(&mut self) {
        self.mulligan_used = false;
    }
}

#[cfg(test)]
mod tests {
    use {
        super::{ManaCost, Player, PlayerCardZone, PrepareHandSpellCastError},
        crate::domain::play::cards::CardInstance,
        crate::domain::play::ids::{CardDefinitionId, CardInstanceId, PlayerId},
    };

    #[test]
    fn prepare_hand_spell_cast_rolls_back_arena_state_when_hand_and_arena_are_desynchronized() {
        let player_id = PlayerId::new("player-a");
        let card_id = CardInstanceId::new("card-a");
        let mut player = Player::new(player_id);
        player.receive_hand_cards(vec![CardInstance::new(
            card_id.clone(),
            CardDefinitionId::new("definition-a"),
            crate::domain::play::cards::CardType::Instant,
            1,
        )]);
        player.add_mana(1);

        let handle = player.handle_in_zone(&card_id, PlayerCardZone::Hand);
        assert!(handle.is_some());
        let Some(handle) = handle else { return };
        let removed_handle = player.hand.remove(handle);
        assert_eq!(removed_handle, Some(handle));

        let result = player.prepare_hand_spell_cast(&card_id, 1, ManaCost::generic(1));

        assert_eq!(result, Err(PrepareHandSpellCastError::MissingCard));
        assert_eq!(player.mana(), 1);
        assert_eq!(player.card_zone(&card_id), Some(PlayerCardZone::Hand));
        assert!(player.hand_card(&card_id).is_some());
    }

    #[test]
    fn move_handle_between_zones_rolls_back_visible_zone_when_arena_update_fails() {
        let player_id = PlayerId::new("player-a");
        let card_id = CardInstanceId::new("card-a");
        let mut player = Player::new(player_id);
        assert!(player
            .receive_battlefield_card(CardInstance::new(
                card_id.clone(),
                CardDefinitionId::new("definition-a"),
                crate::domain::play::cards::CardType::Creature,
                2,
            ))
            .is_some());

        let handle = player.handle_in_zone(&card_id, PlayerCardZone::Battlefield);
        assert!(handle.is_some());
        let Some(handle) = handle else { return };
        let removed = player.cards.begin_remove_by_handle(handle);
        assert!(removed.is_some());

        let moved = player.move_handle_between_zones(
            handle,
            PlayerCardZone::Battlefield,
            PlayerCardZone::Graveyard,
        );

        assert_eq!(moved, None);
        assert!(player.battlefield.contains(handle));
        assert!(!player.graveyard.contains(handle));
    }
}
