#![allow(clippy::unwrap_used)]

//! Unit coverage for unit combat combat subphases.

use {
    crate::support::{
        advance_to_player_first_main_satisfying_cleanup, advance_turn_raw,
        close_empty_priority_window, filled_library, resolve_top_stack_with_passes,
        setup_two_player_game,
    },
    demonictutor::{
        CardDefinitionId, CardInstanceId, CastSpellCommand, DeclareAttackersCommand,
        DeclareBlockersCommand, LibraryCard, Phase, PlayerId, ResolveCombatDamageCommand,
    },
};

#[test]
fn combat_uses_explicit_subphases_in_order() {
    let (service, mut game) = setup_two_player_game(
        "game-combat-subphases",
        filled_library(
            vec![LibraryCard::creature(
                CardDefinitionId::new("attacker"),
                0,
                2,
                2,
            )],
            10,
        ),
        filled_library(
            vec![LibraryCard::creature(
                CardDefinitionId::new("blocker"),
                0,
                2,
                2,
            )],
            10,
        ),
    );

    let attacker_id = CardInstanceId::new("game-combat-subphases-player-1-0");
    let blocker_id = CardInstanceId::new("game-combat-subphases-player-2-0");

    advance_to_player_first_main_satisfying_cleanup(&service, &mut game, "player-1");
    service
        .cast_spell(
            &mut game,
            CastSpellCommand::new(PlayerId::new("player-1"), attacker_id.clone()),
        )
        .unwrap();
    resolve_top_stack_with_passes(&service, &mut game);

    advance_to_player_first_main_satisfying_cleanup(&service, &mut game, "player-2");
    service
        .cast_spell(
            &mut game,
            CastSpellCommand::new(PlayerId::new("player-2"), blocker_id.clone()),
        )
        .unwrap();
    resolve_top_stack_with_passes(&service, &mut game);

    advance_to_player_first_main_satisfying_cleanup(&service, &mut game, "player-1");
    advance_turn_raw(&service, &mut game);
    assert_eq!(game.phase(), &Phase::BeginningOfCombat);
    assert_eq!(
        game.priority().unwrap().current_holder(),
        &PlayerId::new("player-1")
    );

    close_empty_priority_window(&service, &mut game);
    advance_turn_raw(&service, &mut game);
    assert_eq!(game.phase(), &Phase::DeclareAttackers);

    service
        .declare_attackers(
            &mut game,
            DeclareAttackersCommand::new(PlayerId::new("player-1"), vec![attacker_id.clone()]),
        )
        .unwrap();
    assert_eq!(game.phase(), &Phase::DeclareBlockers);
    assert_eq!(
        game.priority().unwrap().current_holder(),
        &PlayerId::new("player-1")
    );

    close_empty_priority_window(&service, &mut game);
    service
        .declare_blockers(
            &mut game,
            DeclareBlockersCommand::new(PlayerId::new("player-2"), vec![(blocker_id, attacker_id)]),
        )
        .unwrap();
    assert_eq!(game.phase(), &Phase::CombatDamage);
    assert_eq!(
        game.priority().unwrap().current_holder(),
        &PlayerId::new("player-1")
    );

    close_empty_priority_window(&service, &mut game);
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
}
