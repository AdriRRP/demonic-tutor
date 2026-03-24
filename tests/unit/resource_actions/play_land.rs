#![allow(clippy::unwrap_used)]
#![allow(clippy::panic)]

//! Unit coverage for unit resource actions play land.

use {
    crate::support::{
        advance_to_player_first_main_satisfying_cleanup, create_service, filled_library, land_card,
        setup_two_player_game, vanilla_creature,
    },
    demonictutor::{
        CardError, CardInstanceId, DomainError, GameError, PhaseError, PlayLandCommand, PlayerId,
    },
};

fn create_game_with_land_in_hand() -> (demonictutor::Game, CardInstanceId) {
    let service = create_service();
    let mut game = crate::support::start_two_player_game(&service, "game-1");
    let land_card_id = CardInstanceId::new("game-1-player-2-0");

    crate::support::deal_opening_hands(
        &service,
        &mut game,
        filled_library(vec![land_card("forest")], 10),
        filled_library(vec![land_card("mountain")], 10),
    );
    advance_to_player_first_main_satisfying_cleanup(&service, &mut game, "player-2");

    (game, land_card_id)
}

#[test]
fn play_land_moves_card_from_hand_to_battlefield() {
    let (mut game, land_card_id) = create_game_with_land_in_hand();
    let service = create_service();

    let cmd = PlayLandCommand::new(PlayerId::new("player-2"), land_card_id.clone());
    let result = service.play_land(&mut game, cmd);

    assert!(result.is_ok());

    assert_eq!(game.players()[1].battlefield_size(), 1);
    assert_eq!(
        game.players()[1]
            .battlefield_card_at(0)
            .map(demonictutor::CardInstance::id),
        Some(&land_card_id)
    );
}

#[test]
fn play_land_emits_event() {
    let (mut game, land_card_id) = create_game_with_land_in_hand();
    let service = create_service();

    let cmd = PlayLandCommand::new(PlayerId::new("player-2"), land_card_id.clone());
    let event = service.play_land(&mut game, cmd).unwrap();

    assert_eq!(event.card_id, land_card_id);
    assert_eq!(event.player_id.as_str(), "player-2");
}

#[test]
fn play_land_fails_when_card_not_in_hand() {
    let (mut game, _) = create_game_with_land_in_hand();
    let service = create_service();

    let cmd = PlayLandCommand::new(
        PlayerId::new("player-2"),
        CardInstanceId::new("nonexistent-card"),
    );
    let result = service.play_land(&mut game, cmd);

    assert!(matches!(
        result,
        Err(DomainError::Card(CardError::NotInHand { .. }))
    ));
}

#[test]
fn play_land_fails_when_card_is_not_a_land() {
    let (mut game, _) = create_game_with_land_in_hand();
    let service = create_service();

    let cmd = PlayLandCommand::new(
        PlayerId::new("player-2"),
        CardInstanceId::new("game-1-player-2-1"),
    );
    let result = service.play_land(&mut game, cmd);

    assert!(matches!(
        result,
        Err(DomainError::Card(CardError::NotALand { .. }))
    ));
}

#[test]
fn play_land_rejected_non_land_card_stays_in_hand() {
    let (mut game, _) = create_game_with_land_in_hand();
    let service = create_service();

    let hand_before = game.players()[1].hand_size();

    let cmd = PlayLandCommand::new(
        PlayerId::new("player-2"),
        CardInstanceId::new("game-1-player-2-1"),
    );
    let result = service.play_land(&mut game, cmd);

    assert!(result.is_err());
    assert_eq!(game.players()[1].hand_size(), hand_before);
}

#[test]
fn play_land_fails_when_not_player_turn() {
    let (mut game, land_card_id) = create_game_with_land_in_hand();
    let service = create_service();

    let cmd = PlayLandCommand::new(PlayerId::new("player-1"), land_card_id);
    let result = service.play_land(&mut game, cmd);

    assert!(matches!(
        result,
        Err(DomainError::Game(GameError::NotYourTurn { .. }))
    ));
}

#[test]
fn play_land_fails_when_land_already_played_this_turn() {
    let (mut game, land_card_id) = create_game_with_land_in_hand();
    let service = create_service();

    service
        .play_land(
            &mut game,
            PlayLandCommand::new(PlayerId::new("player-2"), land_card_id),
        )
        .unwrap();

    let second_land_id = CardInstanceId::new("game-1-player-2-5");
    let result = service.play_land(
        &mut game,
        PlayLandCommand::new(PlayerId::new("player-2"), second_land_id),
    );

    assert!(matches!(
        result,
        Err(DomainError::Phase(PhaseError::AlreadyPlayedLandThisTurn(
            ..
        )))
    ));
}

#[test]
fn play_land_fails_while_priority_window_is_open() {
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

    let result = service.play_land(
        &mut game,
        PlayLandCommand::new(
            PlayerId::new("player-1"),
            CardInstanceId::new("game-1-player-1-1"),
        ),
    );

    assert!(matches!(
        result,
        Err(DomainError::Game(GameError::PriorityWindowOpen { .. }))
    ));
}
