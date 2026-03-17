#![allow(clippy::unwrap_used)]

use crate::support::{filled_library, instant_card, setup_two_player_game};
use demonictutor::{CardInstanceId, CastSpellCommand, Phase, PlayerId};

#[test]
fn responding_player_can_cast_a_second_instant_before_passing_in_draw() {
    let (service, mut game) = setup_two_player_game(
        "game-respond-second-draw",
        filled_library(Vec::new(), 10),
        filled_library(
            vec![
                instant_card("draw-response-a", 0),
                instant_card("draw-response-b", 0),
            ],
            10,
        ),
    );

    for _ in 0..8 {
        if game.phase() == &Phase::Draw && game.active_player() == &PlayerId::new("player-1") {
            break;
        }

        crate::support::advance_turn_raw(&service, &mut game);
    }

    service
        .pass_priority(
            &mut game,
            demonictutor::PassPriorityCommand::new(PlayerId::new("player-1")),
        )
        .unwrap();

    assert_eq!(game.phase(), &Phase::Draw);
    assert_eq!(
        game.priority().unwrap().current_holder(),
        &PlayerId::new("player-2")
    );

    service
        .cast_spell(
            &mut game,
            CastSpellCommand::new(
                PlayerId::new("player-2"),
                CardInstanceId::new("game-respond-second-draw-player-2-0"),
            ),
        )
        .unwrap();
    service
        .cast_spell(
            &mut game,
            CastSpellCommand::new(
                PlayerId::new("player-2"),
                CardInstanceId::new("game-respond-second-draw-player-2-1"),
            ),
        )
        .unwrap();

    assert_eq!(game.stack().len(), 2);
    assert_eq!(
        game.stack().top().unwrap().source_card_id(),
        &CardInstanceId::new("game-respond-second-draw-player-2-1")
    );
    assert_eq!(
        game.priority().unwrap().current_holder(),
        &PlayerId::new("player-2")
    );
}
