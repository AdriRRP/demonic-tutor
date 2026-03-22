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

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ManaColor {
    White = 0,
    Blue = 1,
    Black = 2,
    Green,
    Red,
}

impl ManaColor {
    pub const ALL: [Self; Self::COUNT] =
        [Self::White, Self::Blue, Self::Black, Self::Green, Self::Red];

    pub const COUNT: usize = 5;

    #[must_use]
    pub const fn index(self) -> usize {
        self as usize
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ManaCost {
    generic: u32,
    colored: [u32; ManaColor::COUNT],
}

impl ManaCost {
    #[must_use]
    pub const fn generic(amount: u32) -> Self {
        Self {
            generic: amount,
            colored: [0; ManaColor::COUNT],
        }
    }

    #[must_use]
    pub const fn green(amount: u32) -> Self {
        Self::single_color(ManaColor::Green, amount)
    }

    #[must_use]
    pub const fn red(amount: u32) -> Self {
        Self::single_color(ManaColor::Red, amount)
    }

    #[must_use]
    pub const fn white(amount: u32) -> Self {
        Self::single_color(ManaColor::White, amount)
    }

    #[must_use]
    pub const fn blue(amount: u32) -> Self {
        Self::single_color(ManaColor::Blue, amount)
    }

    #[must_use]
    pub const fn black(amount: u32) -> Self {
        Self::single_color(ManaColor::Black, amount)
    }

    #[must_use]
    pub const fn single_color(color: ManaColor, amount: u32) -> Self {
        let mut colored = [0; ManaColor::COUNT];
        colored[color.index()] = amount;
        Self {
            generic: 0,
            colored,
        }
    }

    #[must_use]
    pub const fn generic_plus_single_color(
        generic: u32,
        color: ManaColor,
        colored_amount: u32,
    ) -> Self {
        let mut colored = [0; ManaColor::COUNT];
        colored[color.index()] = colored_amount;
        Self { generic, colored }
    }

    #[must_use]
    pub const fn total(self) -> u32 {
        self.generic
            + self.colored[ManaColor::White.index()]
            + self.colored[ManaColor::Blue.index()]
            + self.colored[ManaColor::Black.index()]
            + self.colored[ManaColor::Green.index()]
            + self.colored[ManaColor::Red.index()]
    }

    #[must_use]
    pub const fn generic_requirement(self) -> u32 {
        self.generic
    }

    #[must_use]
    pub const fn colored_requirement(self, color: ManaColor) -> u32 {
        self.colored[color.index()]
    }

    #[must_use]
    pub const fn green_requirement(self) -> u32 {
        self.colored_requirement(ManaColor::Green)
    }

    #[must_use]
    pub const fn red_requirement(self) -> u32 {
        self.colored_requirement(ManaColor::Red)
    }

    #[must_use]
    pub const fn white_requirement(self) -> u32 {
        self.colored_requirement(ManaColor::White)
    }

    #[must_use]
    pub const fn blue_requirement(self) -> u32 {
        self.colored_requirement(ManaColor::Blue)
    }

    #[must_use]
    pub const fn black_requirement(self) -> u32 {
        self.colored_requirement(ManaColor::Black)
    }
}

const PERMISSION_OPEN_PRIORITY_WINDOW: u8 = 1 << 0;
const PERMISSION_ACTIVE_PLAYER_EMPTY_MAIN_PHASE_WINDOW: u8 = 1 << 1;
const PERMISSION_OPEN_PRIORITY_WINDOW_DURING_OWN_TURN: u8 = 1 << 2;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CastingRule {
    OpenPriorityWindow,
    ActivePlayerEmptyMainPhaseWindow,
    OpenPriorityWindowDuringOwnTurn,
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
    pub const fn with_rule(mut self, rule: CastingRule) -> Self {
        match rule {
            CastingRule::OpenPriorityWindow => {
                self.0 |= PERMISSION_OPEN_PRIORITY_WINDOW;
            }
            CastingRule::ActivePlayerEmptyMainPhaseWindow => {
                self.0 |= PERMISSION_ACTIVE_PLAYER_EMPTY_MAIN_PHASE_WINDOW;
            }
            CastingRule::OpenPriorityWindowDuringOwnTurn => {
                self.0 |= PERMISSION_OPEN_PRIORITY_WINDOW_DURING_OWN_TURN;
            }
        }
        self
    }

    #[must_use]
    pub const fn supports(self, rule: CastingRule) -> bool {
        match rule {
            CastingRule::OpenPriorityWindow => self.0 & PERMISSION_OPEN_PRIORITY_WINDOW != 0,
            CastingRule::ActivePlayerEmptyMainPhaseWindow => {
                self.0 & PERMISSION_ACTIVE_PLAYER_EMPTY_MAIN_PHASE_WINDOW != 0
            }
            CastingRule::OpenPriorityWindowDuringOwnTurn => {
                self.0 & PERMISSION_OPEN_PRIORITY_WINDOW_DURING_OWN_TURN != 0
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

impl PlayerTargetRule {
    #[must_use]
    pub const fn allows(self, target_is_actor: bool) -> bool {
        match self {
            Self::AnyPlayer => true,
            Self::OpponentOfActor => !target_is_actor,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CreatureTargetRule {
    AnyCreatureOnBattlefield,
    CreatureControlledByActor,
    CreatureControlledByOpponent,
    AttackingCreature,
    BlockingCreature,
    CreatureControlledByActorAndAttacking,
    CreatureControlledByActorAndBlocking,
    BlockingCreatureControlledByOpponent,
    AttackingCreatureControlledByOpponent,
}

impl CreatureTargetRule {
    #[must_use]
    pub const fn allows(
        self,
        target_controlled_by_actor: bool,
        target_is_attacking: bool,
        target_is_blocking: bool,
    ) -> bool {
        match self {
            Self::AnyCreatureOnBattlefield => true,
            Self::CreatureControlledByActor => target_controlled_by_actor,
            Self::CreatureControlledByOpponent => !target_controlled_by_actor,
            Self::AttackingCreature => target_is_attacking,
            Self::BlockingCreature => target_is_blocking,
            Self::CreatureControlledByActorAndAttacking => {
                target_controlled_by_actor && target_is_attacking
            }
            Self::CreatureControlledByActorAndBlocking => {
                target_controlled_by_actor && target_is_blocking
            }
            Self::BlockingCreatureControlledByOpponent => {
                !target_controlled_by_actor && target_is_blocking
            }
            Self::AttackingCreatureControlledByOpponent => {
                !target_controlled_by_actor && target_is_attacking
            }
        }
    }
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
    pub const fn attacking_creature() -> Self {
        Self::Creature(CreatureTargetRule::AttackingCreature)
    }

    #[must_use]
    pub const fn opponents_creature() -> Self {
        Self::Creature(CreatureTargetRule::CreatureControlledByOpponent)
    }

    #[must_use]
    pub const fn blocking_creature() -> Self {
        Self::Creature(CreatureTargetRule::BlockingCreature)
    }

    #[must_use]
    pub const fn controlled_blocking_creature() -> Self {
        Self::Creature(CreatureTargetRule::CreatureControlledByActorAndBlocking)
    }

    #[must_use]
    pub const fn controlled_attacking_creature() -> Self {
        Self::Creature(CreatureTargetRule::CreatureControlledByActorAndAttacking)
    }

    #[must_use]
    pub const fn opponents_blocking_creature() -> Self {
        Self::Creature(CreatureTargetRule::BlockingCreatureControlledByOpponent)
    }

    #[must_use]
    pub const fn opponents_attacking_creature() -> Self {
        Self::Creature(CreatureTargetRule::AttackingCreatureControlledByOpponent)
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
    DestroyTargetCreature,
    ExileTargetCreature,
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
    pub const fn deal_damage_to_opponents_creature(damage: u32) -> Self {
        Self {
            targeting: SpellTargetingProfile::ExactlyOne(SingleTargetRule::opponents_creature()),
            resolution: SpellResolutionProfile::DealDamage { damage },
        }
    }

    #[must_use]
    pub const fn deal_damage_to_attacking_creature(damage: u32) -> Self {
        Self {
            targeting: SpellTargetingProfile::ExactlyOne(SingleTargetRule::attacking_creature()),
            resolution: SpellResolutionProfile::DealDamage { damage },
        }
    }

    #[must_use]
    pub const fn deal_damage_to_blocking_creature(damage: u32) -> Self {
        Self {
            targeting: SpellTargetingProfile::ExactlyOne(SingleTargetRule::blocking_creature()),
            resolution: SpellResolutionProfile::DealDamage { damage },
        }
    }

    #[must_use]
    pub const fn deal_damage_to_controlled_blocking_creature(damage: u32) -> Self {
        Self {
            targeting: SpellTargetingProfile::ExactlyOne(
                SingleTargetRule::controlled_blocking_creature(),
            ),
            resolution: SpellResolutionProfile::DealDamage { damage },
        }
    }

    #[must_use]
    pub const fn deal_damage_to_controlled_attacking_creature(damage: u32) -> Self {
        Self {
            targeting: SpellTargetingProfile::ExactlyOne(
                SingleTargetRule::controlled_attacking_creature(),
            ),
            resolution: SpellResolutionProfile::DealDamage { damage },
        }
    }

    #[must_use]
    pub const fn deal_damage_to_opponents_blocking_creature(damage: u32) -> Self {
        Self {
            targeting: SpellTargetingProfile::ExactlyOne(
                SingleTargetRule::opponents_blocking_creature(),
            ),
            resolution: SpellResolutionProfile::DealDamage { damage },
        }
    }

    #[must_use]
    pub const fn deal_damage_to_opponents_attacking_creature(damage: u32) -> Self {
        Self {
            targeting: SpellTargetingProfile::ExactlyOne(
                SingleTargetRule::opponents_attacking_creature(),
            ),
            resolution: SpellResolutionProfile::DealDamage { damage },
        }
    }

    #[must_use]
    pub const fn destroy_target_creature() -> Self {
        Self {
            targeting: SpellTargetingProfile::ExactlyOne(
                SingleTargetRule::any_creature_on_battlefield(),
            ),
            resolution: SpellResolutionProfile::DestroyTargetCreature,
        }
    }

    #[must_use]
    pub const fn exile_target_creature() -> Self {
        Self {
            targeting: SpellTargetingProfile::ExactlyOne(
                SingleTargetRule::any_creature_on_battlefield(),
            ),
            resolution: SpellResolutionProfile::ExileTargetCreature,
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
