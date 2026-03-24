use crate::domain::play::cards::{CardInstance, ManaColor, ManaCost};
use crate::domain::play::ids::{CardDefinitionId, CardInstanceId, PlayerId};
use crate::domain::play::zones::{Battlefield, Exile, Graveyard, Hand, Library};
use std::collections::HashMap;

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
    pub const fn total(&self) -> u32 {
        self.generic
            + self.colored[ManaColor::White.index()]
            + self.colored[ManaColor::Blue.index()]
            + self.colored[ManaColor::Black.index()]
            + self.colored[ManaColor::Green.index()]
            + self.colored[ManaColor::Red.index()]
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
pub struct Player {
    id: PlayerId,
    library: Library,
    hand: Hand,
    battlefield: Battlefield,
    graveyard: Graveyard,
    exile: Exile,
    cards: HashMap<CardInstanceId, CardInstance>,
    life: u32,
    mana: ManaPool,
    lands_played_this_turn: usize,
    mulligan_used: bool,
}

#[allow(clippy::missing_const_for_fn)]
impl Player {
    #[must_use]
    pub fn new(id: PlayerId) -> Self {
        Self {
            id,
            library: Library::new(Vec::new()),
            hand: Hand::new(),
            battlefield: Battlefield::new(),
            graveyard: Graveyard::new(),
            exile: Exile::new(),
            cards: HashMap::new(),
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
    pub const fn mana(&self) -> u32 {
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
        self.hand.contains(card_id)
    }

    #[must_use]
    pub fn library_contains(&self, card_id: &CardInstanceId) -> bool {
        self.library.iter().any(|stored_id| stored_id == card_id)
            && self.cards.contains_key(card_id)
    }

    #[must_use]
    pub fn battlefield_contains(&self, card_id: &CardInstanceId) -> bool {
        self.battlefield.contains(card_id)
    }

    #[must_use]
    pub fn graveyard_contains(&self, card_id: &CardInstanceId) -> bool {
        self.graveyard.contains(card_id)
    }

    #[must_use]
    pub fn exile_contains(&self, card_id: &CardInstanceId) -> bool {
        self.exile.contains(card_id)
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
        let card_id = self.hand.card_id_at(index)?;
        self.cards.get(card_id)
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
        let card_id = self.battlefield.card_id_at(index)?;
        self.cards.get(card_id)
    }

    #[must_use]
    pub fn graveyard_card(&self, card_id: &CardInstanceId) -> Option<&CardInstance> {
        self.graveyard_contains(card_id)
            .then(|| self.cards.get(card_id))
            .flatten()
    }

    #[must_use]
    pub fn graveyard_card_at(&self, index: usize) -> Option<&CardInstance> {
        let card_id = self.graveyard.card_id_at(index)?;
        self.cards.get(card_id)
    }

    #[must_use]
    pub fn exile_card(&self, card_id: &CardInstanceId) -> Option<&CardInstance> {
        self.exile_contains(card_id)
            .then(|| self.cards.get(card_id))
            .flatten()
    }

    #[must_use]
    pub fn exile_card_at(&self, index: usize) -> Option<&CardInstance> {
        let card_id = self.exile.card_id_at(index)?;
        self.cards.get(card_id)
    }

    #[must_use]
    pub fn hand_card_by_definition(
        &self,
        definition_id: &CardDefinitionId,
    ) -> Option<&CardInstance> {
        self.hand
            .iter()
            .filter_map(|card_id| self.cards.get(card_id))
            .find(|card| card.definition_id() == definition_id)
    }

    #[must_use]
    pub fn hand_card_ids(&self) -> Vec<CardInstanceId> {
        self.hand.iter().cloned().collect()
    }

    #[must_use]
    pub fn battlefield_card_by_definition(
        &self,
        definition_id: &CardDefinitionId,
    ) -> Option<&CardInstance> {
        self.battlefield
            .iter()
            .filter_map(|card_id| self.cards.get(card_id))
            .find(|card| card.definition_id() == definition_id)
    }

    pub fn battlefield_cards(&self) -> impl Iterator<Item = &CardInstance> {
        self.battlefield
            .iter()
            .filter_map(|card_id| self.cards.get(card_id))
    }

    pub fn battlefield_card_ids(&self) -> impl Iterator<Item = &CardInstanceId> {
        self.battlefield.iter()
    }

    pub fn for_each_battlefield_card_mut<F>(&mut self, mut f: F)
    where
        F: FnMut(&mut CardInstance),
    {
        let card_ids = self
            .battlefield
            .iter()
            .cloned()
            .collect::<Vec<CardInstanceId>>();

        for card_id in card_ids {
            if let Some(card) = self.cards.get_mut(&card_id) {
                f(card);
            }
        }
    }

    fn move_owned_card(
        cards: &HashMap<CardInstanceId, CardInstance>,
        source_remove: impl FnOnce() -> Option<CardInstanceId>,
        destination_add: impl FnOnce(CardInstanceId),
    ) -> Option<()> {
        let card_id = source_remove()?;
        if cards.contains_key(&card_id) {
            destination_add(card_id);
            Some(())
        } else {
            None
        }
    }

    #[must_use]
    pub fn card_zone(&self, card_id: &CardInstanceId) -> Option<PlayerCardZone> {
        if self.hand_contains(card_id) {
            return Some(PlayerCardZone::Hand);
        }
        if self.battlefield_contains(card_id) {
            return Some(PlayerCardZone::Battlefield);
        }
        if self.graveyard_contains(card_id) {
            return Some(PlayerCardZone::Graveyard);
        }
        if self.exile_contains(card_id) {
            return Some(PlayerCardZone::Exile);
        }
        if self.library_contains(card_id) {
            return Some(PlayerCardZone::Library);
        }

        None
    }

    #[must_use]
    pub fn owns_card(&self, card_id: &CardInstanceId) -> bool {
        self.card_zone(card_id).is_some()
    }

    pub fn remove_hand_card(&mut self, card_id: &CardInstanceId) -> Option<CardInstance> {
        self.hand.remove(card_id)?;
        self.cards.remove(card_id)
    }

    pub fn remove_battlefield_card(&mut self, card_id: &CardInstanceId) -> Option<CardInstance> {
        self.battlefield.remove(card_id)?;
        self.cards.remove(card_id)
    }

    pub fn remove_graveyard_card(&mut self, card_id: &CardInstanceId) -> Option<CardInstance> {
        self.graveyard.remove(card_id)?;
        self.cards.remove(card_id)
    }

    pub fn move_hand_card_to_battlefield(&mut self, card_id: &CardInstanceId) -> Option<()> {
        Self::move_owned_card(
            &self.cards,
            || self.hand.remove(card_id),
            |moved_card_id| {
                self.battlefield.add(moved_card_id);
            },
        )
    }

    pub fn move_battlefield_card_to_graveyard(&mut self, card_id: &CardInstanceId) -> Option<()> {
        Self::move_owned_card(
            &self.cards,
            || self.battlefield.remove(card_id),
            |moved_card_id| {
                self.graveyard.add(moved_card_id);
            },
        )
    }

    pub fn move_battlefield_card_to_exile(&mut self, card_id: &CardInstanceId) -> Option<()> {
        Self::move_owned_card(
            &self.cards,
            || self.battlefield.remove(card_id),
            |moved_card_id| {
                self.exile.add(moved_card_id);
            },
        )
    }

    pub fn move_graveyard_card_to_exile(&mut self, card_id: &CardInstanceId) -> Option<()> {
        Self::move_owned_card(
            &self.cards,
            || self.graveyard.remove(card_id),
            |moved_card_id| {
                self.exile.add(moved_card_id);
            },
        )
    }

    pub fn receive_hand_cards(&mut self, cards: Vec<CardInstance>) {
        let mut card_ids = Vec::with_capacity(cards.len());
        for card in cards {
            let card_id = card.id().clone();
            card_ids.push(card_id.clone());
            self.cards.insert(card_id, card);
        }
        self.hand.receive(card_ids);
    }

    pub fn receive_library_cards(&mut self, cards: Vec<CardInstance>) {
        let mut card_ids = Vec::with_capacity(cards.len());
        for card in cards {
            let card_id = card.id().clone();
            card_ids.push(card_id.clone());
            self.cards.insert(card_id, card);
        }
        self.library.receive(card_ids);
    }

    pub fn receive_battlefield_card(&mut self, card: CardInstance) {
        let card_id = card.id().clone();
        self.battlefield.add(card_id.clone());
        self.cards.insert(card_id, card);
    }

    pub fn receive_graveyard_card(&mut self, card: CardInstance) {
        let card_id = card.id().clone();
        self.graveyard.add(card_id.clone());
        self.cards.insert(card_id, card);
    }

    pub fn receive_exile_card(&mut self, card: CardInstance) {
        let card_id = card.id().clone();
        self.exile.add(card_id.clone());
        self.cards.insert(card_id, card);
    }

    pub fn draw_cards_into_hand(&mut self, count: usize) -> Option<()> {
        let card_ids = self.library.draw(count)?;
        self.hand.receive(card_ids);
        Some(())
    }

    pub fn draw_one_into_hand(&mut self) -> Option<CardInstanceId> {
        let card_id = self.library.draw_one()?;
        if !self.cards.contains_key(&card_id) {
            return None;
        }
        self.hand.receive(vec![card_id.clone()]);
        Some(card_id)
    }

    pub fn recycle_hand_into_library(&mut self) {
        let card_ids = self.hand.drain_all();
        self.library.receive(card_ids);
    }

    pub fn shuffle_library(&mut self) {
        self.library.shuffle();
    }

    pub fn adjust_life(&mut self, delta: i32) {
        self.life = self.life.saturating_add_signed(delta);
    }

    pub fn add_mana(&mut self, amount: u32) {
        self.mana.add_generic(amount);
    }

    pub fn add_colored_mana(&mut self, color: ManaColor, amount: u32) {
        self.mana.add_colored(color, amount);
    }

    pub fn clear_mana(&mut self) {
        self.mana.clear();
    }

    pub fn spend_mana(&mut self, amount: u32) -> bool {
        self.mana.spend(ManaCost::generic(amount))
    }

    pub fn spend_mana_cost(&mut self, cost: ManaCost) -> bool {
        self.mana.spend(cost)
    }

    pub fn record_land_played(&mut self) {
        self.lands_played_this_turn += 1;
    }

    pub fn reset_lands_played(&mut self) {
        self.lands_played_this_turn = 0;
    }

    pub fn use_mulligan(&mut self) {
        self.mulligan_used = true;
    }

    pub fn reset_mulligan(&mut self) {
        self.mulligan_used = false;
    }
}
