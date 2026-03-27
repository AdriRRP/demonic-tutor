//! Supports player zone moves, receives, and battlefield static effects.

use super::{CardInstance, CardInstanceId, Player, PlayerCardHandle, PlayerCardZone};

#[allow(clippy::missing_const_for_fn)]
impl Player {
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

    pub(super) fn move_handle_between_zones(
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
}
