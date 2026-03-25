//! Tests the deterministic public command-result envelope.

#![allow(clippy::expect_used, clippy::panic)]

use crate::support::{
    advance_to_player_first_main_satisfying_cleanup, create_service, first_hand_card_id,
    forest_card, player_deck, player_library,
};
use demonictutor::{
    DealOpeningHandsCommand, GameId, PlayLandCommand, PlayerId, PublicCommandStatus,
    PublicGameCommand, StartGameCommand,
};

fn game_in_first_main() -> (crate::support::TestService, demonictutor::Game) {
    let service = create_service();
    let libraries = vec![
        player_library(
            "p1",
            vec![
                forest_card("p1-forest-a"),
                forest_card("p1-forest-b"),
                forest_card("p1-forest-c"),
                forest_card("p1-forest-d"),
                forest_card("p1-forest-e"),
                forest_card("p1-forest-f"),
                forest_card("p1-forest-g"),
                forest_card("p1-forest-h"),
                forest_card("p1-forest-i"),
                forest_card("p1-forest-j"),
            ],
        ),
        player_library(
            "p2",
            vec![
                forest_card("p2-forest-a"),
                forest_card("p2-forest-b"),
                forest_card("p2-forest-c"),
                forest_card("p2-forest-d"),
                forest_card("p2-forest-e"),
                forest_card("p2-forest-f"),
                forest_card("p2-forest-g"),
                forest_card("p2-forest-h"),
                forest_card("p2-forest-i"),
                forest_card("p2-forest-j"),
            ],
        ),
    ];
    let decks = vec![player_deck("p1", "d1"), player_deck("p2", "d2")];

    let (mut game, _) = service
        .start_game(StartGameCommand::new(
            GameId::new("game-public-command"),
            decks,
        ))
        .expect("game should start");
    service
        .deal_opening_hands(&mut game, &DealOpeningHandsCommand::new(libraries))
        .expect("opening hands should be dealt");
    advance_to_player_first_main_satisfying_cleanup(&service, &mut game, "p1");
    (service, game)
}

#[test]
fn execute_public_command_returns_applied_status_events_and_next_snapshot() {
    let (service, mut game) = game_in_first_main();
    let land_id = first_hand_card_id(&game, "p1");

    let result = service.execute_public_command(
        &mut game,
        PublicGameCommand::PlayLand(PlayLandCommand::new(PlayerId::new("p1"), land_id.clone())),
    );

    assert!(matches!(result.status, PublicCommandStatus::Applied));
    assert!(!result.emitted_events.is_empty());
    let p1 = result
        .game
        .players
        .iter()
        .find(|player| player.player_id.as_str() == "p1")
        .expect("p1 should exist");
    assert!(p1.battlefield.iter().any(|card| card.card_id == land_id));
}

#[test]
fn execute_public_command_returns_rejected_status_and_preserves_follow_up_contract() {
    let (service, mut game) = game_in_first_main();
    let land_id = first_hand_card_id(&game, "p1");

    let result = service.execute_public_command(
        &mut game,
        PublicGameCommand::PlayLand(PlayLandCommand::new(PlayerId::new("p2"), land_id)),
    );

    match result.status {
        PublicCommandStatus::Rejected(rejection) => {
            assert!(!rejection.message.is_empty());
        }
        PublicCommandStatus::Applied => panic!("command should have been rejected"),
    }
    assert!(result.emitted_events.is_empty());
    assert!(!result.legal_actions.is_empty());
}
