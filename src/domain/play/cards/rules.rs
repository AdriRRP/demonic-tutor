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
pub enum CastingRule {
    OpenPriorityWindow,
    ActivePlayerEmptyMainPhaseWindow,
}

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
    pub const fn supports(self, rule: CastingRule) -> bool {
        match rule {
            CastingRule::OpenPriorityWindow => self.0 & PERMISSION_OPEN_PRIORITY_WINDOW != 0,
            CastingRule::ActivePlayerEmptyMainPhaseWindow => {
                self.0 & PERMISSION_ACTIVE_PLAYER_EMPTY_MAIN_PHASE_WINDOW != 0
            }
        }
    }

    #[must_use]
    pub const fn allows_open_priority_window_cast(self) -> bool {
        self.supports(CastingRule::OpenPriorityWindow)
    }

    #[must_use]
    pub const fn allows_active_player_empty_main_phase_cast(self) -> bool {
        self.supports(CastingRule::ActivePlayerEmptyMainPhaseWindow)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpellTargetKind {
    Player,
    Creature,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlayerTargetRule {
    AnyPlayer,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CreatureTargetRule {
    AnyCreatureOnBattlefield,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SingleTargetRule {
    player_rule: Option<PlayerTargetRule>,
    creature_rule: Option<CreatureTargetRule>,
}

impl SingleTargetRule {
    #[must_use]
    pub const fn any_player() -> Self {
        Self {
            player_rule: Some(PlayerTargetRule::AnyPlayer),
            creature_rule: None,
        }
    }

    #[must_use]
    pub const fn any_creature_on_battlefield() -> Self {
        Self {
            player_rule: None,
            creature_rule: Some(CreatureTargetRule::AnyCreatureOnBattlefield),
        }
    }

    #[must_use]
    pub const fn any_player_or_creature_on_battlefield() -> Self {
        Self {
            player_rule: Some(PlayerTargetRule::AnyPlayer),
            creature_rule: Some(CreatureTargetRule::AnyCreatureOnBattlefield),
        }
    }

    #[must_use]
    pub const fn matches_target_kind(self, kind: SpellTargetKind) -> bool {
        match self {
            Self {
                player_rule: Some(_),
                creature_rule: None,
            } => matches!(kind, SpellTargetKind::Player),
            Self {
                player_rule: None,
                creature_rule: Some(_),
            } => matches!(kind, SpellTargetKind::Creature),
            Self {
                player_rule: Some(_),
                creature_rule: Some(_),
            } => matches!(kind, SpellTargetKind::Player | SpellTargetKind::Creature),
            Self {
                player_rule: None,
                creature_rule: None,
            } => false,
        }
    }

    #[must_use]
    pub const fn player_rule(self) -> Option<PlayerTargetRule> {
        self.player_rule
    }

    #[must_use]
    pub const fn creature_rule(self) -> Option<CreatureTargetRule> {
        self.creature_rule
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
            targeting: SpellTargetingProfile::ExactlyOne(
                SingleTargetRule::any_player_or_creature_on_battlefield(),
            ),
            resolution: SpellResolutionProfile::DealDamage { damage },
        }
    }

    #[must_use]
    pub const fn deal_damage_to_player(damage: u32) -> Self {
        Self {
            targeting: SpellTargetingProfile::ExactlyOne(SingleTargetRule::any_player()),
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
