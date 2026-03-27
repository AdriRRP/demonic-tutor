//! Supports play cards runtime.

mod card_instance;
mod spell_payload;

#[cfg(test)]
mod tests;

use {
    super::{
        ActivatedAbilityProfile, ActivatedManaAbilityProfile, AttachedCombatRestrictionProfile,
        AttachedStatBoostProfile, AttachmentProfile, CardDefinition, CardType,
        CastingPermissionProfile, CastingRule, ControllerStaticEffectProfile, KeywordAbility,
        KeywordAbilitySet, ManaCost, SupportedSpellRules, TriggeredAbilityProfile,
    },
    crate::domain::play::ids::{CardDefinitionId, CardInstanceId, PlayerCardHandle, PlayerId},
    std::sync::Arc,
};

const CREATURE_FLAG_SUMMONING_SICKNESS: u8 = 1 << 0;
const CREATURE_FLAG_ATTACKING: u8 = 1 << 1;
const CREATURE_FLAG_BLOCKING: u8 = 1 << 2;

#[derive(Debug, Clone, PartialEq, Eq)]
struct CreatureRuntime {
    power: u32,
    toughness: u32,
    plus_one_plus_one_counters: u32,
    damage: u32,
    deathtouch_damage: bool,
    temporary_power: u32,
    temporary_toughness: u32,
    attached_power_bonus: u32,
    attached_toughness_bonus: u32,
    controller_static_power_bonus: u32,
    controller_static_toughness_bonus: u32,
    attached_cant_attack_count: u32,
    attached_cant_block_count: u32,
    temporary_cant_block_count: u32,
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
    activated_ability: Option<ActivatedAbilityProfile>,
    triggered_ability: Option<TriggeredAbilityProfile>,
    controller_static_effect: Option<ControllerStaticEffectProfile>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PermanentSpellPayload {
    supported_spell_rules: SupportedSpellRules,
    activated_ability: Option<ActivatedAbilityProfile>,
    triggered_ability: Option<TriggeredAbilityProfile>,
    initial_loyalty: Option<u32>,
    attachment_profile: Option<AttachmentProfile>,
    attached_stat_boost: Option<AttachedStatBoostProfile>,
    attached_combat_restriction: Option<AttachedCombatRestrictionProfile>,
    controller_static_effect: Option<ControllerStaticEffectProfile>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EffectSpellPayload {
    casting_permission: Option<CastingPermissionProfile>,
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
            plus_one_plus_one_counters: 0,
            damage: 0,
            deathtouch_damage: false,
            temporary_power: 0,
            temporary_toughness: 0,
            attached_power_bonus: 0,
            attached_toughness_bonus: 0,
            controller_static_power_bonus: 0,
            controller_static_toughness_bonus: 0,
            attached_cant_attack_count: 0,
            attached_cant_block_count: 0,
            temporary_cant_block_count: 0,
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
            plus_one_plus_one_counters: 0,
            damage: 0,
            deathtouch_damage: false,
            temporary_power: 0,
            temporary_toughness: 0,
            attached_power_bonus: 0,
            attached_toughness_bonus: 0,
            controller_static_power_bonus: 0,
            controller_static_toughness_bonus: 0,
            attached_cant_attack_count: 0,
            attached_cant_block_count: 0,
            temporary_cant_block_count: 0,
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
    attached_to: Option<CardInstanceId>,
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
    owner_id: Option<PlayerId>,
    face: CardFace,
    runtime: CardRuntime,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpellPayload {
    id: CardInstanceId,
    owner_id: Option<PlayerId>,
    definition_id: CardDefinitionId,
    kind: SpellPayloadKind,
}
