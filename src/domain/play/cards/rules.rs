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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CastingPermissionProfile {
    OpenPriorityWindow,
    ActivePlayerEmptyMainPhaseWindow,
}

impl CastingPermissionProfile {
    #[must_use]
    pub const fn for_spell_card_type(card_type: &CardType) -> Option<Self> {
        match card_type {
            CardType::Instant => Some(Self::OpenPriorityWindow),
            CardType::Creature
            | CardType::Sorcery
            | CardType::Enchantment
            | CardType::Artifact
            | CardType::Planeswalker => Some(Self::ActivePlayerEmptyMainPhaseWindow),
            CardType::Land => None,
        }
    }

    #[must_use]
    pub const fn allows_open_priority_window_cast(self) -> bool {
        matches!(self, Self::OpenPriorityWindow)
    }

    #[must_use]
    pub const fn allows_active_player_empty_main_phase_cast(self) -> bool {
        matches!(self, Self::ActivePlayerEmptyMainPhaseWindow)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpellTargetKind {
    Player,
    Creature,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpellTargetLegalityRule {
    Player,
    Creature,
    PlayerOrCreature,
}

impl SpellTargetLegalityRule {
    #[must_use]
    pub const fn allows_target_kind(self, kind: SpellTargetKind) -> bool {
        match self {
            Self::Player => matches!(kind, SpellTargetKind::Player),
            Self::Creature => matches!(kind, SpellTargetKind::Creature),
            Self::PlayerOrCreature => {
                matches!(kind, SpellTargetKind::Player | SpellTargetKind::Creature)
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpellTargetingProfile {
    None,
    ExactlyOneLegalTarget(SpellTargetLegalityRule),
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
            Self::ExactlyOneLegalTarget(rule) => rule.allows_target_kind(kind),
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
            targeting: SpellTargetingProfile::ExactlyOneLegalTarget(
                SpellTargetLegalityRule::PlayerOrCreature,
            ),
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
