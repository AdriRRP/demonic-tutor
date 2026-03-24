#![allow(clippy::unwrap_used)]

//! Unit coverage for unit stack priority respond with second instant in beginning of combat window.

use {
    crate::support::{filled_library, instant_card, setup_two_player_game},
    demonictutor::{CardInstanceId, CastSpellCommand, Phase, PlayerId},
};

#[test]
fn responding_player_can_cast_a_second_instant_before_passing_at_beginning_of_combat() {
    let (service, mut game) = setup_two_player_game(
        "game-respond-second-beginning-combat",
        filled_library(Vec::new(), 10),
        filled_library(
            vec![
                instant_card("beginning-combat-response-a", 0),
                instant_card("beginning-combat-response-b", 0),
            ],
            10,
        ),
    );

    crate::support::advance_to_player_first_main_satisfying_cleanup(
        &service, &mut game, "player-1",
    );
    crate::support::close_empty_priority_window(&service, &mut game);
    crate::support::advance_turn_raw(&service, &mut game);

    service
        .pass_priority(
            &mut game,
            demonictutor::PassPriorityCommand::new(PlayerId::new("player-1")),
        )
        .unwrap();

    assert_eq!(game.phase(), &Phase::BeginningOfCombat);
    assert_eq!(
        game.priority().unwrap().current_holder(),
        &PlayerId::new("player-2")
    );

    service
        .cast_spell(
            &mut game,
            CastSpellCommand::new(
                PlayerId::new("player-2"),
                CardInstanceId::new("game-respond-second-beginning-combat-player-2-0"),
            ),
        )
        .unwrap();
    service
        .cast_spell(
            &mut game,
            CastSpellCommand::new(
                PlayerId::new("player-2"),
                CardInstanceId::new("game-respond-second-beginning-combat-player-2-1"),
            ),
        )
        .unwrap();

    assert_eq!(game.stack().len(), 2);
    assert_eq!(
        game.stack().top().unwrap().source_card_id(),
        &CardInstanceId::new("game-respond-second-beginning-combat-player-2-1")
    );
    assert_eq!(
        game.priority().unwrap().current_holder(),
        &PlayerId::new("player-2")
    );
}
