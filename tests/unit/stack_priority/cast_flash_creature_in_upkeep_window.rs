#![allow(clippy::unwrap_used)]

//! Unit coverage for unit stack priority cast flash creature in upkeep window.

use {
    crate::support::{
        advance_to_phase_satisfying_cleanup, filled_library, flash_creature_card,
        resolve_top_stack_with_passes, setup_two_player_game,
    },
    demonictutor::{CardInstanceId, CastSpellCommand, Phase, PlayerId},
};

#[test]
fn active_player_can_cast_and_resolve_a_flash_creature_in_upkeep() {
    let (service, mut game) = setup_two_player_game(
        "game-flash-upkeep",
        filled_library(vec![flash_creature_card("ambush-viper", 0, 2, 1)], 10),
        filled_library(Vec::new(), 10),
    );

    advance_to_phase_satisfying_cleanup(&service, &mut game, Phase::Upkeep);

    assert_eq!(game.phase(), &Phase::Upkeep);
    assert_eq!(
        game.priority().unwrap().current_holder(),
        &PlayerId::new("player-1")
    );

    let outcome = service
        .cast_spell(
            &mut game,
            CastSpellCommand::new(
                PlayerId::new("player-1"),
                CardInstanceId::new("game-flash-upkeep-player-1-0"),
            ),
        )
        .unwrap();

    assert_eq!(
        outcome.spell_put_on_stack.card_id.as_str(),
        "game-flash-upkeep-player-1-0"
    );
    assert_eq!(game.stack().len(), 1);

    resolve_top_stack_with_passes(&service, &mut game);
    assert_eq!(game.players()[0].battlefield_size(), 1);
    assert_eq!(
        game.players()[0]
            .battlefield_card_at(0)
            .map(|card| card.definition_id().as_str()),
        Some("ambush-viper")
    );
}
