#![allow(clippy::unwrap_used)]

//! Unit coverage for unit stack priority respond with second instant in end step window.

use {
    crate::support::{filled_library, instant_card, setup_two_player_game},
    demonictutor::{CardInstanceId, CastSpellCommand, Phase, PlayerId},
};

#[test]
fn responding_player_can_cast_a_second_instant_before_passing_in_end_step() {
    let (service, mut game) = setup_two_player_game(
        "game-respond-second-end-step",
        filled_library(Vec::new(), 10),
        filled_library(
            vec![
                instant_card("end-step-response-a", 0),
                instant_card("end-step-response-b", 0),
            ],
            10,
        ),
    );

    for _ in 0..16 {
        if game.phase() == &Phase::EndStep && game.active_player() == &PlayerId::new("player-1") {
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

    assert_eq!(game.phase(), &Phase::EndStep);
    assert_eq!(
        game.priority().unwrap().current_holder(),
        &PlayerId::new("player-2")
    );

    service
        .cast_spell(
            &mut game,
            CastSpellCommand::new(
                PlayerId::new("player-2"),
                CardInstanceId::new("game-respond-second-end-step-player-2-0"),
            ),
        )
        .unwrap();
    service
        .cast_spell(
            &mut game,
            CastSpellCommand::new(
                PlayerId::new("player-2"),
                CardInstanceId::new("game-respond-second-end-step-player-2-1"),
            ),
        )
        .unwrap();

    assert_eq!(game.stack().len(), 2);
    assert_eq!(
        game.stack().top().unwrap().source_card_id(),
        CardInstanceId::new("game-respond-second-end-step-player-2-1")
    );
    assert_eq!(
        game.priority().unwrap().current_holder(),
        &PlayerId::new("player-2")
    );
}
