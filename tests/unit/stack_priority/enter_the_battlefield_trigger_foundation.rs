#![allow(clippy::panic)]
#![allow(clippy::unwrap_used)]

//! Unit coverage for unit stack priority enter the battlefield trigger foundation.

use {
    crate::support::{
        advance_to_first_main_satisfying_cleanup, etb_life_gain_creature_card, filled_library,
        player, setup_two_player_game,
    },
    demonictutor::{CastSpellCommand, PassPriorityCommand, PlayerId, TriggeredAbilityEvent},
};

fn resolve_current_stack(
    service: &demonictutor::GameService<
        demonictutor::InMemoryEventStore,
        demonictutor::InMemoryEventBus,
    >,
    game: &mut demonictutor::Game,
) -> demonictutor::PassPriorityOutcome {
    let first_holder = game.priority().unwrap().current_holder().clone();
    service
        .pass_priority(game, PassPriorityCommand::new(first_holder))
        .unwrap();

    let second_holder = game.priority().unwrap().current_holder().clone();
    service
        .pass_priority(game, PassPriorityCommand::new(second_holder))
        .unwrap()
}

#[test]
fn permanent_entry_enqueues_supported_etb_trigger_on_the_stack() {
    let (service, mut game) = setup_two_player_game(
        "game-etb-trigger",
        filled_library(vec![etb_life_gain_creature_card("healer-cub", 0, 2, 2, 2)], 10),
        filled_library(Vec::new(), 10),
    );
    advance_to_first_main_satisfying_cleanup(&service, &mut game);
    let creature_id = player(&game, "player-1")
        .hand_card_by_definition(&demonictutor::CardDefinitionId::new("healer-cub"))
        .unwrap()
        .id()
        .clone();

    service
        .cast_spell(
            &mut game,
            CastSpellCommand::new(PlayerId::new("player-1"), creature_id.clone()),
        )
        .unwrap();

    let resolution = resolve_current_stack(&service, &mut game);

    assert_eq!(resolution.spell_cast.unwrap().card_id, creature_id);
    assert_eq!(resolution.triggered_abilities_put_on_stack.len(), 1);
    assert_eq!(
        resolution.triggered_abilities_put_on_stack[0].trigger,
        TriggeredAbilityEvent::EntersBattlefield
    );
    assert_eq!(
        resolution.triggered_abilities_put_on_stack[0].source_card_id,
        creature_id
    );
    assert_eq!(game.stack().len(), 1);
    assert_eq!(player(&game, "player-1").life(), 20);
}

#[test]
fn etb_trigger_resolves_through_the_existing_stack_corridor() {
    let (service, mut game) = setup_two_player_game(
        "game-etb-trigger-resolve",
        filled_library(vec![etb_life_gain_creature_card("healer-cub", 0, 2, 2, 2)], 10),
        filled_library(Vec::new(), 10),
    );
    advance_to_first_main_satisfying_cleanup(&service, &mut game);
    let creature_id = player(&game, "player-1")
        .hand_card_by_definition(&demonictutor::CardDefinitionId::new("healer-cub"))
        .unwrap()
        .id()
        .clone();

    service
        .cast_spell(
            &mut game,
            CastSpellCommand::new(PlayerId::new("player-1"), creature_id.clone()),
        )
        .unwrap();
    resolve_current_stack(&service, &mut game);

    let trigger_resolution = resolve_current_stack(&service, &mut game);

    assert!(trigger_resolution.spell_cast.is_none());
    assert!(trigger_resolution.stack_top_resolved.is_some());
    assert_eq!(player(&game, "player-1").life(), 22);
    assert_eq!(game.stack().len(), 0);
    assert_eq!(
        game.priority().unwrap().current_holder(),
        &PlayerId::new("player-1")
    );
}
