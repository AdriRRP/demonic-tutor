use crate::domain::play::cards::{ManaColor, ManaCost};
use crate::domain::play::ids::PlayerId;
use crate::domain::play::zones::{Battlefield, Exile, Graveyard, Hand, Library};

const DEFAULT_STARTING_LIFE: u32 = 20;
pub const OPENING_HAND_SIZE: usize = 7;
pub const MAX_HAND_SIZE: usize = 7;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ManaPool {
    generic: u32,
    green: u32,
    red: u32,
}

impl ManaPool {
    #[must_use]
    pub const fn empty() -> Self {
        Self {
            generic: 0,
            green: 0,
            red: 0,
        }
    }

    #[must_use]
    pub const fn total(&self) -> u32 {
        self.generic + self.green + self.red
    }

    #[must_use]
    pub const fn generic(&self) -> u32 {
        self.generic
    }

    #[must_use]
    pub const fn green(&self) -> u32 {
        self.green
    }

    #[must_use]
    pub const fn red(&self) -> u32 {
        self.red
    }

    pub const fn add_generic(&mut self, amount: u32) {
        self.generic = self.generic.saturating_add(amount);
    }

    pub const fn add_colored(&mut self, color: ManaColor, amount: u32) {
        match color {
            ManaColor::Green => self.green = self.green.saturating_add(amount),
            ManaColor::Red => self.red = self.red.saturating_add(amount),
        }
    }

    pub const fn clear(&mut self) {
        self.generic = 0;
        self.green = 0;
        self.red = 0;
    }

    pub fn spend(&mut self, cost: ManaCost) -> bool {
        if self.green < cost.green_requirement() || self.red < cost.red_requirement() {
            return false;
        }

        let remaining_total = self.total() - cost.green_requirement() - cost.red_requirement();
        if remaining_total < cost.generic_requirement() {
            return false;
        }

        self.green -= cost.green_requirement();
        self.red -= cost.red_requirement();

        let mut generic_to_pay = cost.generic_requirement();
        let pay_from_generic = self.generic.min(generic_to_pay);
        self.generic -= pay_from_generic;
        generic_to_pay -= pay_from_generic;

        let pay_from_green = self.green.min(generic_to_pay);
        self.green -= pay_from_green;
        generic_to_pay -= pay_from_green;

        let pay_from_red = self.red.min(generic_to_pay);
        self.red -= pay_from_red;
        debug_assert_eq!(generic_to_pay - pay_from_red, 0);

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
    pub const fn graveyard(&self) -> &Graveyard {
        &self.graveyard
    }

    pub fn graveyard_mut(&mut self) -> &mut Graveyard {
        &mut self.graveyard
    }

    #[must_use]
    pub const fn exile(&self) -> &Exile {
        &self.exile
    }

    pub fn exile_mut(&mut self) -> &mut Exile {
        &mut self.exile
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
        self.hand.cards().len()
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
