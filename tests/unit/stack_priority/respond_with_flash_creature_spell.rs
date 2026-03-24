#![allow(clippy::unwrap_used)]

//! Unit coverage for unit stack priority respond with flash creature spell.

use {
    crate::support::{
        advance_to_first_main_satisfying_cleanup, filled_library, flash_creature_card,
        instant_card, setup_two_player_game,
    },
    demonictutor::{CardInstanceId, CastSpellCommand, PassPriorityCommand, PlayerId},
};

#[test]
fn opponent_can_respond_to_an_existing_stack_with_a_flash_creature() {
    let (service, mut game) = setup_two_player_game(
        "game-flash-response",
        filled_library(vec![instant_card("opt", 0)], 10),
        filled_library(vec![flash_creature_card("ambush-viper", 0, 2, 1)], 10),
    );

    advance_to_first_main_satisfying_cleanup(&service, &mut game);

    service
        .cast_spell(
            &mut game,
            CastSpellCommand::new(
                PlayerId::new("player-1"),
                CardInstanceId::new("game-flash-response-player-1-0"),
            ),
        )
        .unwrap();

    service
        .pass_priority(
            &mut game,
            PassPriorityCommand::new(PlayerId::new("player-1")),
        )
        .unwrap();

    let outcome = service
        .cast_spell(
            &mut game,
            CastSpellCommand::new(
                PlayerId::new("player-2"),
                CardInstanceId::new("game-flash-response-player-2-0"),
            ),
        )
        .unwrap();

    assert_eq!(
        outcome.spell_put_on_stack.card_id.as_str(),
        "game-flash-response-player-2-0"
    );
    assert_eq!(game.stack().len(), 2);
    assert_eq!(
        game.priority().unwrap().current_holder(),
        &PlayerId::new("player-2")
    );
}
