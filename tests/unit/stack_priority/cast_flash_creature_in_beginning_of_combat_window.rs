#![allow(clippy::unwrap_used)]

//! Unit coverage for unit stack priority cast flash creature in beginning of combat window.

use {
    crate::support::{
        advance_to_phase_satisfying_cleanup, close_empty_priority_window, filled_library,
        flash_creature_card, setup_two_player_game,
    },
    demonictutor::{AdvanceTurnCommand, CardInstanceId, CastSpellCommand, Phase, PlayerId},
};

#[test]
fn active_player_can_cast_a_flash_creature_at_beginning_of_combat() {
    let (service, mut game) = setup_two_player_game(
        "game-flash-boc",
        filled_library(vec![flash_creature_card("ambush-viper", 0, 2, 1)], 10),
        filled_library(Vec::new(), 10),
    );

    advance_to_phase_satisfying_cleanup(&service, &mut game, Phase::FirstMain);
    close_empty_priority_window(&service, &mut game);
    service
        .advance_turn(&mut game, AdvanceTurnCommand::new())
        .unwrap();

    assert_eq!(game.phase(), &Phase::BeginningOfCombat);
    assert_eq!(
        game.priority().unwrap().current_holder(),
        &PlayerId::new("player-1")
    );

    service
        .cast_spell(
            &mut game,
            CastSpellCommand::new(
                PlayerId::new("player-1"),
                CardInstanceId::new("game-flash-boc-player-1-0"),
            ),
        )
        .unwrap();

    assert_eq!(game.stack().len(), 1);
    assert_eq!(
        game.stack().top().unwrap().source_card_id().as_str(),
        "game-flash-boc-player-1-0"
    );
}
