#![allow(clippy::panic)]
#![allow(clippy::unwrap_used)]

//! Tests bounded counter-placement effects.

use {
    crate::support::{
        advance_to_first_main_satisfying_cleanup, cast_spell_and_resolve, creature_card,
        distribute_two_counters_among_up_to_two_target_creatures_sorcery_card, filled_library,
        player, put_counter_on_target_creature_sorcery_card, self_growing_creature_card,
        setup_two_player_game, targeted_damage_instant_card, vanilla_creature,
    },
    demonictutor::{
        ActivateAbilityCommand, CardDefinitionId, CastSpellCommand, PassPriorityCommand, PlayerId,
        SpellChoice, SpellTarget,
    },
};

#[test]
fn supported_spell_can_put_a_plus_one_counter_on_target_creature() {
    let (service, mut game) = setup_two_player_game(
        "game-counter-target-creature",
        filled_library(
            vec![
                vanilla_creature("bear"),
                put_counter_on_target_creature_sorcery_card("blessing", 0),
            ],
            10,
        ),
        filled_library(Vec::new(), 10),
    );
    advance_to_first_main_satisfying_cleanup(&service, &mut game);

    let creature_id = player(&game, "player-1")
        .hand_card_by_definition(&demonictutor::CardDefinitionId::new("bear"))
        .unwrap()
        .id()
        .clone();
    cast_spell_and_resolve(&service, &mut game, "player-1", creature_id.clone());

    let spell_id = player(&game, "player-1")
        .hand_card_by_definition(&demonictutor::CardDefinitionId::new("blessing"))
        .unwrap()
        .id()
        .clone();
    service
        .cast_spell(
            &mut game,
            CastSpellCommand::new(PlayerId::new("player-1"), spell_id)
                .with_target(SpellTarget::Creature(creature_id.clone())),
        )
        .unwrap();
    service
        .pass_priority(
            &mut game,
            PassPriorityCommand::new(PlayerId::new("player-1")),
        )
        .unwrap();
    service
        .pass_priority(
            &mut game,
            PassPriorityCommand::new(PlayerId::new("player-2")),
        )
        .unwrap();

    assert_eq!(
        player(&game, "player-1")
            .battlefield_card(&creature_id)
            .unwrap()
            .creature_stats(),
        Some((3, 3))
    );
}

#[test]
fn supported_ability_can_put_a_plus_one_counter_on_its_source_creature() {
    let (service, mut game) = setup_two_player_game(
        "game-counter-source-creature",
        filled_library(vec![self_growing_creature_card("hydra-seed", 0, 1, 1)], 10),
        filled_library(Vec::new(), 10),
    );
    advance_to_first_main_satisfying_cleanup(&service, &mut game);

    let creature_id = player(&game, "player-1")
        .hand_card_by_definition(&demonictutor::CardDefinitionId::new("hydra-seed"))
        .unwrap()
        .id()
        .clone();
    service
        .cast_spell(
            &mut game,
            CastSpellCommand::new(PlayerId::new("player-1"), creature_id.clone()),
        )
        .unwrap();
    service
        .pass_priority(
            &mut game,
            PassPriorityCommand::new(PlayerId::new("player-1")),
        )
        .unwrap();
    service
        .pass_priority(
            &mut game,
            PassPriorityCommand::new(PlayerId::new("player-2")),
        )
        .unwrap();

    service
        .activate_ability(
            &mut game,
            ActivateAbilityCommand::new(PlayerId::new("player-1"), creature_id.clone()),
        )
        .unwrap();
    service
        .pass_priority(
            &mut game,
            PassPriorityCommand::new(PlayerId::new("player-1")),
        )
        .unwrap();
    service
        .pass_priority(
            &mut game,
            PassPriorityCommand::new(PlayerId::new("player-2")),
        )
        .unwrap();

    assert_eq!(
        player(&game, "player-1")
            .battlefield_card(&creature_id)
            .unwrap()
            .creature_stats(),
        Some((2, 2))
    );
}

#[test]
fn distributed_counter_spell_can_put_both_counters_on_one_target_creature() {
    let (service, mut game) = setup_two_player_game(
        "game-distribute-counters-one-target",
        filled_library(
            vec![
                vanilla_creature("bear"),
                distribute_two_counters_among_up_to_two_target_creatures_sorcery_card(
                    "spread-blessing",
                    0,
                ),
            ],
            10,
        ),
        filled_library(Vec::new(), 10),
    );
    advance_to_first_main_satisfying_cleanup(&service, &mut game);

    let creature_id = player(&game, "player-1")
        .hand_card_by_definition(&CardDefinitionId::new("bear"))
        .unwrap()
        .id()
        .clone();
    cast_spell_and_resolve(&service, &mut game, "player-1", creature_id.clone());

    let spell_id = player(&game, "player-1")
        .hand_card_by_definition(&CardDefinitionId::new("spread-blessing"))
        .unwrap()
        .id()
        .clone();
    service
        .cast_spell(
            &mut game,
            CastSpellCommand::new(PlayerId::new("player-1"), spell_id)
                .with_target(SpellTarget::Creature(creature_id.clone()))
                .with_choice(SpellChoice::SecondaryCreatureTarget(None)),
        )
        .unwrap();
    service
        .pass_priority(
            &mut game,
            PassPriorityCommand::new(PlayerId::new("player-1")),
        )
        .unwrap();
    service
        .pass_priority(
            &mut game,
            PassPriorityCommand::new(PlayerId::new("player-2")),
        )
        .unwrap();

    assert_eq!(
        player(&game, "player-1")
            .battlefield_card(&creature_id)
            .unwrap()
            .creature_stats(),
        Some((4, 4))
    );
}

#[test]
fn distributed_counter_spell_can_put_one_counter_on_each_of_two_targets() {
    let (service, mut game) = setup_two_player_game(
        "game-distribute-counters-two-targets",
        filled_library(
            vec![
                vanilla_creature("bear-a"),
                vanilla_creature("bear-b"),
                distribute_two_counters_among_up_to_two_target_creatures_sorcery_card(
                    "spread-blessing",
                    0,
                ),
            ],
            10,
        ),
        filled_library(Vec::new(), 10),
    );
    advance_to_first_main_satisfying_cleanup(&service, &mut game);

    let first_creature_id = player(&game, "player-1")
        .hand_card_by_definition(&CardDefinitionId::new("bear-a"))
        .unwrap()
        .id()
        .clone();
    let second_creature_id = player(&game, "player-1")
        .hand_card_by_definition(&CardDefinitionId::new("bear-b"))
        .unwrap()
        .id()
        .clone();
    cast_spell_and_resolve(&service, &mut game, "player-1", first_creature_id.clone());
    cast_spell_and_resolve(&service, &mut game, "player-1", second_creature_id.clone());

    let spell_id = player(&game, "player-1")
        .hand_card_by_definition(&CardDefinitionId::new("spread-blessing"))
        .unwrap()
        .id()
        .clone();
    service
        .cast_spell(
            &mut game,
            CastSpellCommand::new(PlayerId::new("player-1"), spell_id)
                .with_target(SpellTarget::Creature(first_creature_id.clone()))
                .with_choice(SpellChoice::SecondaryCreatureTarget(Some(
                    second_creature_id.clone(),
                ))),
        )
        .unwrap();
    service
        .pass_priority(
            &mut game,
            PassPriorityCommand::new(PlayerId::new("player-1")),
        )
        .unwrap();
    service
        .pass_priority(
            &mut game,
            PassPriorityCommand::new(PlayerId::new("player-2")),
        )
        .unwrap();

    assert_eq!(
        player(&game, "player-1")
            .battlefield_card(&first_creature_id)
            .unwrap()
            .creature_stats(),
        Some((3, 3))
    );
    assert_eq!(
        player(&game, "player-1")
            .battlefield_card(&second_creature_id)
            .unwrap()
            .creature_stats(),
        Some((3, 3))
    );
}

#[test]
fn distributed_counter_spell_rejects_reusing_the_primary_target_as_secondary_choice() {
    let (service, mut game) = setup_two_player_game(
        "game-distribute-counters-duplicate-target",
        filled_library(
            vec![
                vanilla_creature("bear"),
                distribute_two_counters_among_up_to_two_target_creatures_sorcery_card(
                    "spread-blessing",
                    0,
                ),
            ],
            10,
        ),
        filled_library(Vec::new(), 10),
    );
    advance_to_first_main_satisfying_cleanup(&service, &mut game);

    let creature_id = player(&game, "player-1")
        .hand_card_by_definition(&CardDefinitionId::new("bear"))
        .unwrap()
        .id()
        .clone();
    cast_spell_and_resolve(&service, &mut game, "player-1", creature_id.clone());

    let spell_id = player(&game, "player-1")
        .hand_card_by_definition(&CardDefinitionId::new("spread-blessing"))
        .unwrap()
        .id()
        .clone();

    assert!(service
        .cast_spell(
            &mut game,
            CastSpellCommand::new(PlayerId::new("player-1"), spell_id)
                .with_target(SpellTarget::Creature(creature_id.clone()))
                .with_choice(SpellChoice::SecondaryCreatureTarget(Some(creature_id))),
        )
        .is_err());
}

#[test]
fn distributed_counter_spell_only_applies_one_counter_when_secondary_target_is_gone_on_resolution()
{
    let (service, mut game) = setup_two_player_game(
        "game-distribute-counters-target-gone",
        filled_library(
            vec![
                creature_card("sprite-a", 0, 1, 1),
                creature_card("sprite-b", 0, 1, 1),
                distribute_two_counters_among_up_to_two_target_creatures_sorcery_card(
                    "spread-blessing",
                    0,
                ),
                targeted_damage_instant_card("ping", 0, 1),
            ],
            10,
        ),
        filled_library(Vec::new(), 10),
    );
    advance_to_first_main_satisfying_cleanup(&service, &mut game);

    let first_creature_id = player(&game, "player-1")
        .hand_card_by_definition(&CardDefinitionId::new("sprite-a"))
        .unwrap()
        .id()
        .clone();
    let second_creature_id = player(&game, "player-1")
        .hand_card_by_definition(&CardDefinitionId::new("sprite-b"))
        .unwrap()
        .id()
        .clone();
    cast_spell_and_resolve(&service, &mut game, "player-1", first_creature_id.clone());
    cast_spell_and_resolve(&service, &mut game, "player-1", second_creature_id.clone());

    let blessing_id = player(&game, "player-1")
        .hand_card_by_definition(&CardDefinitionId::new("spread-blessing"))
        .unwrap()
        .id()
        .clone();
    let ping_id = player(&game, "player-1")
        .hand_card_by_definition(&CardDefinitionId::new("ping"))
        .unwrap()
        .id()
        .clone();

    service
        .cast_spell(
            &mut game,
            CastSpellCommand::new(PlayerId::new("player-1"), blessing_id)
                .with_target(SpellTarget::Creature(first_creature_id.clone()))
                .with_choice(SpellChoice::SecondaryCreatureTarget(Some(
                    second_creature_id.clone(),
                ))),
        )
        .unwrap();
    service
        .cast_spell(
            &mut game,
            CastSpellCommand::new(PlayerId::new("player-1"), ping_id)
                .with_target(SpellTarget::Creature(second_creature_id.clone())),
        )
        .unwrap();
    service
        .pass_priority(
            &mut game,
            PassPriorityCommand::new(PlayerId::new("player-1")),
        )
        .unwrap();
    service
        .pass_priority(
            &mut game,
            PassPriorityCommand::new(PlayerId::new("player-2")),
        )
        .unwrap();
    service
        .pass_priority(
            &mut game,
            PassPriorityCommand::new(PlayerId::new("player-1")),
        )
        .unwrap();
    service
        .pass_priority(
            &mut game,
            PassPriorityCommand::new(PlayerId::new("player-2")),
        )
        .unwrap();

    assert_eq!(
        player(&game, "player-1")
            .battlefield_card(&first_creature_id)
            .unwrap()
            .creature_stats(),
        Some((2, 2))
    );
    assert!(player(&game, "player-1")
        .battlefield_card(&second_creature_id)
        .is_none());
}
