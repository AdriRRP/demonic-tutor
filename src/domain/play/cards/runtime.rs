//! Supports play cards runtime.

use {
    super::{
        ActivatedAbilityProfile, ActivatedManaAbilityProfile, CardDefinition, CardType,
        CastingPermissionProfile, KeywordAbility, KeywordAbilitySet, ManaCost, SupportedSpellRules,
    },
    crate::domain::play::ids::{CardDefinitionId, CardInstanceId, PlayerCardHandle},
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
    blocking_target: Option<PlayerCardHandle>,
    keywords: KeywordAbilitySet,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CreatureSpellPayload {
    id: CardInstanceId,
    definition_id: CardDefinitionId,
    power: u32,
    toughness: u32,
    keywords: KeywordAbilitySet,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PermanentSpellPayload {
    id: CardInstanceId,
    definition_id: CardDefinitionId,
    activated_ability: Option<ActivatedAbilityProfile>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EffectSpellPayload {
    id: CardInstanceId,
    definition_id: CardDefinitionId,
    supported_spell_rules: SupportedSpellRules,
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

    const fn set_blocking(&mut self, blocking: bool) {
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
pub enum SpellPayload {
    Instant(EffectSpellPayload),
    Sorcery(EffectSpellPayload),
    Artifact(PermanentSpellPayload),
    Enchantment(PermanentSpellPayload),
    Planeswalker(PermanentSpellPayload),
    Land(PermanentSpellPayload),
    Creature(CreatureSpellPayload),
}

impl CardInstance {
    #[must_use]
    pub(crate) fn from_definition(
        id: CardInstanceId,
        definition: CardDefinition,
        _card_type: CardType,
    ) -> Self {
        Self {
            id,
            face: CardFace {
                definition: Arc::new(definition),
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
    pub fn into_spell_payload(self) -> SpellPayload {
        match &self.runtime.kind {
            CardRuntimeKind::NonCreature => {
                let definition = self.face.definition.as_ref();
                match definition.card_type() {
                    CardType::Artifact => SpellPayload::Artifact(PermanentSpellPayload {
                        id: self.id,
                        definition_id: definition.id().clone(),
                        activated_ability: definition.activated_ability(),
                    }),
                    CardType::Enchantment => SpellPayload::Enchantment(PermanentSpellPayload {
                        id: self.id,
                        definition_id: definition.id().clone(),
                        activated_ability: definition.activated_ability(),
                    }),
                    CardType::Planeswalker => SpellPayload::Planeswalker(PermanentSpellPayload {
                        id: self.id,
                        definition_id: definition.id().clone(),
                        activated_ability: definition.activated_ability(),
                    }),
                    CardType::Land => SpellPayload::Land(PermanentSpellPayload {
                        id: self.id,
                        definition_id: definition.id().clone(),
                        activated_ability: definition.activated_ability(),
                    }),
                    CardType::Instant => SpellPayload::Instant(EffectSpellPayload {
                        id: self.id,
                        definition_id: definition.id().clone(),
                        supported_spell_rules: definition.supported_spell_rules(),
                    }),
                    CardType::Sorcery => SpellPayload::Sorcery(EffectSpellPayload {
                        id: self.id,
                        definition_id: definition.id().clone(),
                        supported_spell_rules: definition.supported_spell_rules(),
                    }),
                    CardType::Creature => {
                        debug_assert!(
                            false,
                            "non-creature spell payloads should never be built from creature definitions"
                        );
                        SpellPayload::Land(PermanentSpellPayload {
                            id: self.id,
                            definition_id: definition.id().clone(),
                            activated_ability: definition.activated_ability(),
                        })
                    }
                }
            }
            CardRuntimeKind::Creature(creature) => SpellPayload::Creature(CreatureSpellPayload {
                id: self.id,
                definition_id: self.face.definition.id().clone(),
                power: creature.power,
                toughness: creature.toughness,
                keywords: creature.keywords,
            }),
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
    pub fn card_type(&self) -> &CardType {
        self.face.definition.card_type()
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

    pub const fn set_blocking(&mut self, blocking: bool) {
        if let CardRuntimeKind::Creature(creature) = &mut self.runtime.kind {
            creature.set_blocking(blocking);
        }
    }

    #[must_use]
    pub const fn blocking_target(&self) -> Option<PlayerCardHandle> {
        match &self.runtime.kind {
            CardRuntimeKind::Creature(creature) => creature.blocking_target,
            CardRuntimeKind::NonCreature => None,
        }
    }

    pub const fn assign_blocking_target(&mut self, attacker_handle: PlayerCardHandle) {
        if let CardRuntimeKind::Creature(creature) = &mut self.runtime.kind {
            creature.set_blocking(true);
            creature.blocking_target = Some(attacker_handle);
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

impl SpellPayload {
    fn effect_into_card_instance(payload: EffectSpellPayload, card_type: CardType) -> CardInstance {
        CardInstance {
            id: payload.id,
            face: CardFace {
                definition: Arc::new(
                    CardDefinition::for_card_type(payload.definition_id, 0, &card_type)
                        .with_supported_spell_rules(payload.supported_spell_rules),
                ),
            },
            runtime: CardRuntime {
                tapped: false,
                kind: CardRuntimeKind::NonCreature,
            },
        }
    }

    fn permanent_into_card_instance(
        payload: PermanentSpellPayload,
        card_type: CardType,
    ) -> CardInstance {
        let mut definition = CardDefinition::for_card_type(payload.definition_id, 0, &card_type);
        if let Some(activated_ability) = payload.activated_ability {
            definition = definition.with_activated_ability(activated_ability);
        }
        CardInstance {
            id: payload.id,
            face: CardFace {
                definition: Arc::new(definition),
            },
            runtime: CardRuntime {
                tapped: false,
                kind: CardRuntimeKind::NonCreature,
            },
        }
    }

    #[must_use]
    pub const fn id(&self) -> &CardInstanceId {
        match self {
            Self::Instant(payload) | Self::Sorcery(payload) => &payload.id,
            Self::Artifact(payload)
            | Self::Enchantment(payload)
            | Self::Planeswalker(payload)
            | Self::Land(payload) => &payload.id,
            Self::Creature(payload) => &payload.id,
        }
    }

    #[must_use]
    pub const fn card_type(&self) -> &CardType {
        match self {
            Self::Instant(_) => &CardType::Instant,
            Self::Sorcery(_) => &CardType::Sorcery,
            Self::Artifact(_) => &CardType::Artifact,
            Self::Enchantment(_) => &CardType::Enchantment,
            Self::Planeswalker(_) => &CardType::Planeswalker,
            Self::Land(_) => &CardType::Land,
            Self::Creature(_) => &CardType::Creature,
        }
    }

    #[must_use]
    pub const fn supported_spell_rules(&self) -> SupportedSpellRules {
        match self {
            Self::Instant(payload) | Self::Sorcery(payload) => payload.supported_spell_rules,
            Self::Artifact(_)
            | Self::Enchantment(_)
            | Self::Planeswalker(_)
            | Self::Land(_)
            | Self::Creature(_) => SupportedSpellRules::none(),
        }
    }

    #[must_use]
    pub fn into_card_instance(self) -> CardInstance {
        match self {
            Self::Instant(payload) => Self::effect_into_card_instance(payload, CardType::Instant),
            Self::Sorcery(payload) => Self::effect_into_card_instance(payload, CardType::Sorcery),
            Self::Artifact(payload) => {
                Self::permanent_into_card_instance(payload, CardType::Artifact)
            }
            Self::Enchantment(payload) => {
                Self::permanent_into_card_instance(payload, CardType::Enchantment)
            }
            Self::Planeswalker(payload) => {
                Self::permanent_into_card_instance(payload, CardType::Planeswalker)
            }
            Self::Land(payload) => Self::permanent_into_card_instance(payload, CardType::Land),
            Self::Creature(payload) => CardInstance {
                id: payload.id,
                face: CardFace {
                    definition: Arc::new(CardDefinition::for_card_type(
                        payload.definition_id,
                        0,
                        &CardType::Creature,
                    )),
                },
                runtime: CardRuntime {
                    tapped: false,
                    kind: CardRuntimeKind::Creature(CreatureRuntime::new_with_keywords(
                        payload.power,
                        payload.toughness,
                        payload.keywords,
                    )),
                },
            },
        }
    }
}

#[cfg(test)]
mod tests {
    //! Supports runtime payload regression tests.

    use super::{
        ActivatedAbilityProfile, CardDefinition, CardDefinitionId, CardInstance, CardInstanceId,
        CardType, KeywordAbility, KeywordAbilitySet, SpellPayload, SupportedSpellRules,
    };

    #[test]
    fn permanent_payload_round_trips_relevant_battlefield_traits_without_static_mana_metadata() {
        let card = CardInstance::from_definition(
            CardInstanceId::new("ivory-cup-on-stack"),
            CardDefinition::for_card_type(
                CardDefinitionId::new("ivory-cup-lite"),
                3,
                &CardType::Artifact,
            )
            .with_activated_ability(ActivatedAbilityProfile::tap_to_gain_life_to_controller(1)),
            CardType::Artifact,
        );

        assert!(matches!(
            card.clone().into_spell_payload(),
            SpellPayload::Artifact(_)
        ));
        let SpellPayload::Artifact(payload) = card.into_spell_payload() else {
            return;
        };
        let resolved = SpellPayload::Artifact(payload).into_card_instance();

        assert_eq!(resolved.card_type(), &CardType::Artifact);
        assert_eq!(
            resolved.definition_id(),
            &CardDefinitionId::new("ivory-cup-lite")
        );
        assert_eq!(
            resolved.activated_ability(),
            Some(ActivatedAbilityProfile::tap_to_gain_life_to_controller(1))
        );
        assert_eq!(resolved.mana_cost(), 0);
    }

    #[test]
    fn effect_payload_round_trips_resolution_rules_without_static_mana_metadata() {
        let card = CardInstance::from_definition(
            CardInstanceId::new("shock-on-stack"),
            CardDefinition::for_card_type(
                CardDefinitionId::new("shock-lite"),
                1,
                &CardType::Instant,
            )
            .with_supported_spell_rules(SupportedSpellRules::deal_damage_to_any_target(2)),
            CardType::Instant,
        );

        assert!(matches!(
            card.clone().into_spell_payload(),
            SpellPayload::Instant(_)
        ));
        let SpellPayload::Instant(payload) = card.into_spell_payload() else {
            return;
        };
        let resolved = SpellPayload::Instant(payload).into_card_instance();

        assert_eq!(resolved.card_type(), &CardType::Instant);
        assert_eq!(
            resolved.definition_id(),
            &CardDefinitionId::new("shock-lite")
        );
        assert_eq!(
            resolved.supported_spell_rules(),
            SupportedSpellRules::deal_damage_to_any_target(2)
        );
        assert_eq!(resolved.mana_cost(), 0);
    }

    #[test]
    fn creature_payload_round_trips_stats_and_keywords_without_static_mana_metadata() {
        let card = CardInstance::new_creature_with_keywords(
            CardInstanceId::new("bear-on-stack"),
            CardDefinition::for_card_type(
                CardDefinitionId::new("swift-bear"),
                2,
                &CardType::Creature,
            ),
            2,
            2,
            KeywordAbilitySet::only(KeywordAbility::Haste).with(KeywordAbility::Trample),
        );

        assert!(matches!(
            card.clone().into_spell_payload(),
            SpellPayload::Creature(_)
        ));
        let SpellPayload::Creature(payload) = card.into_spell_payload() else {
            return;
        };
        let resolved = SpellPayload::Creature(payload).into_card_instance();

        assert_eq!(resolved.card_type(), &CardType::Creature);
        assert_eq!(resolved.creature_stats(), Some((2, 2)));
        assert!(resolved.has_haste());
        assert!(resolved.has_trample());
        assert_eq!(resolved.mana_cost(), 0);
    }
}
