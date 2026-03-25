#![allow(clippy::panic)]
#![allow(clippy::unwrap_used)]

use {
    crate::support::{
        advance_to_first_main_satisfying_cleanup, cast_spell_and_resolve, filled_library, player,
        put_counter_on_target_creature_sorcery_card, self_growing_creature_card,
        setup_two_player_game, vanilla_creature,
    },
    demonictutor::{ActivateAbilityCommand, CastSpellCommand, PassPriorityCommand, PlayerId, SpellTarget},
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
        .pass_priority(&mut game, PassPriorityCommand::new(PlayerId::new("player-1")))
        .unwrap();
    service
        .pass_priority(&mut game, PassPriorityCommand::new(PlayerId::new("player-2")))
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
        .pass_priority(&mut game, PassPriorityCommand::new(PlayerId::new("player-1")))
        .unwrap();
    service
        .pass_priority(&mut game, PassPriorityCommand::new(PlayerId::new("player-2")))
        .unwrap();

    service
        .activate_ability(
            &mut game,
            ActivateAbilityCommand::new(PlayerId::new("player-1"), creature_id.clone()),
        )
        .unwrap();
    service
        .pass_priority(&mut game, PassPriorityCommand::new(PlayerId::new("player-1")))
        .unwrap();
    service
        .pass_priority(&mut game, PassPriorityCommand::new(PlayerId::new("player-2")))
        .unwrap();

    assert_eq!(
        player(&game, "player-1")
            .battlefield_card(&creature_id)
            .unwrap()
            .creature_stats(),
        Some((2, 2))
    );
}
