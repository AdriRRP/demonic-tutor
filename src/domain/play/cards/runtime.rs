//! Supports play cards runtime.

use {
    super::{
        ActivatedAbilityProfile, ActivatedManaAbilityProfile, CardDefinition, CardType,
        CastingPermissionProfile, KeywordAbility, KeywordAbilitySet, ManaCost, SupportedSpellRules,
    },
    crate::domain::play::ids::{CardDefinitionId, CardInstanceId},
    std::sync::Arc,
};

const CREATURE_FLAG_SUMMONING_SICKNESS: u8 = 1 << 0;
const CREATURE_FLAG_ATTACKING: u8 = 1 << 1;
const CREATURE_FLAG_BLOCKING: u8 = 1 << 2;

#[derive(Debug, Clone, PartialEq, Eq)]
struct CreatureRuntime {
    power: u32,
    toughness: u32,
    damage: u32,
    temporary_power: u32,
    temporary_toughness: u32,
    flags: u8,
    blocking_target: Option<CardInstanceId>,
    keywords: KeywordAbilitySet,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct SpellCreatureProfile {
    power: u32,
    toughness: u32,
    keywords: KeywordAbilitySet,
}

impl CreatureRuntime {
    const fn new(power: u32, toughness: u32) -> Self {
        Self {
            power,
            toughness,
            damage: 0,
            temporary_power: 0,
            temporary_toughness: 0,
            flags: CREATURE_FLAG_SUMMONING_SICKNESS,
            blocking_target: None,
            keywords: KeywordAbilitySet::empty(),
        }
    }

    const fn new_with_keywords(power: u32, toughness: u32, keywords: KeywordAbilitySet) -> Self {
        Self {
            power,
            toughness,
            damage: 0,
            temporary_power: 0,
            temporary_toughness: 0,
            flags: CREATURE_FLAG_SUMMONING_SICKNESS,
            blocking_target: None,
            keywords,
        }
    }

    const fn has_summoning_sickness(&self) -> bool {
        self.flags & CREATURE_FLAG_SUMMONING_SICKNESS != 0
    }

    const fn is_attacking(&self) -> bool {
        self.flags & CREATURE_FLAG_ATTACKING != 0
    }

    const fn is_blocking(&self) -> bool {
        self.flags & CREATURE_FLAG_BLOCKING != 0
    }

    const fn remove_summoning_sickness(&mut self) {
        self.flags &= !CREATURE_FLAG_SUMMONING_SICKNESS;
    }

    const fn set_attacking(&mut self, attacking: bool) {
        if attacking {
            self.flags |= CREATURE_FLAG_ATTACKING;
        } else {
            self.flags &= !CREATURE_FLAG_ATTACKING;
        }
    }

    fn set_blocking(&mut self, blocking: bool) {
        if blocking {
            self.flags |= CREATURE_FLAG_BLOCKING;
        } else {
            self.flags &= !CREATURE_FLAG_BLOCKING;
            self.blocking_target = None;
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct CardFace {
    definition: Arc<CardDefinition>,
    card_type: CardType,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct CardRuntime {
    tapped: bool,
    kind: CardRuntimeKind,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum CardRuntimeKind {
    NonCreature,
    Creature(CreatureRuntime),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CardInstance {
    id: CardInstanceId,
    face: CardFace,
    runtime: CardRuntime,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpellCardSnapshot {
    id: CardInstanceId,
    definition: Arc<CardDefinition>,
    card_type: CardType,
    creature_profile: Option<SpellCreatureProfile>,
}

impl CardInstance {
    #[must_use]
    pub(crate) fn from_definition(
        id: CardInstanceId,
        definition: CardDefinition,
        card_type: CardType,
    ) -> Self {
        Self {
            id,
            face: CardFace {
                definition: Arc::new(definition),
                card_type,
            },
            runtime: CardRuntime {
                tapped: false,
                kind: CardRuntimeKind::NonCreature,
            },
        }
    }

    #[must_use]
    pub fn new(
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
    pub fn new_creature(
        id: CardInstanceId,
        definition_id: CardDefinitionId,
        mana_cost: u32,
        power: u32,
        toughness: u32,
    ) -> Self {
        Self {
            id,
            face: CardFace {
                definition: Arc::new(CardDefinition::for_card_type(
                    definition_id,
                    mana_cost,
                    &CardType::Creature,
                )),
                card_type: CardType::Creature,
            },
            runtime: CardRuntime {
                tapped: false,
                kind: CardRuntimeKind::Creature(CreatureRuntime::new(power, toughness)),
            },
        }
    }

    #[must_use]
    pub fn new_creature_with_keywords(
        id: CardInstanceId,
        definition: CardDefinition,
        power: u32,
        toughness: u32,
        keywords: KeywordAbilitySet,
    ) -> Self {
        Self {
            id,
            face: CardFace {
                definition: Arc::new(definition),
                card_type: CardType::Creature,
            },
            runtime: CardRuntime {
                tapped: false,
                kind: CardRuntimeKind::Creature(CreatureRuntime::new_with_keywords(
                    power, toughness, keywords,
                )),
            },
        }
    }

    #[must_use]
    pub fn into_spell_snapshot(self) -> SpellCardSnapshot {
        let creature_profile = match &self.runtime.kind {
            CardRuntimeKind::NonCreature => None,
            CardRuntimeKind::Creature(creature) => Some(SpellCreatureProfile {
                power: creature.power,
                toughness: creature.toughness,
                keywords: creature.keywords,
            }),
        };

        SpellCardSnapshot {
            id: self.id,
            definition: self.face.definition,
            card_type: self.face.card_type,
            creature_profile,
        }
    }

    #[must_use]
    pub const fn id(&self) -> &CardInstanceId {
        &self.id
    }

    #[must_use]
    pub fn definition_id(&self) -> &CardDefinitionId {
        self.face.definition.id()
    }

    #[must_use]
    pub const fn card_type(&self) -> &CardType {
        &self.face.card_type
    }

    #[must_use]
    pub const fn is_tapped(&self) -> bool {
        self.runtime.tapped
    }

    #[must_use]
    pub fn mana_cost(&self) -> u32 {
        self.face.definition.mana_cost()
    }

    #[must_use]
    pub fn mana_cost_profile(&self) -> ManaCost {
        self.face.definition.mana_cost_profile()
    }

    #[must_use]
    pub fn casting_permission_profile(&self) -> Option<CastingPermissionProfile> {
        self.face.definition.casting_permission()
    }

    #[must_use]
    pub fn supported_spell_rules(&self) -> SupportedSpellRules {
        self.face.definition.supported_spell_rules()
    }

    #[must_use]
    pub fn activated_ability(&self) -> Option<ActivatedAbilityProfile> {
        self.face.definition.activated_ability()
    }

    #[must_use]
    pub fn activated_mana_ability(&self) -> Option<ActivatedManaAbilityProfile> {
        self.face.definition.activated_mana_ability()
    }

    #[must_use]
    pub const fn power(&self) -> Option<u32> {
        match &self.runtime.kind {
            CardRuntimeKind::Creature(creature) => Some(creature.power + creature.temporary_power),
            CardRuntimeKind::NonCreature => None,
        }
    }

    #[must_use]
    pub const fn toughness(&self) -> Option<u32> {
        match &self.runtime.kind {
            CardRuntimeKind::Creature(creature) => {
                Some(creature.toughness + creature.temporary_toughness)
            }
            CardRuntimeKind::NonCreature => None,
        }
    }

    #[must_use]
    pub const fn creature_stats(&self) -> Option<(u32, u32)> {
        match &self.runtime.kind {
            CardRuntimeKind::Creature(creature) => Some((
                creature.power + creature.temporary_power,
                creature.toughness + creature.temporary_toughness,
            )),
            CardRuntimeKind::NonCreature => None,
        }
    }

    #[must_use]
    pub const fn has_summoning_sickness(&self) -> bool {
        match &self.runtime.kind {
            CardRuntimeKind::Creature(creature) => creature.has_summoning_sickness(),
            CardRuntimeKind::NonCreature => false,
        }
    }

    #[must_use]
    pub const fn is_attacking(&self) -> bool {
        match &self.runtime.kind {
            CardRuntimeKind::Creature(creature) => creature.is_attacking(),
            CardRuntimeKind::NonCreature => false,
        }
    }

    #[must_use]
    pub const fn is_blocking(&self) -> bool {
        match &self.runtime.kind {
            CardRuntimeKind::Creature(creature) => creature.is_blocking(),
            CardRuntimeKind::NonCreature => false,
        }
    }

    pub const fn tap(&mut self) {
        self.runtime.tapped = true;
    }

    pub const fn untap(&mut self) {
        self.runtime.tapped = false;
    }

    pub const fn remove_summoning_sickness(&mut self) {
        if let CardRuntimeKind::Creature(creature) = &mut self.runtime.kind {
            creature.remove_summoning_sickness();
        }
    }

    pub const fn set_attacking(&mut self, attacking: bool) {
        if let CardRuntimeKind::Creature(creature) = &mut self.runtime.kind {
            creature.set_attacking(attacking);
        }
    }

    pub fn set_blocking(&mut self, blocking: bool) {
        if let CardRuntimeKind::Creature(creature) = &mut self.runtime.kind {
            creature.set_blocking(blocking);
        }
    }

    #[must_use]
    pub const fn blocking_target(&self) -> Option<&CardInstanceId> {
        match &self.runtime.kind {
            CardRuntimeKind::Creature(creature) => creature.blocking_target.as_ref(),
            CardRuntimeKind::NonCreature => None,
        }
    }

    pub fn assign_blocking_target(&mut self, attacker_id: CardInstanceId) {
        if let CardRuntimeKind::Creature(creature) = &mut self.runtime.kind {
            creature.set_blocking(true);
            creature.blocking_target = Some(attacker_id);
        }
    }

    #[must_use]
    pub const fn damage(&self) -> u32 {
        match &self.runtime.kind {
            CardRuntimeKind::Creature(creature) => creature.damage,
            CardRuntimeKind::NonCreature => 0,
        }
    }

    #[must_use]
    pub const fn has_lethal_damage(&self) -> bool {
        match &self.runtime.kind {
            CardRuntimeKind::Creature(creature) => {
                creature.damage >= creature.toughness + creature.temporary_toughness
            }
            CardRuntimeKind::NonCreature => false,
        }
    }

    #[must_use]
    pub const fn has_zero_toughness(&self) -> bool {
        match &self.runtime.kind {
            CardRuntimeKind::Creature(creature) => {
                creature.toughness + creature.temporary_toughness == 0
            }
            CardRuntimeKind::NonCreature => false,
        }
    }

    pub const fn add_damage(&mut self, damage: u32) {
        if let CardRuntimeKind::Creature(creature) = &mut self.runtime.kind {
            creature.damage += damage;
        }
    }

    pub const fn clear_damage(&mut self) {
        if let CardRuntimeKind::Creature(creature) = &mut self.runtime.kind {
            creature.damage = 0;
        }
    }

    pub const fn apply_temporary_stat_bonus(&mut self, power: u32, toughness: u32) {
        if let CardRuntimeKind::Creature(creature) = &mut self.runtime.kind {
            creature.temporary_power += power;
            creature.temporary_toughness += toughness;
        }
    }

    pub const fn clear_temporary_stat_bonuses(&mut self) {
        if let CardRuntimeKind::Creature(creature) = &mut self.runtime.kind {
            creature.temporary_power = 0;
            creature.temporary_toughness = 0;
        }
    }

    #[must_use]
    pub const fn has_keyword(&self, ability: KeywordAbility) -> bool {
        match &self.runtime.kind {
            CardRuntimeKind::Creature(creature) => creature.keywords.contains(ability),
            CardRuntimeKind::NonCreature => false,
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
    pub const fn has_haste(&self) -> bool {
        self.has_keyword(KeywordAbility::Haste)
    }

    #[must_use]
    pub const fn has_vigilance(&self) -> bool {
        self.has_keyword(KeywordAbility::Vigilance)
    }

    #[must_use]
    pub const fn has_trample(&self) -> bool {
        self.has_keyword(KeywordAbility::Trample)
    }

    #[must_use]
    pub const fn has_first_strike(&self) -> bool {
        self.has_keyword(KeywordAbility::FirstStrike)
    }

    #[must_use]
    pub const fn keyword_abilities(&self) -> Option<KeywordAbilitySet> {
        match &self.runtime.kind {
            CardRuntimeKind::Creature(creature) => Some(creature.keywords),
            CardRuntimeKind::NonCreature => None,
        }
    }
}

impl SpellCardSnapshot {
    #[must_use]
    pub const fn id(&self) -> &CardInstanceId {
        &self.id
    }

    #[must_use]
    pub const fn card_type(&self) -> &CardType {
        &self.card_type
    }

    #[must_use]
    pub fn supported_spell_rules(&self) -> SupportedSpellRules {
        self.definition.supported_spell_rules()
    }

    #[must_use]
    pub fn into_card_instance(self) -> CardInstance {
        let runtime = self.creature_profile.map_or(
            CardRuntime {
                tapped: false,
                kind: CardRuntimeKind::NonCreature,
            },
            |creature| CardRuntime {
                tapped: false,
                kind: CardRuntimeKind::Creature(CreatureRuntime::new_with_keywords(
                    creature.power,
                    creature.toughness,
                    creature.keywords,
                )),
            },
        );

        CardInstance {
            id: self.id,
            face: CardFace {
                definition: self.definition,
                card_type: self.card_type,
            },
            runtime,
        }
    }
}
