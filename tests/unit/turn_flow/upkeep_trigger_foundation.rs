#![allow(clippy::panic)]
#![allow(clippy::unwrap_used)]

//! Unit coverage for unit turn flow upkeep trigger foundation.

use {
    crate::support::{
        advance_to_first_main_satisfying_cleanup, advance_to_player_phase_satisfying_cleanup,
        filled_library, player, setup_two_player_game, upkeep_life_gain_artifact_card,
    },
    demonictutor::{
        AdvanceTurnCommand, AdvanceTurnOutcome, CardDefinitionId, CastSpellCommand,
        PassPriorityCommand, PlayerId, TriggeredAbilityEvent,
    },
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
fn entering_upkeep_enqueues_supported_upkeep_triggers_before_normal_play() {
    let (service, mut game) = setup_two_player_game(
        "game-upkeep-trigger",
        filled_library(
            vec![upkeep_life_gain_artifact_card("sun-dial-lite", 0, 1)],
            10,
        ),
        filled_library(Vec::new(), 10),
    );
    advance_to_first_main_satisfying_cleanup(&service, &mut game);
    let artifact_id = player(&game, "player-1")
        .hand_card_by_definition(&CardDefinitionId::new("sun-dial-lite"))
        .unwrap()
        .id()
        .clone();
    service
        .cast_spell(
            &mut game,
            CastSpellCommand::new(PlayerId::new("player-1"), artifact_id),
        )
        .unwrap();
    resolve_current_stack(&service, &mut game);

    advance_to_player_phase_satisfying_cleanup(
        &service,
        &mut game,
        "player-1",
        demonictutor::Phase::Untap,
    );
    let outcome = service
        .advance_turn(&mut game, AdvanceTurnCommand::new())
        .unwrap();

    let AdvanceTurnOutcome::Progressed {
        triggered_abilities_put_on_stack,
        ..
    } = outcome
    else {
        panic!("upkeep transition should progress turn");
    };

    assert_eq!(game.phase(), &demonictutor::Phase::Upkeep);
    assert_eq!(triggered_abilities_put_on_stack.len(), 1);
    assert_eq!(
        triggered_abilities_put_on_stack[0].trigger,
        TriggeredAbilityEvent::BeginningOfUpkeep
    );
    assert_eq!(game.stack().len(), 1);
    assert_eq!(
        game.priority().unwrap().current_holder(),
        &PlayerId::new("player-1")
    );
}

#[test]
fn upkeep_trigger_resolves_through_stack_and_does_not_duplicate_within_same_upkeep() {
    let (service, mut game) = setup_two_player_game(
        "game-upkeep-trigger-resolve",
        filled_library(
            vec![upkeep_life_gain_artifact_card("sun-dial-lite", 0, 1)],
            10,
        ),
        filled_library(Vec::new(), 10),
    );
    advance_to_first_main_satisfying_cleanup(&service, &mut game);
    let artifact_id = player(&game, "player-1")
        .hand_card_by_definition(&CardDefinitionId::new("sun-dial-lite"))
        .unwrap()
        .id()
        .clone();
    service
        .cast_spell(
            &mut game,
            CastSpellCommand::new(PlayerId::new("player-1"), artifact_id),
        )
        .unwrap();
    resolve_current_stack(&service, &mut game);

    advance_to_player_phase_satisfying_cleanup(
        &service,
        &mut game,
        "player-1",
        demonictutor::Phase::Untap,
    );
    service
        .advance_turn(&mut game, AdvanceTurnCommand::new())
        .unwrap();

    let trigger_resolution = resolve_current_stack(&service, &mut game);

    assert!(trigger_resolution.spell_cast.is_none());
    assert_eq!(player(&game, "player-1").life(), 21);
    assert_eq!(game.stack().len(), 0);
}
