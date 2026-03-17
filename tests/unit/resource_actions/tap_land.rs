#![allow(clippy::unwrap_used)]

use crate::support::{
    advance_to_first_main_satisfying_cleanup, advance_turn_raw, create_service, filled_library,
    land_card,
};
use demonictutor::{
    CardError, CardInstanceId, DomainError, GameService, InMemoryEventBus, InMemoryEventStore,
    Phase, PlayLandCommand, PlayerId, TapLandCommand,
};

fn create_game_with_land_on_battlefield() -> (
    demonictutor::Game,
    GameService<InMemoryEventStore, InMemoryEventBus>,
) {
    let service = create_service();
    let mut game = crate::support::start_two_player_game(&service, "game-1");

    crate::support::deal_opening_hands(
        &service,
        &mut game,
        filled_library(vec![land_card("forest")], 10),
        filled_library(vec![land_card("mountain")], 10),
    );

    advance_to_first_main_satisfying_cleanup(&service, &mut game);
    service
        .play_land(
            &mut game,
            PlayLandCommand::new(
                PlayerId::new("player-1"),
                CardInstanceId::new("game-1-player-1-0"),
            ),
        )
        .unwrap();

    (game, service)
}

#[test]
fn players_start_with_zero_mana() {
    let service = create_service();
    let game = crate::support::start_two_player_game(&service, "game-1");

    assert_eq!(game.players()[0].mana(), 0);
    assert_eq!(game.players()[1].mana(), 0);
}

#[test]
fn tap_land_adds_mana() {
    let (mut game, service) = create_game_with_land_on_battlefield();

    let result = service.tap_land(
        &mut game,
        TapLandCommand::new(
            PlayerId::new("player-1"),
            CardInstanceId::new("game-1-player-1-0"),
        ),
    );

    assert!(result.is_ok());
    let (_, mana_event) = result.unwrap();
    assert_eq!(mana_event.amount, 1);
    assert_eq!(mana_event.new_mana_total, 1);
    assert_eq!(game.players()[0].mana(), 1);
}

#[test]
fn tap_land_fails_for_untapped_land() {
    let (mut game, service) = create_game_with_land_on_battlefield();

    assert!(service
        .tap_land(
            &mut game,
            TapLandCommand::new(
                PlayerId::new("player-1"),
                CardInstanceId::new("game-1-player-1-0"),
            ),
        )
        .is_ok());

    let result = service.tap_land(
        &mut game,
        TapLandCommand::new(
            PlayerId::new("player-1"),
            CardInstanceId::new("game-1-player-1-0"),
        ),
    );

    assert!(matches!(
        result,
        Err(DomainError::Card(CardError::AlreadyTapped { .. }))
    ));
}

#[test]
fn tap_land_fails_for_non_land_card() {
    let (mut game, service) = create_game_with_land_on_battlefield();

    let result = service.tap_land(
        &mut game,
        TapLandCommand::new(
            PlayerId::new("player-1"),
            CardInstanceId::new("game-1-player-1-1"),
        ),
    );

    assert!(matches!(
        result,
        Err(DomainError::Card(CardError::NotOnBattlefield { .. }))
    ));
}

#[test]
fn tap_land_fails_for_unknown_card() {
    let (service, mut game) = crate::support::setup_two_player_game(
        "game-1",
        filled_library(vec![land_card("forest")], 10),
        filled_library(vec![land_card("mountain")], 10),
    );
    advance_to_first_main_satisfying_cleanup(&service, &mut game);

    let result = service.tap_land(
        &mut game,
        TapLandCommand::new(
            PlayerId::new("player-1"),
            CardInstanceId::new("nonexistent"),
        ),
    );

    assert!(matches!(
        result,
        Err(DomainError::Card(CardError::NotOnBattlefield { .. }))
    ));
}

#[test]
fn tap_land_fails_when_not_players_turn() {
    let (mut game, service) = create_game_with_land_on_battlefield();

    let result = service.tap_land(
        &mut game,
        TapLandCommand::new(
            PlayerId::new("player-2"),
            CardInstanceId::new("game-1-player-1-0"),
        ),
    );

    assert!(matches!(
        result,
        Err(DomainError::Game(
            demonictutor::GameError::NotYourTurn { .. }
        ))
    ));
}

#[test]
fn tap_land_fails_outside_main_phases() {
    let (mut game, service) = create_game_with_land_on_battlefield();

    advance_turn_raw(&service, &mut game);

    assert_eq!(game.phase(), &Phase::BeginningOfCombat);

    let result = service.tap_land(
        &mut game,
        TapLandCommand::new(
            PlayerId::new("player-1"),
            CardInstanceId::new("game-1-player-1-0"),
        ),
    );

    assert!(matches!(
        result,
        Err(DomainError::Phase(
            demonictutor::PhaseError::InvalidForPlayingCard {
                phase: Phase::BeginningOfCombat
            }
        ))
    ));
}

#[test]
fn advance_turn_clears_mana_pools() {
    let (mut game, service) = create_game_with_land_on_battlefield();

    service
        .tap_land(
            &mut game,
            TapLandCommand::new(
                PlayerId::new("player-1"),
                CardInstanceId::new("game-1-player-1-0"),
            ),
        )
        .unwrap();

    assert_eq!(game.players()[0].mana(), 1);

    advance_turn_raw(&service, &mut game);

    assert_eq!(game.players()[0].mana(), 0);
    assert_eq!(game.players()[1].mana(), 0);
}
