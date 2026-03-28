//! Supports domain play cards.

mod definition;
mod keywords;
mod limited_set_catalog;
mod rules;
mod runtime;

pub use definition::{CardDefinition, CardDefinitionParts};
pub use keywords::{KeywordAbility, KeywordAbilitySet};
pub use limited_set_catalog::{supported_limited_set_card_profile, SupportedLimitedSetCardProfile};
pub use rules::{
    ActivatedAbilityEffect, ActivatedAbilityProfile, ActivatedAbilitySacrificeCost,
    ActivatedManaAbilityProfile, AttachedCombatRestrictionProfile, AttachedStatBoostProfile,
    AttachmentProfile, CardType, CastingPermissionProfile, CastingRule,
    ControllerStaticEffectProfile, CreatureTargetRule, GraveyardCardTargetRule, ManaColor,
    ManaCost, PlayerTargetRule, SingleTargetRule, SpellResolutionProfile, SpellTargetKind,
    SpellTargetingProfile, SupportedSpellRules, TriggeredAbilityEffect, TriggeredAbilityEvent,
    TriggeredAbilityProfile,
};
pub use runtime::{CardInstance, SpellPayload};
