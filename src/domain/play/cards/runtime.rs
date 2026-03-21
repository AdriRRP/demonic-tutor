use super::{
    CardDefinition, CardType, CastingPermissionProfile, KeywordAbility, KeywordAbilitySet,
    SupportedSpellRules,
};
use crate::domain::play::ids::{CardDefinitionId, CardInstanceId};

const FLAG_TAPPED: u8 = 1 << 0;
const FLAG_SUMMONING_SICKNESS: u8 = 1 << 1;
const FLAG_ATTACKING: u8 = 1 << 2;
const FLAG_BLOCKING: u8 = 1 << 3;

#[derive(Debug, Clone, PartialEq, Eq)]
struct CreatureState {
    power: u32,
    toughness: u32,
    damage: u32,
    blocking_target: Option<CardInstanceId>,
    keywords: KeywordAbilitySet,
}

impl CreatureState {
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
struct CardRuntime {
    flags: u8,
    creature: Option<CreatureState>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CardInstance {
    id: CardInstanceId,
    face: CardFace,
    runtime: CardRuntime,
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
            runtime: CardRuntime {
                flags: 0,
                creature: None,
            },
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
            runtime: CardRuntime {
                flags: FLAG_SUMMONING_SICKNESS,
                creature: Some(CreatureState::new(power, toughness)),
            },
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
            runtime: CardRuntime {
                flags: FLAG_SUMMONING_SICKNESS,
                creature: Some(CreatureState::new_with_keywords(power, toughness, keywords)),
            },
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
        self.runtime.flags & FLAG_TAPPED != 0
    }

    #[must_use]
    pub const fn mana_cost(&self) -> u32 {
        self.face.definition.mana_cost()
    }

    #[must_use]
    pub const fn casting_permission_profile(&self) -> Option<CastingPermissionProfile> {
        self.face.definition.casting_permission()
    }

    #[must_use]
    pub const fn supported_spell_rules(&self) -> SupportedSpellRules {
        self.face.definition.supported_spell_rules()
    }

    #[must_use]
    pub const fn power(&self) -> Option<u32> {
        match &self.runtime.creature {
            Some(creature) => Some(creature.power),
            None => None,
        }
    }

    #[must_use]
    pub const fn toughness(&self) -> Option<u32> {
        match &self.runtime.creature {
            Some(creature) => Some(creature.toughness),
            None => None,
        }
    }

    #[must_use]
    pub const fn creature_stats(&self) -> Option<(u32, u32)> {
        match (&self.face.card_type, &self.runtime.creature) {
            (CardType::Creature, Some(creature)) => Some((creature.power, creature.toughness)),
            _ => None,
        }
    }

    #[must_use]
    pub const fn has_summoning_sickness(&self) -> bool {
        self.runtime.creature.is_some() && (self.runtime.flags & FLAG_SUMMONING_SICKNESS != 0)
    }

    #[must_use]
    pub const fn is_attacking(&self) -> bool {
        self.runtime.creature.is_some() && (self.runtime.flags & FLAG_ATTACKING != 0)
    }

    #[must_use]
    pub const fn is_blocking(&self) -> bool {
        self.runtime.creature.is_some() && (self.runtime.flags & FLAG_BLOCKING != 0)
    }

    pub const fn tap(&mut self) {
        self.runtime.flags |= FLAG_TAPPED;
    }

    pub const fn untap(&mut self) {
        self.runtime.flags &= !FLAG_TAPPED;
    }

    pub const fn remove_summoning_sickness(&mut self) {
        if self.runtime.creature.is_some() {
            self.runtime.flags &= !FLAG_SUMMONING_SICKNESS;
        }
    }

    pub const fn set_attacking(&mut self, attacking: bool) {
        if self.runtime.creature.is_some() {
            if attacking {
                self.runtime.flags |= FLAG_ATTACKING;
            } else {
                self.runtime.flags &= !FLAG_ATTACKING;
            }
        }
    }

    pub fn set_blocking(&mut self, blocking: bool) {
        if let Some(creature) = &mut self.runtime.creature {
            if blocking {
                self.runtime.flags |= FLAG_BLOCKING;
            } else {
                self.runtime.flags &= !FLAG_BLOCKING;
                creature.blocking_target = None;
            }
        }
    }

    #[must_use]
    pub const fn blocking_target(&self) -> Option<&CardInstanceId> {
        match &self.runtime.creature {
            Some(creature) => creature.blocking_target.as_ref(),
            None => None,
        }
    }

    pub fn assign_blocking_target(&mut self, attacker_id: CardInstanceId) {
        if let Some(creature) = &mut self.runtime.creature {
            self.runtime.flags |= FLAG_BLOCKING;
            creature.blocking_target = Some(attacker_id);
        }
    }

    #[must_use]
    pub const fn damage(&self) -> u32 {
        match &self.runtime.creature {
            Some(creature) => creature.damage,
            None => 0,
        }
    }

    #[must_use]
    pub const fn has_lethal_damage(&self) -> bool {
        match &self.runtime.creature {
            Some(creature) => creature.damage >= creature.toughness,
            None => false,
        }
    }

    #[must_use]
    pub const fn has_zero_toughness(&self) -> bool {
        match &self.runtime.creature {
            Some(creature) => creature.toughness == 0,
            None => false,
        }
    }

    pub const fn add_damage(&mut self, damage: u32) {
        if let Some(creature) = &mut self.runtime.creature {
            creature.damage += damage;
        }
    }

    pub const fn clear_damage(&mut self) {
        if let Some(creature) = &mut self.runtime.creature {
            creature.damage = 0;
        }
    }

    #[must_use]
    pub const fn has_keyword(&self, ability: KeywordAbility) -> bool {
        match &self.runtime.creature {
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
        match &self.runtime.creature {
            Some(creature) => Some(creature.keywords),
            None => None,
        }
    }
}
