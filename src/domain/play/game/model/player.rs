//! Supports game model player.

use {
    crate::domain::play::cards::{CardInstance, ManaColor, ManaCost, SpellPayload},
    crate::domain::play::ids::{CardDefinitionId, CardInstanceId, PlayerCardHandle, PlayerId},
    crate::domain::play::zones::{Battlefield, Exile, Graveyard, Hand, Library},
    std::collections::HashMap,
};

const DEFAULT_STARTING_LIFE: u32 = 20;
pub const OPENING_HAND_SIZE: usize = 7;
pub const MAX_HAND_SIZE: usize = 7;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlayerCardZone {
    Library,
    Hand,
    Battlefield,
    Graveyard,
    Exile,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PreparedHandSpellCast {
    mana_cost_paid: u32,
    payload: SpellPayload,
}

impl PreparedHandSpellCast {
    #[must_use]
    pub const fn mana_cost_paid(&self) -> u32 {
        self.mana_cost_paid
    }

    #[must_use]
    pub fn into_payload(self) -> SpellPayload {
        self.payload
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PrepareHandSpellCastError {
    MissingCard,
    InsufficientMana { available: u32 },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ManaPool {
    generic: u32,
    colored: [u32; ManaColor::COUNT],
}

impl ManaPool {
    #[must_use]
    pub const fn empty() -> Self {
        Self {
            generic: 0,
            colored: [0; ManaColor::COUNT],
        }
    }

    #[must_use]
    pub fn total(&self) -> u32 {
        self.generic
            + ManaColor::ALL
                .into_iter()
                .map(|color| self.colored(color))
                .sum::<u32>()
    }

    #[must_use]
    pub const fn generic(&self) -> u32 {
        self.generic
    }

    #[must_use]
    pub const fn colored(&self, color: ManaColor) -> u32 {
        self.colored[color.index()]
    }

    #[must_use]
    pub const fn green(&self) -> u32 {
        self.colored(ManaColor::Green)
    }

    #[must_use]
    pub const fn red(&self) -> u32 {
        self.colored(ManaColor::Red)
    }

    #[must_use]
    pub const fn white(&self) -> u32 {
        self.colored(ManaColor::White)
    }

    #[must_use]
    pub const fn blue(&self) -> u32 {
        self.colored(ManaColor::Blue)
    }

    #[must_use]
    pub const fn black(&self) -> u32 {
        self.colored(ManaColor::Black)
    }

    pub const fn add_generic(&mut self, amount: u32) {
        self.generic = self.generic.saturating_add(amount);
    }

    pub const fn add_colored(&mut self, color: ManaColor, amount: u32) {
        self.colored[color.index()] = self.colored[color.index()].saturating_add(amount);
    }

    pub const fn clear(&mut self) {
        self.generic = 0;
        self.colored = [0; ManaColor::COUNT];
    }

    pub fn spend(&mut self, cost: ManaCost) -> bool {
        let mut next_generic = self.generic;
        let mut next_colored = self.colored;

        for color in ManaColor::ALL {
            let required = cost.colored_requirement(color);
            let color_index = color.index();
            if next_colored[color_index] < required {
                return false;
            }
            next_colored[color_index] -= required;
        }

        let mut generic_to_pay = cost.generic_requirement();
        let pay_from_generic = next_generic.min(generic_to_pay);
        next_generic -= pay_from_generic;
        generic_to_pay -= pay_from_generic;

        for color in ManaColor::ALL {
            if generic_to_pay == 0 {
                break;
            }

            let color_index = color.index();
            let pay = next_colored[color_index].min(generic_to_pay);
            next_colored[color_index] -= pay;
            generic_to_pay -= pay;
        }
        if generic_to_pay != 0 {
            return false;
        }

        self.generic = next_generic;
        self.colored = next_colored;
        true
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct PlayerOwnedCard {
    card: CardInstance,
    zone: PlayerCardZone,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
struct PlayerCardArena {
    cards: Vec<Option<PlayerOwnedCard>>,
    id_to_handle: HashMap<CardInstanceId, PlayerCardHandle>,
    free_slots: Vec<usize>,
}

impl PlayerCardArena {
    fn insert(&mut self, card: CardInstance, zone: PlayerCardZone) -> PlayerCardHandle {
        let card_id = card.id().clone();
        let owned_card = PlayerOwnedCard { card, zone };
        let handle = if let Some(index) = self.free_slots.pop() {
            self.cards[index] = Some(owned_card);
            PlayerCardHandle::new(index)
        } else {
            let handle = PlayerCardHandle::new(self.cards.len());
            self.cards.push(Some(owned_card));
            handle
        };
        self.id_to_handle.insert(card_id, handle);
        handle
    }

    fn handle(&self, card_id: &CardInstanceId) -> Option<PlayerCardHandle> {
        self.id_to_handle.get(card_id).copied()
    }

    fn get(&self, card_id: &CardInstanceId) -> Option<&CardInstance> {
        let handle = self.handle(card_id)?;
        self.get_by_handle(handle)
    }

    fn get_mut(&mut self, card_id: &CardInstanceId) -> Option<&mut CardInstance> {
        let handle = self.handle(card_id)?;
        self.get_mut_by_handle(handle)
    }

    fn get_by_handle(&self, handle: PlayerCardHandle) -> Option<&CardInstance> {
        self.cards
            .get(handle.index())
            .and_then(Option::as_ref)
            .map(|owned| &owned.card)
    }

    fn get_mut_by_handle(&mut self, handle: PlayerCardHandle) -> Option<&mut CardInstance> {
        self.cards
            .get_mut(handle.index())
            .and_then(Option::as_mut)
            .map(|owned| &mut owned.card)
    }

    fn begin_remove_by_handle(&mut self, handle: PlayerCardHandle) -> Option<PlayerOwnedCard> {
        self.cards.get_mut(handle.index())?.take()
    }

    fn commit_removed(&mut self, handle: PlayerCardHandle, card_id: &CardInstanceId) {
        self.id_to_handle.remove(card_id);
        self.free_slots.push(handle.index());
    }

    fn rollback_remove(&mut self, handle: PlayerCardHandle, owned: PlayerOwnedCard) -> Option<()> {
        let slot = self.cards.get_mut(handle.index())?;
        if slot.is_some() {
            return None;
        }
        *slot = Some(owned);
        Some(())
    }

    fn zone_by_handle(&self, handle: PlayerCardHandle) -> Option<PlayerCardZone> {
        self.cards
            .get(handle.index())
            .and_then(Option::as_ref)
            .map(|owned| owned.zone)
    }

    fn set_zone(&mut self, handle: PlayerCardHandle, zone: PlayerCardZone) -> Option<()> {
        let owned = self.cards.get_mut(handle.index())?.as_mut()?;
        owned.zone = zone;
        Some(())
    }

    fn remove_by_handle(&mut self, handle: PlayerCardHandle) -> Option<CardInstance> {
        let owned = self.begin_remove_by_handle(handle)?;
        self.commit_removed(handle, owned.card.id());
        Some(owned.card)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Player {
    id: PlayerId,
    library: Library,
    hand: Hand,
    battlefield: Battlefield,
    graveyard: Graveyard,
    exile: Exile,
    cards: PlayerCardArena,
    life: u32,
    mana: ManaPool,
    lands_played_this_turn: usize,
    mulligan_used: bool,
}

#[allow(clippy::missing_const_for_fn)]
impl Player {
    fn handle(&self, card_id: &CardInstanceId) -> Option<PlayerCardHandle> {
        self.cards.handle(card_id)
    }

    fn handle_in_zone(
        &self,
        card_id: &CardInstanceId,
        zone: PlayerCardZone,
    ) -> Option<PlayerCardHandle> {
        let handle = self.handle(card_id)?;
        (self.cards.zone_by_handle(handle) == Some(zone)).then_some(handle)
    }

    fn remove_hand_handle(&mut self, handle: PlayerCardHandle) -> Option<CardInstance> {
        self.hand.remove(handle)?;
        self.cards.remove_by_handle(handle)
    }

    fn remove_battlefield_handle(&mut self, handle: PlayerCardHandle) -> Option<CardInstance> {
        self.battlefield.remove(handle)?;
        self.cards.remove_by_handle(handle)
    }

    fn remove_graveyard_handle(&mut self, handle: PlayerCardHandle) -> Option<CardInstance> {
        self.graveyard.remove(handle)?;
        self.cards.remove_by_handle(handle)
    }

    fn move_handle_between_zones(
        &mut self,
        handle: PlayerCardHandle,
        from: PlayerCardZone,
        to: PlayerCardZone,
    ) -> Option<()> {
        match from {
            PlayerCardZone::Library => return None,
            PlayerCardZone::Hand => self.hand.remove(handle)?,
            PlayerCardZone::Battlefield => self.battlefield.remove(handle)?,
            PlayerCardZone::Graveyard => self.graveyard.remove(handle)?,
            PlayerCardZone::Exile => self.exile.remove(handle)?,
        };

        match to {
            PlayerCardZone::Library => return None,
            PlayerCardZone::Hand => self.hand.receive(vec![handle]),
            PlayerCardZone::Battlefield => self.battlefield.add(handle),
            PlayerCardZone::Graveyard => self.graveyard.add(handle),
            PlayerCardZone::Exile => self.exile.add(handle),
        }

        self.cards.set_zone(handle, to)?;
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
        self.hand_contains(card_id)
            .then(|| self.cards.get(card_id))
            .flatten()
    }

    #[must_use]
    pub fn library_card(&self, card_id: &CardInstanceId) -> Option<&CardInstance> {
        self.library_contains(card_id)
            .then(|| self.cards.get(card_id))
            .flatten()
    }

    #[must_use]
    pub fn hand_card_at(&self, index: usize) -> Option<&CardInstance> {
        let handle = self.hand.handle_at(index)?;
        self.cards.get_by_handle(handle)
    }

    #[must_use]
    pub fn battlefield_card(&self, card_id: &CardInstanceId) -> Option<&CardInstance> {
        self.battlefield_contains(card_id)
            .then(|| self.cards.get(card_id))
            .flatten()
    }

    pub fn battlefield_card_mut(&mut self, card_id: &CardInstanceId) -> Option<&mut CardInstance> {
        self.battlefield_contains(card_id)
            .then(|| self.cards.get_mut(card_id))
            .flatten()
    }

    #[must_use]
    pub fn battlefield_card_at(&self, index: usize) -> Option<&CardInstance> {
        let handle = self.battlefield.handle_at(index)?;
        self.cards.get_by_handle(handle)
    }

    #[must_use]
    pub fn graveyard_card(&self, card_id: &CardInstanceId) -> Option<&CardInstance> {
        self.graveyard_contains(card_id)
            .then(|| self.cards.get(card_id))
            .flatten()
    }

    #[must_use]
    pub fn graveyard_card_at(&self, index: usize) -> Option<&CardInstance> {
        let handle = self.graveyard.handle_at(index)?;
        self.cards.get_by_handle(handle)
    }

    #[must_use]
    pub fn exile_card(&self, card_id: &CardInstanceId) -> Option<&CardInstance> {
        self.exile_contains(card_id)
            .then(|| self.cards.get(card_id))
            .flatten()
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

    pub fn battlefield_card_ids(&self) -> impl Iterator<Item = &CardInstanceId> {
        self.battlefield
            .iter()
            .filter_map(|handle| self.cards.get_by_handle(*handle).map(CardInstance::id))
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
        let handle = self.handle(card_id)?;
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

    pub fn move_battlefield_card_to_exile(&mut self, card_id: &CardInstanceId) -> Option<()> {
        let handle = self.handle_in_zone(card_id, PlayerCardZone::Battlefield)?;
        self.move_handle_between_zones(handle, PlayerCardZone::Battlefield, PlayerCardZone::Exile)
    }

    pub fn move_graveyard_card_to_exile(&mut self, card_id: &CardInstanceId) -> Option<()> {
        let handle = self.handle_in_zone(card_id, PlayerCardZone::Graveyard)?;
        self.move_handle_between_zones(handle, PlayerCardZone::Graveyard, PlayerCardZone::Exile)
    }

    pub fn receive_hand_cards(&mut self, cards: Vec<CardInstance>) {
        let mut handles = Vec::with_capacity(cards.len());
        for card in cards {
            handles.push(self.cards.insert(card, PlayerCardZone::Hand));
        }
        self.hand.receive(handles);
    }

    pub fn receive_library_cards(&mut self, cards: Vec<CardInstance>) {
        let mut handles = Vec::with_capacity(cards.len());
        for card in cards {
            handles.push(self.cards.insert(card, PlayerCardZone::Library));
        }
        self.library.receive(handles);
    }

    pub fn receive_battlefield_card(&mut self, card: CardInstance) {
        let handle = self.cards.insert(card, PlayerCardZone::Battlefield);
        self.battlefield.add(handle);
    }

    pub fn receive_graveyard_card(&mut self, card: CardInstance) {
        let handle = self.cards.insert(card, PlayerCardZone::Graveyard);
        self.graveyard.add(handle);
    }

    pub fn receive_exile_card(&mut self, card: CardInstance) {
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
        assert!(player.cards.get(&card_id).is_some());
    }
}
