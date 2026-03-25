//! Tests the public gameplay projection snapshot.

#![allow(clippy::expect_used)]

use crate::support::{
    advance_to_player_first_main_satisfying_cleanup, create_service, first_hand_card_id,
    forest_card, player, player_deck, player_library,
};
use demonictutor::{
    game_view, DealOpeningHandsCommand, Game, GameId, Phase, PublicGameView, StartGameCommand,
};

fn started_game() -> Game {
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
        .start_game(StartGameCommand::new(GameId::new("game-ui-view"), decks))
        .expect("game should start");
    service
        .deal_opening_hands(&mut game, &DealOpeningHandsCommand::new(libraries))
        .expect("opening hands should be dealt");
    game
}

#[test]
fn game_view_projects_public_state_without_hidden_hand_contents() {
    let game = started_game();

    let view: PublicGameView = game_view(&game);

    assert_eq!(view.phase, Phase::Setup);
    assert_eq!(view.active_player_id.as_str(), "p1");
    assert_eq!(view.players.len(), 2);

    let p1 = view
        .players
        .iter()
        .find(|player| player.player_id.as_str() == "p1")
        .expect("p1 should exist");
    assert!(p1.is_active);
    assert_eq!(p1.hand_count, player(&game, "p1").hand_size());
    assert!(p1.battlefield.is_empty());
    assert!(p1.graveyard.is_empty());
    assert!(p1.exile.is_empty());
}

#[test]
fn game_view_projects_battlefield_cards_after_land_is_played() {
    let service = create_service();
    let mut game = started_game();
    advance_to_player_first_main_satisfying_cleanup(&service, &mut game, "p1");
    let land_id = first_hand_card_id(&game, "p1");

    service
        .play_land(
            &mut game,
            demonictutor::PlayLandCommand::new(demonictutor::PlayerId::new("p1"), land_id.clone()),
        )
        .expect("land should be playable");

    let view = game_view(&game);
    let p1 = view
        .players
        .iter()
        .find(|player| player.player_id.as_str() == "p1")
        .expect("p1 should exist");
    let permanent = p1.battlefield.first().expect("land should be visible");
    assert_eq!(permanent.card_id, land_id);
    assert_eq!(permanent.card_type, demonictutor::CardType::Land);
    assert!(!permanent.permanent_state.tapped);
}
