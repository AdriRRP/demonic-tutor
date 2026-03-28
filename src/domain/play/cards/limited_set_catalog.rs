//! Supports explicit authoring profiles for the first curated limited set.

use super::{
    ActivatedAbilityProfile, ActivatedManaAbilityProfile, AttachedCombatRestrictionProfile,
    AttachedStatBoostProfile, AttachmentProfile, CardDefinition, CardType, CastingRule,
    ControllerStaticEffectProfile, KeywordAbilitySet, SpellResolutionProfile,
    TriggeredAbilityProfile,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SupportedLimitedSetCardProfile {
    Land {
        mana_ability: ActivatedManaAbilityProfile,
    },
    Creature {
        keyword_abilities: KeywordAbilitySet,
        activated_ability: Option<ActivatedAbilityProfile>,
        triggered_ability: Option<TriggeredAbilityProfile>,
    },
    Spell {
        card_type: CardType,
        resolution: SpellResolutionProfile,
        casting_rules: Vec<CastingRule>,
    },
    Artifact {
        activated_ability: Option<ActivatedAbilityProfile>,
        triggered_ability: Option<TriggeredAbilityProfile>,
        casting_rules: Vec<CastingRule>,
    },
    Enchantment {
        attachment_profile: Option<AttachmentProfile>,
        attached_stat_boost: Option<AttachedStatBoostProfile>,
        attached_combat_restriction: Option<AttachedCombatRestrictionProfile>,
        controller_static_effect: Option<ControllerStaticEffectProfile>,
        casting_rules: Vec<CastingRule>,
    },
    Planeswalker {
        initial_loyalty: u32,
        activated_ability: ActivatedAbilityProfile,
        casting_rules: Vec<CastingRule>,
    },
}

#[derive(Debug, Clone, Copy)]
struct CardAuthoringShape {
    resolution: SpellResolutionProfile,
    has_spell_rules: bool,
    activated_ability: Option<ActivatedAbilityProfile>,
    triggered_ability: Option<TriggeredAbilityProfile>,
    activated_mana_ability: Option<ActivatedManaAbilityProfile>,
    attachment_profile: Option<AttachmentProfile>,
    attached_stat_boost: Option<AttachedStatBoostProfile>,
    attached_combat_restriction: Option<AttachedCombatRestrictionProfile>,
    controller_static_effect: Option<ControllerStaticEffectProfile>,
    initial_loyalty: Option<u32>,
}

fn explicit_casting_rules(definition: &CardDefinition) -> Vec<CastingRule> {
    let Some(permission) = definition.casting_permission() else {
        return Vec::new();
    };

    let mut rules = Vec::new();
    for rule in [
        CastingRule::ActivePlayerEmptyMainPhaseWindow,
        CastingRule::OpenPriorityWindow,
        CastingRule::OpenPriorityWindowDuringOwnTurn,
        CastingRule::CastFromOwnGraveyard,
        CastingRule::ExileOnResolutionWhenCastFromOwnGraveyard,
    ] {
        if permission.supports(rule) {
            rules.push(rule);
        }
    }
    rules
}

const fn card_authoring_shape(definition: &CardDefinition) -> CardAuthoringShape {
    let resolution = definition.supported_spell_rules().resolution();
    CardAuthoringShape {
        resolution,
        has_spell_rules: !matches!(resolution, SpellResolutionProfile::None),
        activated_ability: definition.activated_ability(),
        triggered_ability: definition.triggered_ability(),
        activated_mana_ability: definition.activated_mana_ability(),
        attachment_profile: definition.attachment_profile(),
        attached_stat_boost: definition.attached_stat_boost(),
        attached_combat_restriction: definition.attached_combat_restriction(),
        controller_static_effect: definition.controller_static_effect(),
        initial_loyalty: definition.initial_loyalty(),
    }
}

fn classify_land(
    shape: CardAuthoringShape,
    is_creature: bool,
) -> Option<SupportedLimitedSetCardProfile> {
    let CardAuthoringShape {
        activated_mana_ability: Some(mana_ability),
        has_spell_rules: false,
        activated_ability: None,
        triggered_ability: None,
        attachment_profile: None,
        attached_stat_boost: None,
        attached_combat_restriction: None,
        controller_static_effect: None,
        initial_loyalty: None,
        ..
    } = shape
    else {
        return None;
    };

    (!is_creature).then_some(SupportedLimitedSetCardProfile::Land { mana_ability })
}

fn classify_creature(
    shape: CardAuthoringShape,
    creature_keywords: KeywordAbilitySet,
    is_creature: bool,
) -> Option<SupportedLimitedSetCardProfile> {
    let CardAuthoringShape {
        has_spell_rules: false,
        activated_ability,
        triggered_ability,
        activated_mana_ability: None,
        attachment_profile: None,
        attached_stat_boost: None,
        attached_combat_restriction: None,
        controller_static_effect: None,
        initial_loyalty: None,
        ..
    } = shape
    else {
        return None;
    };

    (is_creature && !(activated_ability.is_some() && triggered_ability.is_some())).then_some(
        SupportedLimitedSetCardProfile::Creature {
            keyword_abilities: creature_keywords,
            activated_ability,
            triggered_ability,
        },
    )
}

fn classify_spell(
    card_type: CardType,
    shape: CardAuthoringShape,
    is_creature: bool,
    casting_rules: Vec<CastingRule>,
) -> Option<SupportedLimitedSetCardProfile> {
    let CardAuthoringShape {
        resolution,
        has_spell_rules: true,
        activated_ability: None,
        triggered_ability: None,
        activated_mana_ability: None,
        attachment_profile: None,
        attached_stat_boost: None,
        attached_combat_restriction: None,
        controller_static_effect: None,
        initial_loyalty: None,
    } = shape
    else {
        return None;
    };

    (!is_creature).then_some(SupportedLimitedSetCardProfile::Spell {
        card_type,
        resolution,
        casting_rules,
    })
}

fn classify_artifact(
    shape: CardAuthoringShape,
    is_creature: bool,
    casting_rules: Vec<CastingRule>,
) -> Option<SupportedLimitedSetCardProfile> {
    let CardAuthoringShape {
        has_spell_rules: false,
        activated_ability,
        triggered_ability,
        activated_mana_ability: None,
        attachment_profile: None,
        attached_stat_boost: None,
        attached_combat_restriction: None,
        controller_static_effect: None,
        initial_loyalty: None,
        ..
    } = shape
    else {
        return None;
    };

    (!(is_creature || activated_ability.is_some() && triggered_ability.is_some())).then_some(
        SupportedLimitedSetCardProfile::Artifact {
            activated_ability,
            triggered_ability,
            casting_rules,
        },
    )
}

fn classify_enchantment(
    shape: CardAuthoringShape,
    is_creature: bool,
    casting_rules: Vec<CastingRule>,
) -> Option<SupportedLimitedSetCardProfile> {
    let CardAuthoringShape {
        resolution,
        has_spell_rules,
        activated_ability: None,
        triggered_ability: None,
        activated_mana_ability: None,
        attachment_profile,
        attached_stat_boost,
        attached_combat_restriction,
        controller_static_effect,
        initial_loyalty: None,
    } = shape
    else {
        return None;
    };

    let is_supported_aura = attachment_profile == Some(AttachmentProfile::EnchantCreature)
        && matches!(resolution, SpellResolutionProfile::AttachToTargetCreature)
        && controller_static_effect.is_none();
    let is_supported_static_enchantment = !has_spell_rules
        && attachment_profile.is_none()
        && attached_stat_boost.is_none()
        && attached_combat_restriction.is_none();

    (!is_creature && (is_supported_aura || is_supported_static_enchantment)).then_some(
        SupportedLimitedSetCardProfile::Enchantment {
            attachment_profile,
            attached_stat_boost,
            attached_combat_restriction,
            controller_static_effect,
            casting_rules,
        },
    )
}

fn classify_planeswalker(
    shape: CardAuthoringShape,
    is_creature: bool,
    casting_rules: Vec<CastingRule>,
) -> Option<SupportedLimitedSetCardProfile> {
    let CardAuthoringShape {
        has_spell_rules: false,
        activated_ability: Some(activated_ability),
        triggered_ability: None,
        activated_mana_ability: None,
        attachment_profile: None,
        attached_stat_boost: None,
        attached_combat_restriction: None,
        controller_static_effect: None,
        initial_loyalty: Some(initial_loyalty),
        ..
    } = shape
    else {
        return None;
    };

    (!is_creature).then_some(SupportedLimitedSetCardProfile::Planeswalker {
        initial_loyalty,
        activated_ability,
        casting_rules,
    })
}

#[must_use]
pub fn supported_limited_set_card_profile(
    definition: &CardDefinition,
    creature_keywords: KeywordAbilitySet,
    is_creature: bool,
) -> Option<SupportedLimitedSetCardProfile> {
    let shape = card_authoring_shape(definition);
    let casting_rules = explicit_casting_rules(definition);

    match definition.card_type() {
        CardType::Land => classify_land(shape, is_creature),
        CardType::Creature => classify_creature(shape, creature_keywords, is_creature),
        CardType::Instant | CardType::Sorcery => {
            classify_spell(*definition.card_type(), shape, is_creature, casting_rules)
        }
        CardType::Artifact => classify_artifact(shape, is_creature, casting_rules),
        CardType::Enchantment => classify_enchantment(shape, is_creature, casting_rules),
        CardType::Planeswalker => classify_planeswalker(shape, is_creature, casting_rules),
    }
}

#[cfg(test)]
mod tests {
    //! Unit coverage for the limited-set authoring profile catalog.

    use crate::domain::play::{
        cards::{
            AttachmentProfile, CardType, ControllerStaticEffectProfile, KeywordAbility,
            KeywordAbilitySet,
        },
        commands::LibraryCard,
        ids::CardDefinitionId,
    };

    use super::{supported_limited_set_card_profile, SupportedLimitedSetCardProfile};

    #[test]
    fn classifies_supported_attack_trigger_creature_with_keywords() {
        let card = LibraryCard::creature_with_keywords(
            CardDefinitionId::new("battle-adept"),
            1,
            2,
            2,
            KeywordAbilitySet::only(KeywordAbility::Haste),
        )
        .with_triggered_ability(
            crate::domain::play::cards::TriggeredAbilityProfile::attacks_gain_life_to_controller(2),
        );

        assert!(matches!(
            card.supported_limited_set_profile(),
            Some(SupportedLimitedSetCardProfile::Creature { triggered_ability: Some(_), keyword_abilities, .. })
            if keyword_abilities.contains(KeywordAbility::Haste)
        ));
    }

    #[test]
    fn classifies_supported_stat_boost_aura_enchantment() {
        let card = LibraryCard::new(
            CardDefinitionId::new("holy-strength"),
            CardType::Enchantment,
            1,
        )
        .with_supported_spell_rules(
            crate::domain::play::cards::SupportedSpellRules::attach_to_target_creature(),
        )
        .with_attachment_profile(AttachmentProfile::EnchantCreature)
        .with_attached_stat_boost(
            crate::domain::play::cards::AttachedStatBoostProfile::plus(1, 2),
        );

        assert!(matches!(
            card.supported_limited_set_profile(),
            Some(SupportedLimitedSetCardProfile::Enchantment {
                attachment_profile: Some(AttachmentProfile::EnchantCreature),
                attached_stat_boost: Some(_),
                ..
            })
        ));
    }

    #[test]
    fn classifies_supported_spell_profile_using_existing_resolution_shape() {
        let card = LibraryCard::new(CardDefinitionId::new("bolt"), CardType::Instant, 1)
            .with_supported_spell_rules(
                crate::domain::play::cards::SupportedSpellRules::deal_damage_to_any_target(3),
            );

        assert!(matches!(
            card.supported_limited_set_profile(),
            Some(SupportedLimitedSetCardProfile::Spell {
                card_type: CardType::Instant,
                ..
            })
        ));
    }

    #[test]
    fn rejects_unsupported_cross_family_card_shape() {
        let card = LibraryCard::creature(CardDefinitionId::new("illegal-creature"), 1, 2, 2)
            .with_supported_spell_rules(
                crate::domain::play::cards::SupportedSpellRules::deal_damage_to_any_target(2),
            );

        assert!(supported_limited_set_card_profile(
            card.definition(),
            card.creature_profile()
                .map_or(KeywordAbilitySet::empty(), |creature| creature
                    .keyword_abilities),
            card.creature_profile().is_some(),
        )
        .is_none());
    }

    #[test]
    fn classifies_supported_controller_anthem_enchantment() {
        let card = LibraryCard::new(CardDefinitionId::new("anthem"), CardType::Enchantment, 3)
            .with_controller_static_effect(
                ControllerStaticEffectProfile::CreaturesYouControlPlusOnePlusOne,
            );

        assert!(matches!(
            card.supported_limited_set_profile(),
            Some(SupportedLimitedSetCardProfile::Enchantment {
                controller_static_effect: Some(
                    ControllerStaticEffectProfile::CreaturesYouControlPlusOnePlusOne
                ),
                ..
            })
        ));
    }
}
