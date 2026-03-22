use crate::domain::play::cards::{ManaColor, ManaCost};
use crate::domain::play::ids::PlayerId;
use crate::domain::play::zones::{Battlefield, Exile, Graveyard, Hand, Library};

const DEFAULT_STARTING_LIFE: u32 = 20;
pub const OPENING_HAND_SIZE: usize = 7;
pub const MAX_HAND_SIZE: usize = 7;

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
        let mut remaining_total = self.total();
        for color in [
            ManaColor::White,
            ManaColor::Blue,
            ManaColor::Black,
            ManaColor::Green,
            ManaColor::Red,
        ] {
            let required = cost.colored_requirement(color);
            if self.colored(color) < required {
                return false;
            }
            remaining_total -= required;
        }

        if remaining_total < cost.generic_requirement() {
            return false;
        }

        for color in [
            ManaColor::White,
            ManaColor::Blue,
            ManaColor::Black,
            ManaColor::Green,
            ManaColor::Red,
        ] {
            let required = cost.colored_requirement(color);
            self.colored[color.index()] -= required;
        }

        let mut generic_to_pay = cost.generic_requirement();
        let pay_from_generic = self.generic.min(generic_to_pay);
        self.generic -= pay_from_generic;
        generic_to_pay -= pay_from_generic;

        for color in [
            ManaColor::White,
            ManaColor::Blue,
            ManaColor::Black,
            ManaColor::Green,
            ManaColor::Red,
        ] {
            if generic_to_pay == 0 {
                break;
            }

            let pay = self.colored[color.index()].min(generic_to_pay);
            self.colored[color.index()] -= pay;
            generic_to_pay -= pay;
        }
        debug_assert_eq!(generic_to_pay, 0);

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
        self.hand.len()
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
