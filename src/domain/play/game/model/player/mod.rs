//! Supports game model player.

use crate::domain::play::{
    cards::{CardInstance, ManaColor, ManaCost, SpellPayload},
    ids::{CardDefinitionId, CardInstanceId, PlayerCardHandle, PlayerId},
    support::HashMap,
    zones::{Battlefield, Exile, Graveyard, Hand, Library},
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
    free_slots: Vec<usize>,
    public_index: HashMap<CardInstanceId, PlayerCardHandle>,
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
        self.public_index.insert(card_id, handle);
        handle
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

    fn find_handle(&self, card_id: &CardInstanceId) -> Option<PlayerCardHandle> {
        self.public_index.get(card_id).copied()
    }

    fn begin_remove_by_handle(&mut self, handle: PlayerCardHandle) -> Option<PlayerOwnedCard> {
        self.cards.get_mut(handle.index())?.take()
    }

    fn commit_removed(&mut self, handle: PlayerCardHandle, card_id: &CardInstanceId) {
        self.public_index.remove(card_id);
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
        let card_id = owned.card.id().clone();
        self.commit_removed(handle, &card_id);
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
    mulligan_count: u32,
}

mod access;
mod casting;
mod core;
#[cfg(test)]
mod tests;
mod zone_moves;
