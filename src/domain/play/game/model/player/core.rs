//! Supports player core state, lifecycle, and mana operations.

use super::{
    Battlefield, Exile, Graveyard, Hand, Library, ManaColor, ManaCost, ManaPool, Player, PlayerId,
    DEFAULT_STARTING_LIFE,
};

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
            cards: super::PlayerCardArena::default(),
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
