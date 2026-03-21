use crate::domain::play::ids::{CardDefinitionId, CardInstanceId};

const FLAG_TAPPED: u8 = 1 << 0;
const FLAG_SUMMONING_SICKNESS: u8 = 1 << 1;
const FLAG_ATTACKING: u8 = 1 << 2;
const FLAG_BLOCKING: u8 = 1 << 3;

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
pub enum CastingTimingProfile {
    InstantSpeed,
    SorcerySpeed,
}

impl CastingTimingProfile {
    #[must_use]
    pub const fn for_card_type(card_type: &CardType) -> Self {
        match card_type {
            CardType::Instant => Self::InstantSpeed,
            CardType::Creature
            | CardType::Sorcery
            | CardType::Enchantment
            | CardType::Artifact
            | CardType::Planeswalker
            | CardType::Land => Self::SorcerySpeed,
        }
    }

    #[must_use]
    pub const fn allows_cast_while_holding_priority(self) -> bool {
        matches!(self, Self::InstantSpeed)
    }

    #[must_use]
    pub const fn requires_empty_main_phase_window(self) -> bool {
        matches!(self, Self::SorcerySpeed)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum SpellTargetKind {
    Player = 1 << 0,
    Creature = 1 << 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct SpellTargetKindSet(u8);

impl SpellTargetKindSet {
    #[must_use]
    pub const fn empty() -> Self {
        Self(0)
    }

    #[must_use]
    pub const fn only(kind: SpellTargetKind) -> Self {
        Self(kind as u8)
    }

    #[must_use]
    pub const fn any_supported_target() -> Self {
        Self::only(SpellTargetKind::Player).with(SpellTargetKind::Creature)
    }

    #[must_use]
    pub const fn with(self, kind: SpellTargetKind) -> Self {
        Self(self.0 | kind as u8)
    }

    #[must_use]
    pub const fn contains(self, kind: SpellTargetKind) -> bool {
        self.0 & kind as u8 != 0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpellTargetingProfile {
    None,
    SingleTarget(SpellTargetKindSet),
}

impl SpellTargetingProfile {
    #[must_use]
    pub const fn requires_target(&self) -> bool {
        !matches!(self, Self::None)
    }

    #[must_use]
    pub const fn accepts_kind(self, kind: SpellTargetKind) -> bool {
        match self {
            Self::None => false,
            Self::SingleTarget(kinds) => kinds.contains(kind),
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
            targeting: SpellTargetingProfile::SingleTarget(
                SpellTargetKindSet::any_supported_target(),
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CardDefinition {
    id: CardDefinitionId,
    mana_cost: u32,
    casting_timing: CastingTimingProfile,
    supported_spell_rules: SupportedSpellRules,
}

impl CardDefinition {
    #[must_use]
    pub const fn new(id: CardDefinitionId, mana_cost: u32) -> Self {
        Self {
            id,
            mana_cost,
            casting_timing: CastingTimingProfile::SorcerySpeed,
            supported_spell_rules: SupportedSpellRules::none(),
        }
    }

    #[must_use]
    pub const fn for_card_type(id: CardDefinitionId, mana_cost: u32, card_type: &CardType) -> Self {
        Self {
            id,
            mana_cost,
            casting_timing: CastingTimingProfile::for_card_type(card_type),
            supported_spell_rules: SupportedSpellRules::none(),
        }
    }

    #[must_use]
    pub const fn with_supported_spell_rules(
        mut self,
        supported_spell_rules: SupportedSpellRules,
    ) -> Self {
        self.supported_spell_rules = supported_spell_rules;
        self
    }

    #[must_use]
    pub const fn id(&self) -> &CardDefinitionId {
        &self.id
    }

    #[must_use]
    pub const fn mana_cost(&self) -> u32 {
        self.mana_cost
    }

    #[must_use]
    pub const fn casting_timing(&self) -> CastingTimingProfile {
        self.casting_timing
    }

    #[must_use]
    pub const fn supported_spell_rules(&self) -> SupportedSpellRules {
        self.supported_spell_rules
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct CreatureRuntime {
    power: u32,
    toughness: u32,
    damage: u32,
    blocking_target: Option<CardInstanceId>,
    keywords: KeywordAbilitySet,
}

impl CreatureRuntime {
    const fn new(power: u32, toughness: u32) -> Self {
        Self {
            power,
            toughness,
            damage: 0,
            blocking_target: None,
            keywords: KeywordAbilitySet::empty(),
        }
    }

    const fn new_with_keywords(power: u32, toughness: u32, keywords: KeywordAbilitySet) -> Self {
        Self {
            power,
            toughness,
            damage: 0,
            blocking_target: None,
            keywords,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct CardFace {
    definition: CardDefinition,
    card_type: CardType,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CardInstance {
    id: CardInstanceId,
    face: CardFace,
    flags: u8,
    creature: Option<CreatureRuntime>,
}

impl CardInstance {
    #[must_use]
    pub(crate) const fn from_definition(
        id: CardInstanceId,
        definition: CardDefinition,
        card_type: CardType,
    ) -> Self {
        Self {
            id,
            face: CardFace {
                definition,
                card_type,
            },
            flags: 0,
            creature: None,
        }
    }

    #[must_use]
    pub const fn new(
        id: CardInstanceId,
        definition_id: CardDefinitionId,
        card_type: CardType,
        mana_cost: u32,
    ) -> Self {
        Self::from_definition(
            id,
            CardDefinition::for_card_type(definition_id, mana_cost, &card_type),
            card_type,
        )
    }

    #[must_use]
    pub const fn new_creature(
        id: CardInstanceId,
        definition_id: CardDefinitionId,
        mana_cost: u32,
        power: u32,
        toughness: u32,
    ) -> Self {
        Self {
            id,
            face: CardFace {
                definition: CardDefinition::for_card_type(
                    definition_id,
                    mana_cost,
                    &CardType::Creature,
                ),
                card_type: CardType::Creature,
            },
            flags: FLAG_SUMMONING_SICKNESS,
            creature: Some(CreatureRuntime::new(power, toughness)),
        }
    }

    #[must_use]
    pub const fn new_creature_with_keywords(
        id: CardInstanceId,
        definition: CardDefinition,
        power: u32,
        toughness: u32,
        keywords: KeywordAbilitySet,
    ) -> Self {
        Self {
            id,
            face: CardFace {
                definition,
                card_type: CardType::Creature,
            },
            flags: FLAG_SUMMONING_SICKNESS,
            creature: Some(CreatureRuntime::new_with_keywords(
                power, toughness, keywords,
            )),
        }
    }

    #[must_use]
    pub const fn id(&self) -> &CardInstanceId {
        &self.id
    }

    #[must_use]
    pub const fn definition_id(&self) -> &CardDefinitionId {
        self.face.definition.id()
    }

    #[must_use]
    pub const fn card_type(&self) -> &CardType {
        &self.face.card_type
    }

    #[must_use]
    pub const fn is_tapped(&self) -> bool {
        self.flags & FLAG_TAPPED != 0
    }

    #[must_use]
    pub const fn mana_cost(&self) -> u32 {
        self.face.definition.mana_cost()
    }

    #[must_use]
    pub const fn casting_timing_profile(&self) -> CastingTimingProfile {
        self.face.definition.casting_timing()
    }

    #[must_use]
    pub const fn supported_spell_rules(&self) -> SupportedSpellRules {
        self.face.definition.supported_spell_rules()
    }

    #[must_use]
    pub const fn power(&self) -> Option<u32> {
        match &self.creature {
            Some(creature) => Some(creature.power),
            None => None,
        }
    }

    #[must_use]
    pub const fn toughness(&self) -> Option<u32> {
        match &self.creature {
            Some(creature) => Some(creature.toughness),
            None => None,
        }
    }

    #[must_use]
    pub const fn creature_stats(&self) -> Option<(u32, u32)> {
        match (&self.face.card_type, &self.creature) {
            (CardType::Creature, Some(creature)) => Some((creature.power, creature.toughness)),
            _ => None,
        }
    }

    #[must_use]
    pub const fn has_summoning_sickness(&self) -> bool {
        self.creature.is_some() && (self.flags & FLAG_SUMMONING_SICKNESS != 0)
    }

    #[must_use]
    pub const fn is_attacking(&self) -> bool {
        self.creature.is_some() && (self.flags & FLAG_ATTACKING != 0)
    }

    #[must_use]
    pub const fn is_blocking(&self) -> bool {
        self.creature.is_some() && (self.flags & FLAG_BLOCKING != 0)
    }

    pub const fn tap(&mut self) {
        self.flags |= FLAG_TAPPED;
    }

    pub const fn untap(&mut self) {
        self.flags &= !FLAG_TAPPED;
    }

    pub const fn remove_summoning_sickness(&mut self) {
        if self.creature.is_some() {
            self.flags &= !FLAG_SUMMONING_SICKNESS;
        }
    }

    pub const fn set_attacking(&mut self, attacking: bool) {
        if self.creature.is_some() {
            if attacking {
                self.flags |= FLAG_ATTACKING;
            } else {
                self.flags &= !FLAG_ATTACKING;
            }
        }
    }

    pub fn set_blocking(&mut self, blocking: bool) {
        if let Some(creature) = &mut self.creature {
            if blocking {
                self.flags |= FLAG_BLOCKING;
            } else {
                self.flags &= !FLAG_BLOCKING;
                creature.blocking_target = None;
            }
        }
    }

    #[must_use]
    pub const fn blocking_target(&self) -> Option<&CardInstanceId> {
        match &self.creature {
            Some(creature) => creature.blocking_target.as_ref(),
            None => None,
        }
    }

    pub fn assign_blocking_target(&mut self, attacker_id: CardInstanceId) {
        if let Some(creature) = &mut self.creature {
            self.flags |= FLAG_BLOCKING;
            creature.blocking_target = Some(attacker_id);
        }
    }

    #[must_use]
    pub const fn damage(&self) -> u32 {
        match &self.creature {
            Some(creature) => creature.damage,
            None => 0,
        }
    }

    #[must_use]
    pub const fn has_lethal_damage(&self) -> bool {
        match &self.creature {
            Some(creature) => creature.damage >= creature.toughness,
            None => false,
        }
    }

    #[must_use]
    pub const fn has_zero_toughness(&self) -> bool {
        match &self.creature {
            Some(creature) => creature.toughness == 0,
            None => false,
        }
    }

    pub const fn add_damage(&mut self, damage: u32) {
        if let Some(creature) = &mut self.creature {
            creature.damage += damage;
        }
    }

    pub const fn clear_damage(&mut self) {
        if let Some(creature) = &mut self.creature {
            creature.damage = 0;
        }
    }

    #[must_use]
    pub const fn has_keyword(&self, ability: KeywordAbility) -> bool {
        match &self.creature {
            Some(creature) => creature.keywords.contains(ability),
            None => false,
        }
    }

    #[must_use]
    pub const fn has_flying(&self) -> bool {
        self.has_keyword(KeywordAbility::Flying)
    }

    #[must_use]
    pub const fn has_reach(&self) -> bool {
        self.has_keyword(KeywordAbility::Reach)
    }

    #[must_use]
    pub const fn keyword_abilities(&self) -> Option<KeywordAbilitySet> {
        match &self.creature {
            Some(creature) => Some(creature.keywords),
            None => None,
        }
    }
}
