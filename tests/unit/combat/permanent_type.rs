use demonictutor::domain::play::cards::{
    CardDefinition, CardInstance, CardType, CastingPermissionProfile,
};
use demonictutor::domain::play::ids::{CardDefinitionId, CardInstanceId};

#[test]
fn test_is_permanent_returns_true_for_permanent_types() {
    let land_def =
        CardDefinition::for_card_type(CardDefinitionId::new("test-def"), 0, &CardType::Land);
    let land = CardInstance::new(
        CardInstanceId::new("test-instance"),
        land_def.id().clone(),
        CardType::Land,
        0,
    );
    assert!(land.card_type().is_permanent(), "Land should be permanent");

    // Test Creature
    let creature_def =
        CardDefinition::for_card_type(CardDefinitionId::new("test-def"), 2, &CardType::Creature);
    let creature = CardInstance::new_creature(
        CardInstanceId::new("test-instance"),
        creature_def.id().clone(),
        2,
        2,
        2,
    );
    assert!(
        creature.card_type().is_permanent(),
        "Creature should be permanent"
    );

    // Test Enchantment
    let enchantment_def =
        CardDefinition::for_card_type(CardDefinitionId::new("test-def"), 1, &CardType::Enchantment);
    let enchantment = CardInstance::new(
        CardInstanceId::new("test-instance"),
        enchantment_def.id().clone(),
        CardType::Enchantment,
        1,
    );
    assert!(
        enchantment.card_type().is_permanent(),
        "Enchantment should be permanent"
    );

    // Test Artifact
    let artifact_def =
        CardDefinition::for_card_type(CardDefinitionId::new("test-def"), 3, &CardType::Artifact);
    let artifact = CardInstance::new(
        CardInstanceId::new("test-instance"),
        artifact_def.id().clone(),
        CardType::Artifact,
        3,
    );
    assert!(
        artifact.card_type().is_permanent(),
        "Artifact should be permanent"
    );

    // Test Planeswalker
    let planeswalker_def = CardDefinition::for_card_type(
        CardDefinitionId::new("test-def"),
        4,
        &CardType::Planeswalker,
    );
    let planeswalker = CardInstance::new(
        CardInstanceId::new("test-instance"),
        planeswalker_def.id().clone(),
        CardType::Planeswalker,
        4,
    );
    assert!(
        planeswalker.card_type().is_permanent(),
        "Planeswalker should be permanent"
    );
}

#[test]
fn test_is_permanent_returns_false_for_non_permanent_types() {
    let instant_def =
        CardDefinition::for_card_type(CardDefinitionId::new("test-def"), 1, &CardType::Instant);
    let instant = CardInstance::new(
        CardInstanceId::new("test-instance"),
        instant_def.id().clone(),
        CardType::Instant,
        1,
    );
    assert!(
        !instant.card_type().is_permanent(),
        "Instant should not be permanent"
    );

    // Test Sorcery
    let sorcery_def =
        CardDefinition::for_card_type(CardDefinitionId::new("test-def"), 2, &CardType::Sorcery);
    let sorcery = CardInstance::new(
        CardInstanceId::new("test-instance"),
        sorcery_def.id().clone(),
        CardType::Sorcery,
        2,
    );
    assert!(
        !sorcery.card_type().is_permanent(),
        "Sorcery should not be permanent"
    );
}

#[test]
fn test_casting_permission_profile_for_card_type_matches_supported_model() {
    assert_eq!(
        CastingPermissionProfile::for_spell_card_type(&CardType::Instant),
        Some(CastingPermissionProfile::OpenPriorityWindow)
    );
    assert_eq!(
        CastingPermissionProfile::for_spell_card_type(&CardType::Creature),
        Some(CastingPermissionProfile::ActivePlayerEmptyMainPhaseWindow)
    );
    assert_eq!(
        CastingPermissionProfile::for_spell_card_type(&CardType::Sorcery),
        Some(CastingPermissionProfile::ActivePlayerEmptyMainPhaseWindow)
    );
    assert_eq!(
        CastingPermissionProfile::for_spell_card_type(&CardType::Artifact),
        Some(CastingPermissionProfile::ActivePlayerEmptyMainPhaseWindow)
    );
    assert_eq!(
        CastingPermissionProfile::for_spell_card_type(&CardType::Enchantment),
        Some(CastingPermissionProfile::ActivePlayerEmptyMainPhaseWindow)
    );
    assert_eq!(
        CastingPermissionProfile::for_spell_card_type(&CardType::Planeswalker),
        Some(CastingPermissionProfile::ActivePlayerEmptyMainPhaseWindow)
    );
    assert_eq!(
        CastingPermissionProfile::for_spell_card_type(&CardType::Land),
        None
    );
}
