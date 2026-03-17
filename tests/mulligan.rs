#![allow(clippy::unwrap_used)]

mod support;

use demonictutor::{
    AdvanceTurnCommand, DomainError, GameError, MulliganCommand, Phase, PhaseError, PlayerId,
};
use support::{create_service, creature_library, setup_two_player_game};

#[test]
fn mulligan_hand_contains_exactly_seven_cards_after_mulligan() {
    let (service, mut game) =
        setup_two_player_game("game-1", creature_library(14), creature_library(14));

    assert_eq!(game.players()[0].hand().cards().len(), 7);

    service
        .mulligan(&mut game, MulliganCommand::new(PlayerId::new("player-1")))
        .unwrap();

    assert_eq!(game.players()[0].hand().cards().len(), 7);
}

#[test]
fn mulligan_succeeds() {
    let (service, mut game) =
        setup_two_player_game("game-1", creature_library(14), creature_library(14));

    assert_eq!(game.phase(), &Phase::Setup);
    assert!(service
        .mulligan(&mut game, MulliganCommand::new(PlayerId::new("player-1")))
        .is_ok());
}

#[test]
fn mulligan_fails_already_used() {
    let (service, mut game) =
        setup_two_player_game("game-1", creature_library(14), creature_library(14));

    service
        .mulligan(&mut game, MulliganCommand::new(PlayerId::new("player-1")))
        .unwrap();

    let result = service.mulligan(&mut game, MulliganCommand::new(PlayerId::new("player-1")));

    assert_eq!(
        result.unwrap_err(),
        DomainError::Game(GameError::MulliganAlreadyUsed(PlayerId::new("player-1")))
    );
}

#[test]
fn mulligan_fails_not_enough_cards() {
    let service = create_service();
    let mut game = support::start_two_player_game(&service, "game-1");

    let result = service.mulligan(&mut game, MulliganCommand::new(PlayerId::new("player-1")));

    assert!(matches!(
        result.unwrap_err(),
        DomainError::Game(GameError::NotEnoughCardsInLibrary {
            available: 0,
            requested: 7,
            ..
        })
    ));
}

#[test]
fn mulligan_fails_not_setup_phase() {
    let (service, mut game) =
        setup_two_player_game("game-1", creature_library(14), creature_library(14));

    service
        .advance_turn(&mut game, AdvanceTurnCommand::new())
        .unwrap();

    assert_eq!(game.phase(), &Phase::Untap);

    let result = service.mulligan(&mut game, MulliganCommand::new(PlayerId::new("player-1")));

    assert_eq!(
        result.unwrap_err(),
        DomainError::Phase(PhaseError::InvalidForMulligan)
    );
}
