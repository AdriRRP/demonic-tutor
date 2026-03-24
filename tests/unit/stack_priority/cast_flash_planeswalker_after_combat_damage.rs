#![allow(clippy::unwrap_used)]

//! Unit coverage for unit stack priority cast flash planeswalker after combat damage.

use {
    crate::support::{
        advance_to_player_first_main_satisfying_cleanup, close_empty_priority_window,
        filled_library, flash_planeswalker_card, resolve_top_stack_with_passes,
        setup_two_player_game,
    },
    demonictutor::{
        CardDefinitionId, CardInstanceId, CastSpellCommand, DeclareAttackersCommand, LibraryCard,
        Phase, PlayerId, ResolveCombatDamageCommand,
    },
};

#[test]
fn active_player_can_cast_a_flash_planeswalker_after_combat_damage_resolves() {
    let (service, mut game) = setup_two_player_game(
        "game-flash-planeswalker-after-combat-damage",
        filled_library(
            vec![
                LibraryCard::creature(CardDefinitionId::new("attacker"), 0, 3, 3),
                flash_planeswalker_card("jace", 0),
            ],
            10,
        ),
        filled_library(Vec::new(), 10),
    );

    advance_to_player_first_main_satisfying_cleanup(&service, &mut game, "player-1");
    let attacker_id = CardInstanceId::new("game-flash-planeswalker-after-combat-damage-player-1-0");
    service
        .cast_spell(
            &mut game,
            CastSpellCommand::new(PlayerId::new("player-1"), attacker_id.clone()),
        )
        .unwrap();
    resolve_top_stack_with_passes(&service, &mut game);

    advance_to_player_first_main_satisfying_cleanup(&service, &mut game, "player-2");
    advance_to_player_first_main_satisfying_cleanup(&service, &mut game, "player-1");
    crate::support::advance_turn_raw(&service, &mut game);
    close_empty_priority_window(&service, &mut game);
    crate::support::advance_turn_raw(&service, &mut game);
    service
        .declare_attackers(
            &mut game,
            DeclareAttackersCommand::new(PlayerId::new("player-1"), vec![attacker_id]),
        )
        .unwrap();
    close_empty_priority_window(&service, &mut game);
    crate::support::advance_turn_raw(&service, &mut game);
    service
        .resolve_combat_damage(
            &mut game,
            ResolveCombatDamageCommand::new(PlayerId::new("player-1")),
        )
        .unwrap();

    assert_eq!(game.phase(), &Phase::EndOfCombat);
    assert_eq!(
        game.priority().unwrap().current_holder(),
        &PlayerId::new("player-1")
    );

    service
        .cast_spell(
            &mut game,
            CastSpellCommand::new(
                PlayerId::new("player-1"),
                CardInstanceId::new("game-flash-planeswalker-after-combat-damage-player-1-1"),
            ),
        )
        .unwrap();

    assert_eq!(game.stack().len(), 1);
    assert_eq!(
        game.stack().top().unwrap().source_card_id().as_str(),
        "game-flash-planeswalker-after-combat-damage-player-1-1"
    );
}
