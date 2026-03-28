//! Unit coverage for lifecycle concede support.

#![allow(clippy::expect_used)]

use crate::support::{
    advance_to_player_first_main_satisfying_cleanup, filled_library, forest_card,
    setup_two_player_game,
};
use demonictutor::{ConcedeCommand, GameEndReason, PlayerId};

#[test]
fn concede_ends_an_active_game_even_with_priority_open() {
    let (service, mut game) = setup_two_player_game(
        "game-lifecycle-concede",
        filled_library(vec![forest_card("p1-forest-a")], 10),
        filled_library(vec![forest_card("p2-forest-a")], 10),
    );

    advance_to_player_first_main_satisfying_cleanup(&service, &mut game, "player-1");
    assert!(game.has_open_priority_window());

    let game_ended = service
        .concede(&mut game, ConcedeCommand::new(PlayerId::new("player-1")))
        .expect("concede should succeed while the game is active");

    assert!(game.is_over());
    assert_eq!(game_ended.reason, GameEndReason::Conceded);
    assert_eq!(game_ended.loser_id, PlayerId::new("player-1"));
    assert_eq!(game_ended.winner_id, PlayerId::new("player-2"));
}
