use crate::domain::ids::{DeckId, PlayerId};
use crate::domain::zones::{Battlefield, Hand, Library};

const DEFAULT_STARTING_LIFE: u32 = 20;
pub const OPENING_HAND_SIZE: usize = 7;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Player {
    id: PlayerId,
    deck_id: DeckId,
    library: Library,
    hand: Hand,
    battlefield: Battlefield,
    life: u32,
    mana: u32,
    lands_played_this_turn: usize,
    mulligan_used: bool,
}

#[allow(clippy::missing_const_for_fn)]
impl Player {
    #[must_use]
    pub const fn new(id: PlayerId, deck_id: DeckId) -> Self {
        Self {
            id,
            deck_id,
            library: Library::new(Vec::new()),
            hand: Hand::new(),
            battlefield: Battlefield::new(),
            life: DEFAULT_STARTING_LIFE,
            mana: 0,
            lands_played_this_turn: 0,
            mulligan_used: false,
        }
    }

    #[must_use]
    pub const fn id(&self) -> &PlayerId {
        &self.id
    }

    #[must_use]
    pub const fn deck_id(&self) -> &DeckId {
        &self.deck_id
    }

    #[must_use]
    pub const fn hand(&self) -> &Hand {
        &self.hand
    }

    pub fn hand_mut(&mut self) -> &mut Hand {
        &mut self.hand
    }

    #[must_use]
    pub const fn library(&self) -> &Library {
        &self.library
    }

    pub fn library_mut(&mut self) -> &mut Library {
        &mut self.library
    }

    #[must_use]
    pub const fn battlefield(&self) -> &Battlefield {
        &self.battlefield
    }

    pub fn battlefield_mut(&mut self) -> &mut Battlefield {
        &mut self.battlefield
    }

    #[must_use]
    pub const fn life(&self) -> u32 {
        self.life
    }

    #[must_use]
    pub const fn mana(&self) -> u32 {
        self.mana
    }

    #[must_use]
    pub const fn lands_played_this_turn(&self) -> usize {
        self.lands_played_this_turn
    }

    #[must_use]
    pub const fn mulligan_used(&self) -> bool {
        self.mulligan_used
    }

    pub fn set_life(&mut self, new_life: u32) {
        self.life = new_life;
    }

    pub fn change_life(&mut self, delta: i32) {
        self.life = self.life.saturating_add(delta.unsigned_abs());
    }

    pub fn gain_life(&mut self, amount: u32) {
        self.life = self.life.saturating_add(amount);
    }

    pub fn lose_life(&mut self, amount: u32) {
        self.life = self.life.saturating_sub(amount);
    }

    pub fn add_mana(&mut self, amount: u32) {
        self.mana = self.mana.saturating_add(amount);
    }

    pub fn spend_mana(&mut self, amount: u32) -> bool {
        if self.mana >= amount {
            self.mana -= amount;
            true
        } else {
            false
        }
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
