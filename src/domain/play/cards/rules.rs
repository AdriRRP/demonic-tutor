#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CardType {
    Land,
    Creature,
    Instant,
    Sorcery,
    Enchantment,
    Artifact,
    Planeswalker,
}

impl CardType {
    #[must_use]
    pub const fn is_land(&self) -> bool {
        matches!(self, Self::Land)
    }

    #[must_use]
    pub const fn is_spell_card(&self) -> bool {
        !self.is_land()
    }

    #[must_use]
    pub const fn is_creature(&self) -> bool {
        matches!(self, Self::Creature)
    }

    #[must_use]
    pub const fn is_permanent(&self) -> bool {
        matches!(
            self,
            Self::Land | Self::Creature | Self::Enchantment | Self::Artifact | Self::Planeswalker
        )
    }
}

const PERMISSION_OPEN_PRIORITY_WINDOW: u8 = 1 << 0;
const PERMISSION_ACTIVE_PLAYER_EMPTY_MAIN_PHASE_WINDOW: u8 = 1 << 1;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CastingPermissionProfile(u8);

impl CastingPermissionProfile {
    #[allow(non_upper_case_globals)]
    pub const OpenPriorityWindow: Self = Self::open_priority_window();
    #[allow(non_upper_case_globals)]
    pub const ActivePlayerEmptyMainPhaseWindow: Self =
        Self::active_player_empty_main_phase_window();

    #[must_use]
    pub const fn for_spell_card_type(card_type: &CardType) -> Option<Self> {
        match card_type {
            CardType::Instant => Some(Self::open_priority_window()),
            CardType::Creature
            | CardType::Sorcery
            | CardType::Enchantment
            | CardType::Artifact
            | CardType::Planeswalker => Some(Self::active_player_empty_main_phase_window()),
            CardType::Land => None,
        }
    }

    #[must_use]
    pub const fn open_priority_window() -> Self {
        Self(PERMISSION_OPEN_PRIORITY_WINDOW)
    }

    #[must_use]
    pub const fn active_player_empty_main_phase_window() -> Self {
        Self(PERMISSION_ACTIVE_PLAYER_EMPTY_MAIN_PHASE_WINDOW)
    }

    #[must_use]
    pub const fn allows_open_priority_window_cast(self) -> bool {
        self.0 & PERMISSION_OPEN_PRIORITY_WINDOW != 0
    }

    #[must_use]
    pub const fn allows_active_player_empty_main_phase_cast(self) -> bool {
        self.0 & PERMISSION_ACTIVE_PLAYER_EMPTY_MAIN_PHASE_WINDOW != 0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpellTargetKind {
    Player,
    Creature,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SingleTargetRule {
    AnyPlayer,
    AnyCreature,
    AnyPlayerOrCreature,
}

impl SingleTargetRule {
    #[must_use]
    pub const fn matches_target_kind(self, kind: SpellTargetKind) -> bool {
        match self {
            Self::AnyPlayer => matches!(kind, SpellTargetKind::Player),
            Self::AnyCreature => matches!(kind, SpellTargetKind::Creature),
            Self::AnyPlayerOrCreature => {
                matches!(kind, SpellTargetKind::Player | SpellTargetKind::Creature)
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpellTargetingProfile {
    None,
    ExactlyOne(SingleTargetRule),
}

impl SpellTargetingProfile {
    #[must_use]
    pub const fn requires_target(&self) -> bool {
        !matches!(self, Self::None)
    }

    #[must_use]
    pub const fn allows_target_kind(self, kind: SpellTargetKind) -> bool {
        match self {
            Self::None => false,
            Self::ExactlyOne(rule) => rule.matches_target_kind(kind),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpellResolutionProfile {
    None,
    DealDamage { damage: u32 },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SupportedSpellRules {
    targeting: SpellTargetingProfile,
    resolution: SpellResolutionProfile,
}

impl SupportedSpellRules {
    #[must_use]
    pub const fn none() -> Self {
        Self {
            targeting: SpellTargetingProfile::None,
            resolution: SpellResolutionProfile::None,
        }
    }

    #[must_use]
    pub const fn deal_damage_to_any_target(damage: u32) -> Self {
        Self {
            targeting: SpellTargetingProfile::ExactlyOne(SingleTargetRule::AnyPlayerOrCreature),
            resolution: SpellResolutionProfile::DealDamage { damage },
        }
    }

    #[must_use]
    pub const fn deal_damage_to_player(damage: u32) -> Self {
        Self {
            targeting: SpellTargetingProfile::ExactlyOne(SingleTargetRule::AnyPlayer),
            resolution: SpellResolutionProfile::DealDamage { damage },
        }
    }

    #[must_use]
    pub const fn targeting(self) -> SpellTargetingProfile {
        self.targeting
    }

    #[must_use]
    pub const fn resolution(self) -> SpellResolutionProfile {
        self.resolution
    }
}
