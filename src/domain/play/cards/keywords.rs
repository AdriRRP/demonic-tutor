//! Supports play cards keywords.

#[repr(u16)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum KeywordAbility {
    Flying = 1 << 0,
    Reach = 1 << 1,
    Haste = 1 << 2,
    Vigilance = 1 << 3,
    Trample = 1 << 4,
    FirstStrike = 1 << 5,
    Deathtouch = 1 << 6,
    DoubleStrike = 1 << 7,
    Lifelink = 1 << 8,
    Menace = 1 << 9,
    Hexproof = 1 << 10,
    Indestructible = 1 << 11,
    Defender = 1 << 12,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct KeywordAbilitySet(u16);

impl KeywordAbilitySet {
    #[must_use]
    pub const fn empty() -> Self {
        Self(0)
    }

    #[must_use]
    pub const fn only(ability: KeywordAbility) -> Self {
        Self(ability as u16)
    }

    #[must_use]
    pub const fn with(self, ability: KeywordAbility) -> Self {
        Self(self.0 | ability as u16)
    }

    #[must_use]
    pub const fn contains(self, ability: KeywordAbility) -> bool {
        self.0 & ability as u16 != 0
    }
}
