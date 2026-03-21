#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum KeywordAbility {
    Flying = 1 << 0,
    Reach = 1 << 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct KeywordAbilitySet(u8);

impl KeywordAbilitySet {
    #[must_use]
    pub const fn empty() -> Self {
        Self(0)
    }

    #[must_use]
    pub const fn only(ability: KeywordAbility) -> Self {
        Self(ability as u8)
    }

    #[must_use]
    pub const fn with(self, ability: KeywordAbility) -> Self {
        Self(self.0 | ability as u8)
    }

    #[must_use]
    pub const fn contains(self, ability: KeywordAbility) -> bool {
        self.0 & ability as u8 != 0
    }
}
