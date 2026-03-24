#![allow(clippy::unwrap_used)]

//! Unit coverage for unit stack priority cast second instant in upkeep window.

use {
    crate::support::{filled_library, instant_card, setup_two_player_game},
    demonictutor::{CardInstanceId, CastSpellCommand, Phase, PlayerId},
};

#[test]
fn active_player_can_cast_a_second_instant_before_passing_in_upkeep() {
    let (service, mut game) = setup_two_player_game(
        "game-second-upkeep-instant",
        filled_library(
            vec![instant_card("upkeep-a", 0), instant_card("upkeep-b", 0)],
            10,
        ),
        filled_library(Vec::new(), 10),
    );

    for _ in 0..8 {
        if game.phase() == &Phase::Upkeep && game.active_player() == &PlayerId::new("player-1") {
            break;
        }

        crate::support::advance_turn_raw(&service, &mut game);
    }

    assert_eq!(game.phase(), &Phase::Upkeep);
    assert_eq!(
        game.priority().unwrap().current_holder(),
        &PlayerId::new("player-1")
    );

    service
        .cast_spell(
            &mut game,
            CastSpellCommand::new(
                PlayerId::new("player-1"),
                CardInstanceId::new("game-second-upkeep-instant-player-1-0"),
            ),
        )
        .unwrap();
    service
        .cast_spell(
            &mut game,
            CastSpellCommand::new(
                PlayerId::new("player-1"),
                CardInstanceId::new("game-second-upkeep-instant-player-1-1"),
            ),
        )
        .unwrap();

    assert_eq!(game.stack().len(), 2);
    assert_eq!(
        game.stack().top().unwrap().source_card_id(),
        &CardInstanceId::new("game-second-upkeep-instant-player-1-1")
    );
    assert_eq!(
        game.priority().unwrap().current_holder(),
        &PlayerId::new("player-1")
    );
}
