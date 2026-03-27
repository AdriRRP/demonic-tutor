#![allow(clippy::unwrap_used)]

//! Unit coverage for optional may triggers.

use {
    crate::support::{
        advance_to_first_main_satisfying_cleanup, etb_may_life_gain_creature_card, filled_library,
        setup_two_player_game,
    },
    demonictutor::{
        CardInstanceId, CastSpellCommand, DomainError, GameError, PassPriorityCommand, PlayerId,
        ResolveOptionalEffectCommand,
    },
};

fn pass_until_optional_decision(
    service: &demonictutor::GameService<
        demonictutor::InMemoryEventStore,
        demonictutor::InMemoryEventBus,
    >,
    game: &mut demonictutor::Game,
) {
    for _ in 0..2 {
        let first_holder = game.priority().unwrap().current_holder().clone();
        service
            .pass_priority(game, PassPriorityCommand::new(first_holder))
            .unwrap();
        let second_holder = game.priority().unwrap().current_holder().clone();
        service
            .pass_priority(game, PassPriorityCommand::new(second_holder))
            .unwrap();
    }
}

#[test]
fn may_trigger_surfaces_a_pending_optional_choice_at_resolution_time() {
    let (service, mut game) = setup_two_player_game(
        "game-may-trigger-pending",
        filled_library(
            vec![etb_may_life_gain_creature_card("kindly-cleric", 0, 1, 1, 2)],
            10,
        ),
        filled_library(Vec::new(), 10),
    );

    advance_to_first_main_satisfying_cleanup(&service, &mut game);
    service
        .cast_spell(
            &mut game,
            CastSpellCommand::new(
                PlayerId::new("player-1"),
                CardInstanceId::new("game-may-trigger-pending-player-1-0"),
            ),
        )
        .unwrap();
    pass_until_optional_decision(&service, &mut game);

    assert_eq!(game.stack().len(), 1);
    assert!(game.priority().is_none());
    assert!(matches!(
        game.pending_decision(),
        Some(demonictutor::PendingDecision::OptionalEffect { .. })
    ));
}

#[test]
fn may_trigger_does_nothing_when_controller_answers_no() {
    let (service, mut game) = setup_two_player_game(
        "game-may-trigger-no",
        filled_library(
            vec![etb_may_life_gain_creature_card("kindly-cleric", 0, 1, 1, 2)],
            10,
        ),
        filled_library(Vec::new(), 10),
    );

    advance_to_first_main_satisfying_cleanup(&service, &mut game);
    service
        .cast_spell(
            &mut game,
            CastSpellCommand::new(
                PlayerId::new("player-1"),
                CardInstanceId::new("game-may-trigger-no-player-1-0"),
            ),
        )
        .unwrap();
    pass_until_optional_decision(&service, &mut game);

    let outcome = service
        .resolve_optional_effect(
            &mut game,
            ResolveOptionalEffectCommand::decline(PlayerId::new("player-1")),
        )
        .unwrap();

    assert!(outcome.stack_top_resolved.is_some());
    assert!(outcome.life_changed.is_none());
    assert_eq!(game.stack().len(), 0);
    assert_eq!(game.players()[0].life(), 20);
    assert!(game.priority().is_some());
    assert!(game.pending_decision().is_none());
}

#[test]
fn may_trigger_applies_its_effect_when_controller_answers_yes() {
    let (service, mut game) = setup_two_player_game(
        "game-may-trigger-yes",
        filled_library(
            vec![etb_may_life_gain_creature_card("kindly-cleric", 0, 1, 1, 2)],
            10,
        ),
        filled_library(Vec::new(), 10),
    );

    advance_to_first_main_satisfying_cleanup(&service, &mut game);
    service
        .cast_spell(
            &mut game,
            CastSpellCommand::new(
                PlayerId::new("player-1"),
                CardInstanceId::new("game-may-trigger-yes-player-1-0"),
            ),
        )
        .unwrap();
    pass_until_optional_decision(&service, &mut game);

    let outcome = service
        .resolve_optional_effect(
            &mut game,
            ResolveOptionalEffectCommand::accept(PlayerId::new("player-1")),
        )
        .unwrap();

    assert!(outcome.stack_top_resolved.is_some());
    assert!(outcome.life_changed.is_some());
    assert_eq!(game.players()[0].life(), 22);
    assert_eq!(game.stack().len(), 0);
    assert!(game.priority().is_some());
}

#[test]
fn only_the_optional_effect_controller_can_answer_the_pending_choice() {
    let (service, mut game) = setup_two_player_game(
        "game-may-trigger-wrong-player",
        filled_library(
            vec![etb_may_life_gain_creature_card("kindly-cleric", 0, 1, 1, 2)],
            10,
        ),
        filled_library(Vec::new(), 10),
    );

    advance_to_first_main_satisfying_cleanup(&service, &mut game);
    service
        .cast_spell(
            &mut game,
            CastSpellCommand::new(
                PlayerId::new("player-1"),
                CardInstanceId::new("game-may-trigger-wrong-player-player-1-0"),
            ),
        )
        .unwrap();
    pass_until_optional_decision(&service, &mut game);

    let result = service.resolve_optional_effect(
        &mut game,
        ResolveOptionalEffectCommand::accept(PlayerId::new("player-2")),
    );

    assert!(matches!(
        result,
        Err(DomainError::Game(GameError::NotOptionalEffectController { current, requested }))
            if current == PlayerId::new("player-1") && requested == PlayerId::new("player-2")
    ));
    assert!(matches!(
        game.pending_decision(),
        Some(demonictutor::PendingDecision::OptionalEffect { .. })
    ));
    assert_eq!(game.stack().len(), 1);
}
