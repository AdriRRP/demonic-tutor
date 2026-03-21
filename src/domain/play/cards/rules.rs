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
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpellTargetKind {
    Player,
    Creature,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlayerTargetRule {
    AnyPlayer,
    OpponentOfActor,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CreatureTargetRule {
    AnyCreatureOnBattlefield,
    CreatureControlledByActor,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SingleTargetRule {
    Player(PlayerTargetRule),
    Creature(CreatureTargetRule),
    PlayerOrCreature {
        player: PlayerTargetRule,
        creature: CreatureTargetRule,
    },
}

impl SingleTargetRule {
    #[must_use]
    pub const fn any_player() -> Self {
        Self::Player(PlayerTargetRule::AnyPlayer)
    }

    #[must_use]
    pub const fn any_creature_on_battlefield() -> Self {
        Self::Creature(CreatureTargetRule::AnyCreatureOnBattlefield)
    }

    #[must_use]
    pub const fn opponent_of_actor() -> Self {
        Self::Player(PlayerTargetRule::OpponentOfActor)
    }

    #[must_use]
    pub const fn creature_controlled_by_actor() -> Self {
        Self::Creature(CreatureTargetRule::CreatureControlledByActor)
    }

    #[must_use]
    pub const fn any_player_or_creature_on_battlefield() -> Self {
        Self::PlayerOrCreature {
            player: PlayerTargetRule::AnyPlayer,
            creature: CreatureTargetRule::AnyCreatureOnBattlefield,
        }
    }

    #[must_use]
    pub const fn matches_target_kind(self, kind: SpellTargetKind) -> bool {
        match self {
            Self::Player(_) => matches!(kind, SpellTargetKind::Player),
            Self::Creature(_) => matches!(kind, SpellTargetKind::Creature),
            Self::PlayerOrCreature { .. } => {
                matches!(kind, SpellTargetKind::Player | SpellTargetKind::Creature)
            }
        }
    }

    #[must_use]
    pub const fn player_rule(self) -> Option<PlayerTargetRule> {
        match self {
            Self::Player(rule) | Self::PlayerOrCreature { player: rule, .. } => Some(rule),
            Self::Creature(_) => None,
        }
    }

    #[must_use]
    pub const fn creature_rule(self) -> Option<CreatureTargetRule> {
        match self {
            Self::Creature(rule) | Self::PlayerOrCreature { creature: rule, .. } => Some(rule),
            Self::Player(_) => None,
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
    pub const fn deal_damage_to_opponent(damage: u32) -> Self {
        Self {
            targeting: SpellTargetingProfile::ExactlyOne(SingleTargetRule::opponent_of_actor()),
            resolution: SpellResolutionProfile::DealDamage { damage },
        }
    }

    #[must_use]
    pub const fn deal_damage_to_controlled_creature(damage: u32) -> Self {
        Self {
            targeting: SpellTargetingProfile::ExactlyOne(
                SingleTargetRule::creature_controlled_by_actor(),
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
