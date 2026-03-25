//! Supports play cards runtime.

use {
    super::{
        ActivatedAbilityProfile, ActivatedManaAbilityProfile, CardDefinition, CardType,
        CastingPermissionProfile, KeywordAbility, KeywordAbilitySet, ManaCost, SupportedSpellRules,
        TriggeredAbilityProfile,
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
    deathtouch_damage: bool,
    temporary_power: u32,
    temporary_toughness: u32,
    flags: u8,
    blocking_target: Option<PlayerCardHandle>,
    blocked_by: Vec<PlayerCardHandle>,
    keywords: KeywordAbilitySet,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CreatureSpellPayload {
    power: u32,
    toughness: u32,
    keywords: KeywordAbilitySet,
    triggered_ability: Option<TriggeredAbilityProfile>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PermanentSpellPayload {
    activated_ability: Option<ActivatedAbilityProfile>,
    triggered_ability: Option<TriggeredAbilityProfile>,
    initial_loyalty: Option<u32>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EffectSpellPayload {
    supported_spell_rules: SupportedSpellRules,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SpellPayloadKind {
    Instant(EffectSpellPayload),
    Sorcery(EffectSpellPayload),
    Artifact(PermanentSpellPayload),
    Enchantment(PermanentSpellPayload),
    Planeswalker(PermanentSpellPayload),
    Land(PermanentSpellPayload),
    Creature(CreatureSpellPayload),
}

impl CreatureRuntime {
    const fn new(power: u32, toughness: u32) -> Self {
        Self {
            power,
            toughness,
            damage: 0,
            deathtouch_damage: false,
            temporary_power: 0,
            temporary_toughness: 0,
            flags: CREATURE_FLAG_SUMMONING_SICKNESS,
            blocking_target: None,
            blocked_by: Vec::new(),
            keywords: KeywordAbilitySet::empty(),
        }
    }

    const fn new_with_keywords(power: u32, toughness: u32, keywords: KeywordAbilitySet) -> Self {
        Self {
            power,
            toughness,
            damage: 0,
            deathtouch_damage: false,
            temporary_power: 0,
            temporary_toughness: 0,
            flags: CREATURE_FLAG_SUMMONING_SICKNESS,
            blocking_target: None,
            blocked_by: Vec::new(),
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

    fn set_attacking(&mut self, attacking: bool) {
        if attacking {
            self.flags |= CREATURE_FLAG_ATTACKING;
        } else {
            self.flags &= !CREATURE_FLAG_ATTACKING;
            self.blocked_by.clear();
        }
    }

    fn set_blocking(&mut self, blocking: bool) {
        if blocking {
            self.flags |= CREATURE_FLAG_BLOCKING;
        } else {
            self.flags &= !CREATURE_FLAG_BLOCKING;
            self.blocking_target = None;
            self.blocked_by.clear();
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
    loyalty: u32,
    loyalty_ability_activated_this_turn: bool,
    is_token: bool,
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
pub struct SpellPayload {
    id: CardInstanceId,
    definition_id: CardDefinitionId,
    kind: SpellPayloadKind,
}

impl CardInstance {
    #[must_use]
    pub const fn id(&self) -> &CardInstanceId {
        &self.id
    }

    #[must_use]
    pub(crate) fn from_definition(
        id: CardInstanceId,
        definition: CardDefinition,
        _card_type: CardType,
    ) -> Self {
        let loyalty = definition.initial_loyalty().unwrap_or(0);
        Self {
            id,
            face: CardFace {
                definition: Arc::new(definition),
            },
            runtime: CardRuntime {
                tapped: false,
                loyalty,
                loyalty_ability_activated_this_turn: false,
                is_token: false,
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
                loyalty: 0,
                loyalty_ability_activated_this_turn: false,
                is_token: false,
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
                loyalty: 0,
                loyalty_ability_activated_this_turn: false,
                is_token: false,
                kind: CardRuntimeKind::Creature(CreatureRuntime::new_with_keywords(
                    power, toughness, keywords,
                )),
            },
        }
    }

    #[must_use]
    pub fn new_vanilla_creature_token(
        id: CardInstanceId,
        definition_id: CardDefinitionId,
        power: u32,
        toughness: u32,
    ) -> Self {
        Self {
            id,
            face: CardFace {
                definition: Arc::new(CardDefinition::for_card_type(
                    definition_id,
                    0,
                    &CardType::Creature,
                )),
            },
            runtime: CardRuntime {
                tapped: false,
                loyalty: 0,
                loyalty_ability_activated_this_turn: false,
                is_token: true,
                kind: CardRuntimeKind::Creature(CreatureRuntime::new(power, toughness)),
            },
        }
    }

    #[must_use]
    pub fn into_spell_payload(self) -> SpellPayload {
        let definition_id = self.face.definition.id().clone();
        match &self.runtime.kind {
            CardRuntimeKind::NonCreature => {
                let definition = self.face.definition.as_ref();
                match definition.card_type() {
                    CardType::Artifact => SpellPayload {
                        id: self.id,
                        definition_id,
                        kind: SpellPayloadKind::Artifact(PermanentSpellPayload {
                            activated_ability: definition.activated_ability(),
                            triggered_ability: definition.triggered_ability(),
                            initial_loyalty: definition.initial_loyalty(),
                        }),
                    },
                    CardType::Enchantment => SpellPayload {
                        id: self.id,
                        definition_id,
                        kind: SpellPayloadKind::Enchantment(PermanentSpellPayload {
                            activated_ability: definition.activated_ability(),
                            triggered_ability: definition.triggered_ability(),
                            initial_loyalty: definition.initial_loyalty(),
                        }),
                    },
                    CardType::Planeswalker => SpellPayload {
                        id: self.id,
                        definition_id,
                        kind: SpellPayloadKind::Planeswalker(PermanentSpellPayload {
                            activated_ability: definition.activated_ability(),
                            triggered_ability: definition.triggered_ability(),
                            initial_loyalty: definition.initial_loyalty(),
                        }),
                    },
                    CardType::Land => SpellPayload {
                        id: self.id,
                        definition_id,
                        kind: SpellPayloadKind::Land(PermanentSpellPayload {
                            activated_ability: definition.activated_ability(),
                            triggered_ability: definition.triggered_ability(),
                            initial_loyalty: definition.initial_loyalty(),
                        }),
                    },
                    CardType::Instant => SpellPayload {
                        id: self.id,
                        definition_id,
                        kind: SpellPayloadKind::Instant(EffectSpellPayload {
                            supported_spell_rules: definition.supported_spell_rules(),
                        }),
                    },
                    CardType::Sorcery => SpellPayload {
                        id: self.id,
                        definition_id,
                        kind: SpellPayloadKind::Sorcery(EffectSpellPayload {
                            supported_spell_rules: definition.supported_spell_rules(),
                        }),
                    },
                    CardType::Creature => {
                        debug_assert!(
                            false,
                            "non-creature spell payloads should never be built from creature definitions"
                        );
                        SpellPayload {
                            id: self.id,
                            definition_id,
                            kind: SpellPayloadKind::Land(PermanentSpellPayload {
                                activated_ability: definition.activated_ability(),
                                triggered_ability: definition.triggered_ability(),
                                initial_loyalty: definition.initial_loyalty(),
                            }),
                        }
                    }
                }
            }
            CardRuntimeKind::Creature(creature) => SpellPayload {
                id: self.id,
                definition_id,
                kind: SpellPayloadKind::Creature(CreatureSpellPayload {
                    power: creature.power,
                    toughness: creature.toughness,
                    keywords: creature.keywords,
                    triggered_ability: self.face.definition.triggered_ability(),
                }),
            },
        }
    }
}

impl SpellPayload {
    #[must_use]
    pub const fn id(&self) -> &CardInstanceId {
        &self.id
    }

    #[must_use]
    pub const fn definition_id(&self) -> &CardDefinitionId {
        &self.definition_id
    }

    #[must_use]
    pub const fn kind(&self) -> &SpellPayloadKind {
        &self.kind
    }

    fn effect_into_card_instance(
        id: CardInstanceId,
        definition_id: CardDefinitionId,
        payload: &EffectSpellPayload,
        card_type: CardType,
    ) -> CardInstance {
        CardInstance {
            id,
            face: CardFace {
                definition: Arc::new(
                    CardDefinition::for_card_type(definition_id, 0, &card_type)
                        .with_supported_spell_rules(payload.supported_spell_rules),
                ),
            },
            runtime: CardRuntime {
                tapped: false,
                loyalty: 0,
                loyalty_ability_activated_this_turn: false,
                is_token: false,
                kind: CardRuntimeKind::NonCreature,
            },
        }
    }

    fn permanent_into_card_instance(
        id: CardInstanceId,
        definition_id: CardDefinitionId,
        payload: &PermanentSpellPayload,
        card_type: CardType,
    ) -> CardInstance {
        let mut definition = CardDefinition::for_card_type(definition_id, 0, &card_type);
        if let Some(activated_ability) = payload.activated_ability {
            definition = definition.with_activated_ability(activated_ability);
        }
        if let Some(triggered_ability) = payload.triggered_ability {
            definition = definition.with_triggered_ability(triggered_ability);
        }
        if let Some(initial_loyalty) = payload.initial_loyalty {
            definition = definition.with_initial_loyalty(initial_loyalty);
        }
        CardInstance {
            id,
            face: CardFace {
                definition: Arc::new(definition),
            },
            runtime: CardRuntime {
                tapped: false,
                loyalty: payload.initial_loyalty.unwrap_or(0),
                loyalty_ability_activated_this_turn: false,
                is_token: false,
                kind: CardRuntimeKind::NonCreature,
            },
        }
    }

    #[must_use]
    pub const fn card_type(&self) -> &CardType {
        match self.kind() {
            SpellPayloadKind::Instant(_) => &CardType::Instant,
            SpellPayloadKind::Sorcery(_) => &CardType::Sorcery,
            SpellPayloadKind::Artifact(_) => &CardType::Artifact,
            SpellPayloadKind::Enchantment(_) => &CardType::Enchantment,
            SpellPayloadKind::Planeswalker(_) => &CardType::Planeswalker,
            SpellPayloadKind::Land(_) => &CardType::Land,
            SpellPayloadKind::Creature(_) => &CardType::Creature,
        }
    }

    #[must_use]
    pub const fn supported_spell_rules(&self) -> SupportedSpellRules {
        match self.kind() {
            SpellPayloadKind::Instant(payload) | SpellPayloadKind::Sorcery(payload) => {
                payload.supported_spell_rules
            }
            SpellPayloadKind::Artifact(_)
            | SpellPayloadKind::Enchantment(_)
            | SpellPayloadKind::Planeswalker(_)
            | SpellPayloadKind::Land(_)
            | SpellPayloadKind::Creature(_) => SupportedSpellRules::none(),
        }
    }

    #[must_use]
    pub fn into_card_instance(self) -> CardInstance {
        let Self {
            id,
            definition_id,
            kind,
        } = self;

        match kind {
            SpellPayloadKind::Instant(payload) => {
                Self::effect_into_card_instance(id, definition_id, &payload, CardType::Instant)
            }
            SpellPayloadKind::Sorcery(payload) => {
                Self::effect_into_card_instance(id, definition_id, &payload, CardType::Sorcery)
            }
            SpellPayloadKind::Artifact(payload) => {
                Self::permanent_into_card_instance(id, definition_id, &payload, CardType::Artifact)
            }
            SpellPayloadKind::Enchantment(payload) => Self::permanent_into_card_instance(
                id,
                definition_id,
                &payload,
                CardType::Enchantment,
            ),
            SpellPayloadKind::Planeswalker(payload) => Self::permanent_into_card_instance(
                id,
                definition_id,
                &payload,
                CardType::Planeswalker,
            ),
            SpellPayloadKind::Land(payload) => {
                Self::permanent_into_card_instance(id, definition_id, &payload, CardType::Land)
            }
            SpellPayloadKind::Creature(payload) => CardInstance {
                id,
                face: CardFace {
                    definition: Arc::new({
                        let mut definition =
                            CardDefinition::for_card_type(definition_id, 0, &CardType::Creature);
                        if let Some(triggered_ability) = payload.triggered_ability {
                            definition = definition.with_triggered_ability(triggered_ability);
                        }
                        definition
                    }),
                },
                runtime: CardRuntime {
                    tapped: false,
                    loyalty: 0,
                    loyalty_ability_activated_this_turn: false,
                    is_token: false,
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

impl CardInstance {
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
    pub fn loyalty(&self) -> Option<u32> {
        matches!(self.card_type(), CardType::Planeswalker).then_some(self.runtime.loyalty)
    }

    #[must_use]
    pub fn loyalty_ability_activated_this_turn(&self) -> bool {
        matches!(self.card_type(), CardType::Planeswalker)
            && self.runtime.loyalty_ability_activated_this_turn
    }

    #[must_use]
    pub const fn is_token(&self) -> bool {
        self.runtime.is_token
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
    pub fn triggered_ability(&self) -> Option<TriggeredAbilityProfile> {
        self.face.definition.triggered_ability()
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

    pub fn adjust_loyalty(&mut self, delta: i32) -> bool {
        if !matches!(self.card_type(), CardType::Planeswalker) {
            return false;
        }
        if delta.is_negative() {
            let amount = delta.unsigned_abs();
            if self.runtime.loyalty < amount {
                return false;
            }
            self.runtime.loyalty -= amount;
        } else {
            self.runtime.loyalty = self.runtime.loyalty.saturating_add(delta.cast_unsigned());
        }
        true
    }

    pub fn mark_loyalty_ability_activated(&mut self) -> bool {
        if !matches!(self.card_type(), CardType::Planeswalker) {
            return false;
        }
        if self.runtime.loyalty_ability_activated_this_turn {
            return false;
        }
        self.runtime.loyalty_ability_activated_this_turn = true;
        true
    }

    pub fn reset_loyalty_activation_for_new_turn(&mut self) {
        if matches!(self.card_type(), CardType::Planeswalker) {
            self.runtime.loyalty_ability_activated_this_turn = false;
        }
    }

    pub const fn remove_summoning_sickness(&mut self) {
        if let CardRuntimeKind::Creature(creature) = &mut self.runtime.kind {
            creature.remove_summoning_sickness();
        }
    }

    pub fn set_attacking(&mut self, attacking: bool) {
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
    pub const fn blocking_target(&self) -> Option<PlayerCardHandle> {
        match &self.runtime.kind {
            CardRuntimeKind::Creature(creature) => creature.blocking_target,
            CardRuntimeKind::NonCreature => None,
        }
    }

    pub fn assign_blocking_target(&mut self, attacker_handle: PlayerCardHandle) {
        if let CardRuntimeKind::Creature(creature) = &mut self.runtime.kind {
            creature.set_blocking(true);
            creature.blocking_target = Some(attacker_handle);
        }
    }

    pub fn add_blocker(&mut self, blocker_handle: PlayerCardHandle) {
        if let CardRuntimeKind::Creature(creature) = &mut self.runtime.kind {
            if !creature.blocked_by.contains(&blocker_handle) {
                creature.blocked_by.push(blocker_handle);
            }
        }
    }

    #[must_use]
    pub const fn blocked_by(&self) -> &[PlayerCardHandle] {
        match &self.runtime.kind {
            CardRuntimeKind::Creature(creature) => creature.blocked_by.as_slice(),
            CardRuntimeKind::NonCreature => &[],
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
                    || (creature.deathtouch_damage && creature.damage > 0)
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

    pub const fn add_deathtouch_damage(&mut self, damage: u32) {
        if let CardRuntimeKind::Creature(creature) = &mut self.runtime.kind {
            creature.damage += damage;
            if damage > 0 {
                creature.deathtouch_damage = true;
            }
        }
    }

    pub const fn clear_damage(&mut self) {
        if let CardRuntimeKind::Creature(creature) = &mut self.runtime.kind {
            creature.damage = 0;
            creature.deathtouch_damage = false;
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
    pub const fn has_deathtouch(&self) -> bool {
        self.has_keyword(KeywordAbility::Deathtouch)
    }

    #[must_use]
    pub const fn has_double_strike(&self) -> bool {
        self.has_keyword(KeywordAbility::DoubleStrike)
    }

    #[must_use]
    pub const fn has_lifelink(&self) -> bool {
        self.has_keyword(KeywordAbility::Lifelink)
    }

    #[must_use]
    pub const fn has_menace(&self) -> bool {
        self.has_keyword(KeywordAbility::Menace)
    }

    #[must_use]
    pub const fn has_hexproof(&self) -> bool {
        self.has_keyword(KeywordAbility::Hexproof)
    }

    #[must_use]
    pub const fn has_indestructible(&self) -> bool {
        self.has_keyword(KeywordAbility::Indestructible)
    }

    #[must_use]
    pub const fn keyword_abilities(&self) -> Option<KeywordAbilitySet> {
        match &self.runtime.kind {
            CardRuntimeKind::Creature(creature) => Some(creature.keywords),
            CardRuntimeKind::NonCreature => None,
        }
    }
}

#[cfg(test)]
mod tests {
    //! Supports runtime payload regression tests.

    use super::{
        ActivatedAbilityProfile, CardDefinition, CardDefinitionId, CardInstance, CardInstanceId,
        CardType, KeywordAbility, KeywordAbilitySet, SpellPayloadKind, SupportedSpellRules,
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
            card.clone().into_spell_payload().kind(),
            SpellPayloadKind::Artifact(_)
        ));
        let payload = card.into_spell_payload();
        let resolved = payload.into_card_instance();

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
            card.clone().into_spell_payload().kind(),
            SpellPayloadKind::Instant(_)
        ));
        let payload = card.into_spell_payload();
        let resolved = payload.into_card_instance();

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
            card.clone().into_spell_payload().kind(),
            SpellPayloadKind::Creature(_)
        ));
        let payload = card.into_spell_payload();
        let resolved = payload.into_card_instance();

        assert_eq!(resolved.card_type(), &CardType::Creature);
        assert_eq!(resolved.creature_stats(), Some((2, 2)));
        assert!(resolved.has_haste());
        assert!(resolved.has_trample());
        assert_eq!(resolved.mana_cost(), 0);
    }
}
