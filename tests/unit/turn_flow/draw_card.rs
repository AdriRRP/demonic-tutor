#![allow(clippy::unwrap_used)]

use crate::support::{
    advance_n, advance_to_player_first_main, filled_library, land_card, setup_two_player_game,
};
use demonictutor::{
    CardInstanceId, DomainError, DrawCardCommand, GameError, PlayLandCommand, PlayerId,
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
fn draw_card_works_in_main_phase() {
    let (service, mut game) = setup_two_player_game(
        "game-1",
        filled_library(vec![land_card("forest")], 10),
        filled_library(vec![land_card("mountain")], 10),
    );

    advance_to_player_first_main(&service, &mut game, "player-2");

    let result = service.draw_card(&mut game, DrawCardCommand::new(PlayerId::new("player-2")));
    assert!(result.is_ok());
}

#[test]
fn draw_card_moves_card_from_library_to_hand() {
    let (service, mut game) = setup_two_player_game(
        "game-1",
        filled_library(vec![land_card("forest")], 10),
        filled_library(vec![land_card("mountain")], 10),
    );

    advance_to_player_first_main(&service, &mut game, "player-2");

    let hand_before = game.players()[1].hand().cards().len();
    let lib_before = game.players()[1].library().len();

    service
        .draw_card(&mut game, DrawCardCommand::new(PlayerId::new("player-2")))
        .unwrap();

    let hand_after = game.players()[1].hand().cards().len();
    let lib_after = game.players()[1].library().len();

    assert_eq!(hand_before + 1, hand_after);
    assert_eq!(lib_before - 1, lib_after);
}

#[test]
fn draw_card_emits_event() {
    let (service, mut game) = setup_two_player_game(
        "game-1",
        filled_library(vec![land_card("forest")], 8),
        filled_library(vec![land_card("mountain")], 10),
    );

    advance_to_player_first_main(&service, &mut game, "player-2");

    let event = service
        .draw_card(&mut game, DrawCardCommand::new(PlayerId::new("player-2")))
        .unwrap();

    assert_eq!(event.player_id.as_str(), "player-2");
}

#[test]
fn draw_card_fails_when_not_enough_cards() {
    let mut game = create_game_with_library_cards();
    let service = crate::support::create_service();

    advance_to_player_first_main(&service, &mut game, "player-2");

    assert!(service
        .draw_card(&mut game, DrawCardCommand::new(PlayerId::new("player-2")))
        .is_ok());
    assert!(service
        .draw_card(&mut game, DrawCardCommand::new(PlayerId::new("player-2")))
        .is_ok());

    let result = service.draw_card(&mut game, DrawCardCommand::new(PlayerId::new("player-2")));

    assert!(matches!(
        result,
        Err(DomainError::Game(GameError::NotEnoughCardsInLibrary { .. }))
    ));
}

#[test]
fn draw_card_fails_when_not_player_turn() {
    let (service, mut game) = setup_two_player_game(
        "game-1",
        filled_library(vec![land_card("forest")], 8),
        filled_library(vec![land_card("mountain")], 7),
    );

    let result = service.draw_card(&mut game, DrawCardCommand::new(PlayerId::new("player-2")));

    assert!(matches!(
        result,
        Err(DomainError::Game(GameError::NotYourTurn { .. }))
    ));
}

#[test]
fn draw_card_allows_playing_land_after_draw() {
    let mut game = create_game_with_library_cards();
    let service = crate::support::create_service();

    advance_n(&service, &mut game, 3);

    service
        .draw_card(&mut game, DrawCardCommand::new(PlayerId::new("player-1")))
        .unwrap();

    advance_to_player_first_main(&service, &mut game, "player-2");

    let result = service.play_land(
        &mut game,
        PlayLandCommand::new(
            PlayerId::new("player-2"),
            CardInstanceId::new("game-1-player-2-0"),
        ),
    );

    assert!(result.is_ok());
}
