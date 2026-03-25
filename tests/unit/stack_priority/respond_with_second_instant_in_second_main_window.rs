#![allow(clippy::unwrap_used)]

//! Unit coverage for unit stack priority respond with second instant in second main window.

use {
    crate::support::{filled_library, instant_card, setup_two_player_game},
    demonictutor::{CardInstanceId, CastSpellCommand, Phase, PlayerId},
};

#[test]
fn responding_player_can_cast_a_second_instant_before_passing_in_second_main() {
    let (service, mut game) = setup_two_player_game(
        "game-respond-second-second-main",
        filled_library(Vec::new(), 10),
        filled_library(
            vec![
                instant_card("second-main-response-a", 0),
                instant_card("second-main-response-b", 0),
            ],
            10,
        ),
    );

    for _ in 0..16 {
        if game.phase() == &Phase::SecondMain && game.active_player() == &PlayerId::new("player-1")
        {
            break;
        }

        crate::support::advance_turn_satisfying_cleanup(&service, &mut game);
    }

    service
        .pass_priority(
            &mut game,
            demonictutor::PassPriorityCommand::new(PlayerId::new("player-1")),
        )
        .unwrap();

    assert_eq!(game.phase(), &Phase::SecondMain);
    assert_eq!(
        game.priority().unwrap().current_holder(),
        &PlayerId::new("player-2")
    );

    service
        .cast_spell(
            &mut game,
            CastSpellCommand::new(
                PlayerId::new("player-2"),
                CardInstanceId::new("game-respond-second-second-main-player-2-0"),
            ),
        )
        .unwrap();
    service
        .cast_spell(
            &mut game,
            CastSpellCommand::new(
                PlayerId::new("player-2"),
                CardInstanceId::new("game-respond-second-second-main-player-2-1"),
            ),
        )
        .unwrap();

    assert_eq!(game.stack().len(), 2);
    assert_eq!(
        game.stack().top().unwrap().source_card_id(),
        CardInstanceId::new("game-respond-second-second-main-player-2-1")
    );
    assert_eq!(
        game.priority().unwrap().current_holder(),
        &PlayerId::new("player-2")
    );
}
