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
        CardDefinition::for_card_type(CardDefinitionId::new("shock-lite"), 1, &CardType::Instant)
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
        CardDefinition::for_card_type(CardDefinitionId::new("swift-bear"), 2, &CardType::Creature),
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

#[test]
fn plus_one_plus_one_counters_modify_current_creature_stats() {
    let mut card = CardInstance::new_creature(
        CardInstanceId::new("counter-bear"),
        CardDefinitionId::new("counter-bear"),
        2,
        2,
        2,
    );

    assert_eq!(card.creature_stats(), Some((2, 2)));
    card.add_plus_one_plus_one_counters(2);

    assert_eq!(card.power(), Some(4));
    assert_eq!(card.toughness(), Some(4));
    assert_eq!(card.creature_stats(), Some((4, 4)));
    assert!(!card.has_zero_toughness());
}
