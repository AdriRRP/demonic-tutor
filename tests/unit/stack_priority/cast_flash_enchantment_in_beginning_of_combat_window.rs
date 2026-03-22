#![allow(clippy::unwrap_used)]

use crate::support::{
    advance_to_phase_satisfying_cleanup, close_empty_priority_window, filled_library,
    flash_enchantment_card, resolve_top_stack_with_passes, setup_two_player_game,
};
use demonictutor::{AdvanceTurnCommand, CardInstanceId, CastSpellCommand, Phase, PlayerId};

#[test]
fn active_player_can_cast_and_resolve_a_flash_enchantment_at_beginning_of_combat() {
    let (service, mut game) = setup_two_player_game(
        "game-flash-enchantment-boc",
        filled_library(vec![flash_enchantment_card("battle-aura", 0)], 10),
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
                CardInstanceId::new("game-flash-enchantment-boc-player-1-0"),
            ),
        )
        .unwrap();

    assert_eq!(game.stack().len(), 1);
    resolve_top_stack_with_passes(&service, &mut game);
    assert_eq!(game.players()[0].battlefield_size(), 1);
    assert_eq!(
        game.players()[0]
            .battlefield_card_at(0)
            .map(|card| card.definition_id().as_str()),
        Some("battle-aura")
    );
}
