#![allow(clippy::panic)]
#![allow(clippy::unwrap_used)]

//! Unit coverage for unit turn flow end step trigger foundation.

use {
    crate::support::{
        advance_to_first_main_satisfying_cleanup, end_step_life_gain_artifact_card, filled_library,
        player, satisfy_cleanup_discard, setup_two_player_game,
    },
    demonictutor::{
        AdvanceTurnCommand, AdvanceTurnOutcome, CardDefinitionId, CastSpellCommand,
        PassPriorityCommand, Phase, PlayerId, TriggeredAbilityEvent,
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

fn advance_to_player_phase_resolving_stack(
    service: &demonictutor::GameService<
        demonictutor::InMemoryEventStore,
        demonictutor::InMemoryEventBus,
    >,
    game: &mut demonictutor::Game,
    player_id: &str,
    phase: Phase,
) {
    let target_player = PlayerId::new(player_id);

    for _ in 0..64 {
        if game.active_player() == &target_player && game.phase() == &phase {
            let priority_ready = game
                .priority()
                .is_none_or(|priority| priority.current_holder() == &target_player);
            if game.stack().is_empty() && priority_ready {
                return;
            }
        }

        while game.has_open_priority_window() {
            if game.stack().is_empty() {
                let first_holder = game.priority().unwrap().current_holder().clone();
                service
                    .pass_priority(game, PassPriorityCommand::new(first_holder))
                    .unwrap();
                let second_holder = game.priority().unwrap().current_holder().clone();
                service
                    .pass_priority(game, PassPriorityCommand::new(second_holder))
                    .unwrap();
            } else {
                let _ = resolve_current_stack(service, game);
            }
        }

        satisfy_cleanup_discard(service, game);
        let outcome = service
            .advance_turn(game, AdvanceTurnCommand::new())
            .unwrap();
        assert!(matches!(outcome, AdvanceTurnOutcome::Progressed { .. }));
    }

    panic!("failed to reach {phase} for {target_player}");
}

#[test]
fn entering_end_step_enqueues_supported_end_step_triggers_before_cleanup() {
    let (service, mut game) = setup_two_player_game(
        "game-end-step-trigger",
        filled_library(
            vec![end_step_life_gain_artifact_card("twilight-idol", 0, 1)],
            10,
        ),
        filled_library(Vec::new(), 10),
    );
    advance_to_first_main_satisfying_cleanup(&service, &mut game);
    let artifact_id = player(&game, "player-1")
        .hand_card_by_definition(&CardDefinitionId::new("twilight-idol"))
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

    advance_to_player_phase_resolving_stack(&service, &mut game, "player-1", Phase::SecondMain);
    let first_holder = game.priority().unwrap().current_holder().clone();
    service
        .pass_priority(&mut game, PassPriorityCommand::new(first_holder))
        .unwrap();
    let second_holder = game.priority().unwrap().current_holder().clone();
    service
        .pass_priority(&mut game, PassPriorityCommand::new(second_holder))
        .unwrap();
    let outcome = service
        .advance_turn(&mut game, AdvanceTurnCommand::new())
        .unwrap();

    let AdvanceTurnOutcome::Progressed {
        triggered_abilities_put_on_stack,
        ..
    } = outcome
    else {
        panic!("end-step transition should progress turn");
    };

    assert_eq!(game.phase(), &demonictutor::Phase::EndStep);
    assert_eq!(triggered_abilities_put_on_stack.len(), 1);
    assert_eq!(
        triggered_abilities_put_on_stack[0].trigger,
        TriggeredAbilityEvent::BeginningOfEndStep
    );
    assert_eq!(game.stack().len(), 1);
}

#[test]
fn end_step_trigger_resolves_before_cleanup_only_follow_up() {
    let (service, mut game) = setup_two_player_game(
        "game-end-step-trigger-resolve",
        filled_library(
            vec![end_step_life_gain_artifact_card("twilight-idol", 0, 1)],
            10,
        ),
        filled_library(Vec::new(), 10),
    );
    advance_to_first_main_satisfying_cleanup(&service, &mut game);
    let artifact_id = player(&game, "player-1")
        .hand_card_by_definition(&CardDefinitionId::new("twilight-idol"))
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

    advance_to_player_phase_resolving_stack(&service, &mut game, "player-1", Phase::SecondMain);
    let first_holder = game.priority().unwrap().current_holder().clone();
    service
        .pass_priority(&mut game, PassPriorityCommand::new(first_holder))
        .unwrap();
    let second_holder = game.priority().unwrap().current_holder().clone();
    service
        .pass_priority(&mut game, PassPriorityCommand::new(second_holder))
        .unwrap();
    service
        .advance_turn(&mut game, AdvanceTurnCommand::new())
        .unwrap();

    let trigger_resolution = resolve_current_stack(&service, &mut game);

    assert!(trigger_resolution.spell_cast.is_none());
    assert_eq!(player(&game, "player-1").life(), 21);
    assert_eq!(game.phase(), &demonictutor::Phase::EndStep);
}

#[test]
fn entering_end_step_enqueues_triggers_from_all_players_battlefields() {
    let (service, mut game) = setup_two_player_game(
        "game-end-step-trigger-all-players",
        filled_library(
            vec![end_step_life_gain_artifact_card("twilight-idol", 0, 1)],
            10,
        ),
        filled_library(
            vec![end_step_life_gain_artifact_card("midnight-idol", 0, 1)],
            10,
        ),
    );
    advance_to_player_phase_resolving_stack(&service, &mut game, "player-1", Phase::FirstMain);
    let player_one_artifact_id = player(&game, "player-1")
        .hand_card_by_definition(&CardDefinitionId::new("twilight-idol"))
        .unwrap()
        .id()
        .clone();
    service
        .cast_spell(
            &mut game,
            CastSpellCommand::new(PlayerId::new("player-1"), player_one_artifact_id),
        )
        .unwrap();
    resolve_current_stack(&service, &mut game);

    advance_to_player_phase_resolving_stack(&service, &mut game, "player-2", Phase::FirstMain);
    let player_two_artifact_id = player(&game, "player-2")
        .hand_card_by_definition(&CardDefinitionId::new("midnight-idol"))
        .unwrap()
        .id()
        .clone();
    service
        .cast_spell(
            &mut game,
            CastSpellCommand::new(PlayerId::new("player-2"), player_two_artifact_id),
        )
        .unwrap();
    resolve_current_stack(&service, &mut game);

    advance_to_player_phase_resolving_stack(&service, &mut game, "player-1", Phase::SecondMain);
    let first_holder = game.priority().unwrap().current_holder().clone();
    service
        .pass_priority(&mut game, PassPriorityCommand::new(first_holder))
        .unwrap();
    let second_holder = game.priority().unwrap().current_holder().clone();
    service
        .pass_priority(&mut game, PassPriorityCommand::new(second_holder))
        .unwrap();
    let outcome = service
        .advance_turn(&mut game, AdvanceTurnCommand::new())
        .unwrap();

    let AdvanceTurnOutcome::Progressed {
        triggered_abilities_put_on_stack,
        ..
    } = outcome
    else {
        panic!("end-step transition should progress turn");
    };

    assert_eq!(triggered_abilities_put_on_stack.len(), 2);
    assert!(triggered_abilities_put_on_stack.iter().any(|event| {
        event.player_id == PlayerId::new("player-1")
            && event.trigger == TriggeredAbilityEvent::BeginningOfEndStep
    }));
    assert!(triggered_abilities_put_on_stack.iter().any(|event| {
        event.player_id == PlayerId::new("player-2")
            && event.trigger == TriggeredAbilityEvent::BeginningOfEndStep
    }));
    assert_eq!(game.stack().len(), 2);
}
