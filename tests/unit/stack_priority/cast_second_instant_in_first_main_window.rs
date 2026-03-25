#![allow(clippy::unwrap_used)]

//! Unit coverage for unit stack priority cast second instant in first main window.

use {
    crate::support::{
        advance_to_first_main_satisfying_cleanup, filled_library, instant_card,
        setup_two_player_game,
    },
    demonictutor::{CardInstanceId, CastSpellCommand, Phase, PlayerId},
};

#[test]
fn active_player_can_cast_a_second_instant_before_passing_in_first_main() {
    let (service, mut game) = setup_two_player_game(
        "game-second-first-main-instant",
        filled_library(
            vec![instant_card("main-a", 0), instant_card("main-b", 0)],
            10,
        ),
        filled_library(Vec::new(), 10),
    );

    advance_to_first_main_satisfying_cleanup(&service, &mut game);

    assert_eq!(game.phase(), &Phase::FirstMain);
    assert_eq!(
        game.priority().unwrap().current_holder(),
        &PlayerId::new("player-1")
    );

    service
        .cast_spell(
            &mut game,
            CastSpellCommand::new(
                PlayerId::new("player-1"),
                CardInstanceId::new("game-second-first-main-instant-player-1-0"),
            ),
        )
        .unwrap();
    service
        .cast_spell(
            &mut game,
            CastSpellCommand::new(
                PlayerId::new("player-1"),
                CardInstanceId::new("game-second-first-main-instant-player-1-1"),
            ),
        )
        .unwrap();

    assert_eq!(game.stack().len(), 2);
    assert_eq!(
        game.stack().top().unwrap().source_card_id(),
        CardInstanceId::new("game-second-first-main-instant-player-1-1")
    );
    assert_eq!(
        game.priority().unwrap().current_holder(),
        &PlayerId::new("player-1")
    );
}
