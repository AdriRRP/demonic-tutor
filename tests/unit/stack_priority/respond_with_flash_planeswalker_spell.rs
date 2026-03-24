#![allow(clippy::unwrap_used)]

use crate::support::{
    advance_to_first_main_satisfying_cleanup, filled_library, flash_planeswalker_card,
    instant_card, setup_two_player_game,
};
use demonictutor::{CardInstanceId, CastSpellCommand, PassPriorityCommand, PlayerId};

#[test]
fn opponent_can_respond_to_an_existing_stack_with_a_flash_planeswalker() {
    let (service, mut game) = setup_two_player_game(
        "game-flash-planeswalker-response",
        filled_library(vec![instant_card("opt", 0)], 10),
        filled_library(vec![flash_planeswalker_card("jace", 0)], 10),
    );

    advance_to_first_main_satisfying_cleanup(&service, &mut game);

    service
        .cast_spell(
            &mut game,
            CastSpellCommand::new(
                PlayerId::new("player-1"),
                CardInstanceId::new("game-flash-planeswalker-response-player-1-0"),
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
                CardInstanceId::new("game-flash-planeswalker-response-player-2-0"),
            ),
        )
        .unwrap();

    assert_eq!(game.stack().len(), 2);
    assert_eq!(
        outcome.spell_put_on_stack.card_id,
        CardInstanceId::new("game-flash-planeswalker-response-player-2-0")
    );
}
