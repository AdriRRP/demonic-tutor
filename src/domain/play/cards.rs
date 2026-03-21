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
    pub const fn is_instant(&self) -> bool {
        matches!(self, Self::Instant)
    }

    #[must_use]
    pub const fn is_sorcery_speed_spell(&self) -> bool {
        self.is_spell_card() && !self.is_instant()
    }

    #[must_use]
    pub const fn is_permanent(&self) -> bool {
        matches!(
            self,
            Self::Land | Self::Creature | Self::Enchantment | Self::Artifact | Self::Planeswalker
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SpellEffectProfile {
    None,
    DealDamageToAnyTarget { damage: u32 },
}

impl SpellEffectProfile {
    #[must_use]
    pub const fn requires_target(&self) -> bool {
        !matches!(self, Self::None)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CardDefinition {
    id: CardDefinitionId,
    mana_cost: u32,
    spell_effect: SpellEffectProfile,
}

impl CardDefinition {
    #[must_use]
    pub const fn new(id: CardDefinitionId, mana_cost: u32) -> Self {
        Self {
            id,
            mana_cost,
            spell_effect: SpellEffectProfile::None,
        }
    }

    #[must_use]
    pub const fn with_spell_effect(mut self, spell_effect: SpellEffectProfile) -> Self {
        self.spell_effect = spell_effect;
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
    pub const fn spell_effect(&self) -> &SpellEffectProfile {
        &self.spell_effect
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

impl CardFace {
    const fn new(definition_id: CardDefinitionId, card_type: CardType, mana_cost: u32) -> Self {
        Self {
            definition: CardDefinition::new(definition_id, mana_cost),
            card_type,
        }
    }
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
        Self::from_definition(id, CardDefinition::new(definition_id, mana_cost), card_type)
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
            face: CardFace::new(definition_id, CardType::Creature, mana_cost),
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
    pub const fn spell_effect_profile(&self) -> &SpellEffectProfile {
        self.face.definition.spell_effect()
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
    pub const fn has_flying(&self) -> bool {
        match &self.creature {
            Some(creature) => creature.keywords.contains(KeywordAbility::Flying),
            None => false,
        }
    }

    #[must_use]
    pub const fn has_reach(&self) -> bool {
        match &self.creature {
            Some(creature) => creature.keywords.contains(KeywordAbility::Reach),
            None => false,
        }
    }

    #[must_use]
    pub const fn keyword_abilities(&self) -> Option<KeywordAbilitySet> {
        match &self.creature {
            Some(creature) => Some(creature.keywords),
            None => None,
        }
    }
}
