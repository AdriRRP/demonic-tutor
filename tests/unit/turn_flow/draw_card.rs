#![allow(clippy::unwrap_used)]

use crate::support::{
    advance_n_raw, advance_to_player_first_main_satisfying_cleanup, filled_library, land_card,
    setup_two_player_game,
};
use demonictutor::{
    CardInstanceId, DomainError, DrawCardEffectCommand, DrawCardEffectOutcome, GameEndReason,
    GameError, Phase, PlayLandCommand, PlayerId,
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
fn draw_card_effect_works_in_main_phase() {
    let (service, mut game) = setup_two_player_game(
        "game-1",
        filled_library(vec![land_card("forest")], 10),
        filled_library(vec![land_card("mountain")], 10),
    );

    advance_to_player_first_main_satisfying_cleanup(&service, &mut game, "player-2");

    let result = service.draw_card_effect(
        &mut game,
        DrawCardEffectCommand::new(PlayerId::new("player-2")),
    );
    assert!(matches!(result, Ok(DrawCardEffectOutcome::CardDrawn(_))));
}

#[test]
fn draw_card_effect_moves_card_from_library_to_hand() {
    let (service, mut game) = setup_two_player_game(
        "game-1",
        filled_library(vec![land_card("forest")], 10),
        filled_library(vec![land_card("mountain")], 10),
    );

    advance_to_player_first_main_satisfying_cleanup(&service, &mut game, "player-2");

    let hand_before = game.players()[1].hand().cards().len();
    let lib_before = game.players()[1].library().len();

    let outcome = service
        .draw_card_effect(
            &mut game,
            DrawCardEffectCommand::new(PlayerId::new("player-2")),
        )
        .unwrap();
    assert!(matches!(outcome, DrawCardEffectOutcome::CardDrawn(_)));

    let hand_after = game.players()[1].hand().cards().len();
    let lib_after = game.players()[1].library().len();

    assert_eq!(hand_before + 1, hand_after);
    assert_eq!(lib_before - 1, lib_after);
}

#[test]
fn draw_card_effect_emits_event() {
    let (service, mut game) = setup_two_player_game(
        "game-1",
        filled_library(vec![land_card("forest")], 8),
        filled_library(vec![land_card("mountain")], 10),
    );

    advance_to_player_first_main_satisfying_cleanup(&service, &mut game, "player-2");

    let outcome = service
        .draw_card_effect(
            &mut game,
            DrawCardEffectCommand::new(PlayerId::new("player-2")),
        )
        .unwrap();
    let event = match outcome {
        DrawCardEffectOutcome::CardDrawn(event) => Some(event),
        DrawCardEffectOutcome::GameEnded(_) => None,
    };
    assert!(event.is_some());
    let event = event.unwrap();

    assert_eq!(event.player_id.as_str(), "player-2");
}

#[test]
fn draw_card_effect_ends_the_game_when_the_library_is_empty() {
    let mut game = create_game_with_library_cards();
    let service = crate::support::create_service();

    advance_to_player_first_main_satisfying_cleanup(&service, &mut game, "player-2");

    assert!(matches!(
        service.draw_card_effect(
            &mut game,
            DrawCardEffectCommand::new(PlayerId::new("player-2"))
        ),
        Ok(DrawCardEffectOutcome::CardDrawn(_))
    ));
    assert!(matches!(
        service.draw_card_effect(
            &mut game,
            DrawCardEffectCommand::new(PlayerId::new("player-2"))
        ),
        Ok(DrawCardEffectOutcome::CardDrawn(_))
    ));

    let result = service.draw_card_effect(
        &mut game,
        DrawCardEffectCommand::new(PlayerId::new("player-2")),
    );

    let outcome = result.unwrap();
    let game_ended = match outcome {
        DrawCardEffectOutcome::GameEnded(game_ended) => Some(game_ended),
        DrawCardEffectOutcome::CardDrawn(_) => None,
    };
    assert!(game_ended.is_some());
    let game_ended = game_ended.unwrap();
    assert_eq!(game_ended.loser_id, PlayerId::new("player-2"));
    assert_eq!(game_ended.winner_id, PlayerId::new("player-1"));
    assert_eq!(game_ended.reason, GameEndReason::EmptyLibraryDraw);
    assert!(game.is_over());
}

#[test]
fn draw_card_effect_fails_when_not_player_turn() {
    let (service, mut game) = setup_two_player_game(
        "game-1",
        filled_library(vec![land_card("forest")], 8),
        filled_library(vec![land_card("mountain")], 7),
    );

    let result = service.draw_card_effect(
        &mut game,
        DrawCardEffectCommand::new(PlayerId::new("player-2")),
    );

    assert!(matches!(
        result,
        Err(DomainError::Game(GameError::NotYourTurn { .. }))
    ));
}

#[test]
fn draw_card_effect_allows_playing_land_after_draw() {
    let mut game = create_game_with_library_cards();
    let service = crate::support::create_service();

    advance_to_player_first_main_satisfying_cleanup(&service, &mut game, "player-1");

    let outcome = service
        .draw_card_effect(
            &mut game,
            DrawCardEffectCommand::new(PlayerId::new("player-1")),
        )
        .unwrap();
    assert!(matches!(outcome, DrawCardEffectOutcome::CardDrawn(_)));

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

    assert!(matches!(
        service
            .draw_card_effect(
                &mut game,
                DrawCardEffectCommand::new(PlayerId::new("player-2"))
            )
            .unwrap(),
        DrawCardEffectOutcome::CardDrawn(_)
    ));
    assert!(matches!(
        service
            .draw_card_effect(
                &mut game,
                DrawCardEffectCommand::new(PlayerId::new("player-2"))
            )
            .unwrap(),
        DrawCardEffectOutcome::CardDrawn(_)
    ));
    assert!(matches!(
        service
            .draw_card_effect(
                &mut game,
                DrawCardEffectCommand::new(PlayerId::new("player-2"))
            )
            .unwrap(),
        DrawCardEffectOutcome::GameEnded(_)
    ));

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
fn draw_card_effect_fails_outside_main_phases() {
    let (service, mut game) = setup_two_player_game(
        "game-1",
        filled_library(vec![land_card("forest")], 10),
        filled_library(vec![land_card("mountain")], 10),
    );

    advance_n_raw(&service, &mut game, 2);
    assert_eq!(game.phase(), &Phase::Upkeep);

    let result = service.draw_card_effect(
        &mut game,
        DrawCardEffectCommand::new(PlayerId::new("player-1")),
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
