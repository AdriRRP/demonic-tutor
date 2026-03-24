#![allow(clippy::unwrap_used)]

//! Unit coverage for unit turn flow draw card.

use {
    crate::support::{
        advance_n_raw, advance_to_player_first_main_satisfying_cleanup, filled_library, land_card,
        setup_two_player_game, vanilla_creature,
    },
    demonictutor::{
        CardInstanceId, DomainError, DrawCardsEffectCommand, DrawCardsEffectOutcome, GameEndReason,
        GameError, Phase, PlayLandCommand, PlayerId,
    },
};

fn create_game_with_library_cards() -> demonictutor::Game {
    let (.., game) = setup_two_player_game(
        "game-1",
        filled_library(vec![land_card("forest")], 12),
        filled_library(vec![land_card("mountain")], 10),
    );
    game
}

#[test]
fn draw_cards_effect_works_in_main_phase() {
    let (service, mut game) = setup_two_player_game(
        "game-1",
        filled_library(vec![vanilla_creature("grizzly-bears")], 10),
        filled_library(vec![land_card("mountain")], 10),
    );

    advance_to_player_first_main_satisfying_cleanup(&service, &mut game, "player-2");

    let result = service.draw_cards_effect(
        &mut game,
        &DrawCardsEffectCommand::new(PlayerId::new("player-2"), PlayerId::new("player-2"), 1),
    );
    assert!(matches!(
        result,
        Ok(DrawCardsEffectOutcome {
            game_ended: None,
            ..
        })
    ));
}

#[test]
fn draw_cards_effect_moves_cards_from_library_to_hand() {
    let (service, mut game) = setup_two_player_game(
        "game-1",
        filled_library(vec![land_card("forest")], 10),
        filled_library(vec![land_card("mountain")], 10),
    );

    advance_to_player_first_main_satisfying_cleanup(&service, &mut game, "player-2");

    let hand_before = game.players()[1].hand_size();
    let lib_before = game.players()[1].library_size();

    let outcome = service
        .draw_cards_effect(
            &mut game,
            &DrawCardsEffectCommand::new(PlayerId::new("player-2"), PlayerId::new("player-2"), 2),
        )
        .unwrap();
    assert_eq!(outcome.cards_drawn.len(), 2);
    assert!(outcome.game_ended.is_none());

    let hand_after = game.players()[1].hand_size();
    let lib_after = game.players()[1].library_size();

    assert_eq!(hand_before + 2, hand_after);
    assert_eq!(lib_before - 2, lib_after);
}

#[test]
fn draw_cards_effect_emits_one_event_per_drawn_card() {
    let (service, mut game) = setup_two_player_game(
        "game-1",
        filled_library(vec![land_card("forest")], 8),
        filled_library(vec![land_card("mountain")], 10),
    );

    advance_to_player_first_main_satisfying_cleanup(&service, &mut game, "player-2");

    let outcome = service
        .draw_cards_effect(
            &mut game,
            &DrawCardsEffectCommand::new(PlayerId::new("player-2"), PlayerId::new("player-2"), 2),
        )
        .unwrap();
    assert_eq!(outcome.cards_drawn.len(), 2);
    assert!(outcome
        .cards_drawn
        .iter()
        .all(|event| event.player_id.as_str() == "player-2"));
}

#[test]
fn draw_cards_effect_can_target_another_player() {
    let (service, mut game) = setup_two_player_game(
        "game-1",
        filled_library(vec![land_card("forest")], 10),
        filled_library(vec![land_card("mountain")], 10),
    );

    advance_to_player_first_main_satisfying_cleanup(&service, &mut game, "player-1");

    let bob_hand_before = game.players()[1].hand_size();
    let bob_library_before = game.players()[1].library_size();

    let outcome = service
        .draw_cards_effect(
            &mut game,
            &DrawCardsEffectCommand::new(PlayerId::new("player-1"), PlayerId::new("player-2"), 2),
        )
        .unwrap();

    assert_eq!(outcome.cards_drawn.len(), 2);
    assert!(outcome
        .cards_drawn
        .iter()
        .all(|event| event.player_id == PlayerId::new("player-2")));
    assert_eq!(game.players()[1].hand_size(), bob_hand_before + 2);
    assert_eq!(game.players()[1].library_size(), bob_library_before - 2);
}

#[test]
fn draw_cards_effect_ends_the_game_when_the_library_runs_out_mid_effect() {
    let mut game = create_game_with_library_cards();
    let service = crate::support::create_service();

    advance_to_player_first_main_satisfying_cleanup(&service, &mut game, "player-2");

    let result = service.draw_cards_effect(
        &mut game,
        &DrawCardsEffectCommand::new(PlayerId::new("player-2"), PlayerId::new("player-2"), 3),
    );

    let outcome = result.unwrap();
    assert_eq!(outcome.cards_drawn.len(), 2);
    assert_eq!(
        outcome
            .game_ended
            .as_ref()
            .map(|event| event.loser_id.clone()),
        Some(PlayerId::new("player-2"))
    );
    assert_eq!(
        outcome
            .game_ended
            .as_ref()
            .map(|event| event.winner_id.clone()),
        Some(PlayerId::new("player-1"))
    );
    assert_eq!(
        outcome.game_ended.as_ref().map(|event| event.reason),
        Some(GameEndReason::EmptyLibraryDraw)
    );
    assert!(game.is_over());
}

#[test]
fn draw_cards_effect_ends_the_game_when_the_target_library_runs_out_mid_effect() {
    let (service, mut game) = setup_two_player_game(
        "game-1",
        filled_library(vec![land_card("forest")], 10),
        filled_library(vec![land_card("mountain")], 9),
    );

    advance_to_player_first_main_satisfying_cleanup(&service, &mut game, "player-1");

    let result = service.draw_cards_effect(
        &mut game,
        &DrawCardsEffectCommand::new(PlayerId::new("player-1"), PlayerId::new("player-2"), 3),
    );

    let outcome = result.unwrap();
    assert_eq!(outcome.cards_drawn.len(), 2);
    assert_eq!(
        outcome
            .game_ended
            .as_ref()
            .map(|event| event.loser_id.clone()),
        Some(PlayerId::new("player-2"))
    );
    assert_eq!(
        outcome
            .game_ended
            .as_ref()
            .map(|event| event.winner_id.clone()),
        Some(PlayerId::new("player-1"))
    );
    assert_eq!(
        outcome.game_ended.as_ref().map(|event| event.reason),
        Some(GameEndReason::EmptyLibraryDraw)
    );
    assert!(game.is_over());
}

#[test]
fn draw_cards_effect_fails_when_not_player_turn() {
    let (service, mut game) = setup_two_player_game(
        "game-1",
        filled_library(vec![land_card("forest")], 8),
        filled_library(vec![land_card("mountain")], 7),
    );

    let result = service.draw_cards_effect(
        &mut game,
        &DrawCardsEffectCommand::new(PlayerId::new("player-2"), PlayerId::new("player-2"), 1),
    );

    assert!(matches!(
        result,
        Err(DomainError::Game(GameError::NotYourTurn { .. }))
    ));
}

#[test]
fn draw_cards_effect_allows_playing_land_after_draw() {
    let mut game = create_game_with_library_cards();
    let service = crate::support::create_service();

    advance_to_player_first_main_satisfying_cleanup(&service, &mut game, "player-1");

    let outcome = service
        .draw_cards_effect(
            &mut game,
            &DrawCardsEffectCommand::new(PlayerId::new("player-1"), PlayerId::new("player-1"), 1),
        )
        .unwrap();
    assert_eq!(outcome.cards_drawn.len(), 1);
    assert!(outcome.game_ended.is_none());

    advance_to_player_first_main_satisfying_cleanup(&service, &mut game, "player-2");

    let result = service.play_land(
        &mut game,
        PlayLandCommand::new(
            PlayerId::new("player-2"),
            CardInstanceId::new("game-1-player-2-0"),
        ),
    );

    assert!(result.is_ok());
}

#[test]
fn gameplay_actions_fail_after_the_game_has_ended() {
    let mut game = create_game_with_library_cards();
    let service = crate::support::create_service();

    advance_to_player_first_main_satisfying_cleanup(&service, &mut game, "player-2");

    let outcome = service
        .draw_cards_effect(
            &mut game,
            &DrawCardsEffectCommand::new(PlayerId::new("player-2"), PlayerId::new("player-2"), 3),
        )
        .unwrap();
    assert_eq!(outcome.cards_drawn.len(), 2);
    assert!(outcome.game_ended.is_some());

    let result = service.play_land(
        &mut game,
        PlayLandCommand::new(
            PlayerId::new("player-2"),
            CardInstanceId::new("game-1-player-2-0"),
        ),
    );

    assert_eq!(
        result.unwrap_err(),
        DomainError::Game(GameError::GameAlreadyEnded)
    );
}

#[test]
fn draw_cards_effect_fails_outside_main_phases() {
    let (service, mut game) = setup_two_player_game(
        "game-1",
        filled_library(vec![land_card("forest")], 10),
        filled_library(vec![land_card("mountain")], 10),
    );

    advance_n_raw(&service, &mut game, 2);
    assert_eq!(game.phase(), &Phase::Upkeep);

    let result = service.draw_cards_effect(
        &mut game,
        &DrawCardsEffectCommand::new(PlayerId::new("player-1"), PlayerId::new("player-1"), 1),
    );

    assert!(matches!(
        result,
        Err(DomainError::Phase(
            demonictutor::PhaseError::InvalidForDraw {
                phase: Phase::Upkeep
            }
        ))
    ));
}

#[test]
fn draw_cards_effect_fails_when_zero_cards_are_requested() {
    let (service, mut game) = setup_two_player_game(
        "game-1",
        filled_library(vec![land_card("forest")], 10),
        filled_library(vec![land_card("mountain")], 10),
    );

    advance_to_player_first_main_satisfying_cleanup(&service, &mut game, "player-1");

    let result = service.draw_cards_effect(
        &mut game,
        &DrawCardsEffectCommand::new(PlayerId::new("player-1"), PlayerId::new("player-1"), 0),
    );

    assert_eq!(
        result.unwrap_err(),
        DomainError::Game(GameError::InvalidDrawCount(0))
    );
}

#[test]
fn draw_cards_effect_fails_while_priority_window_is_open() {
    let (service, mut game) = setup_two_player_game(
        "game-1",
        filled_library(vec![vanilla_creature("grizzly-bears")], 10),
        filled_library(vec![land_card("mountain")], 10),
    );

    advance_to_player_first_main_satisfying_cleanup(&service, &mut game, "player-1");
    service
        .cast_spell(
            &mut game,
            demonictutor::CastSpellCommand::new(
                PlayerId::new("player-1"),
                CardInstanceId::new("game-1-player-1-0"),
            ),
        )
        .unwrap();

    let result = service.draw_cards_effect(
        &mut game,
        &DrawCardsEffectCommand::new(PlayerId::new("player-1"), PlayerId::new("player-1"), 1),
    );

    assert!(matches!(
        result,
        Err(DomainError::Game(GameError::PriorityWindowOpen { .. }))
    ));
}
