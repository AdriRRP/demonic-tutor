#![allow(clippy::expect_used)]

//! Unit coverage for indestructible combat behavior.

use {
    crate::support::{
        advance_to_player_first_main_satisfying_cleanup, advance_turn_raw,
        close_empty_priority_window, creature_card, creature_card_with_keyword, filled_library,
        resolve_top_stack_with_passes, setup_two_player_game,
    },
    demonictutor::{
        CardInstanceId, CastSpellCommand, DeclareAttackersCommand, DeclareBlockersCommand,
        KeywordAbility, PlayerId, ResolveCombatDamageCommand,
    },
};

#[test]
fn lethal_combat_damage_does_not_destroy_indestructible_creature() {
    let (service, mut game) = setup_two_player_game(
        "game-indestructible-combat",
        filled_library(
            vec![creature_card_with_keyword(
                "adamant-guardian",
                0,
                2,
                2,
                KeywordAbility::Indestructible,
            )],
            10,
        ),
        filled_library(vec![creature_card("hill-ogre", 0, 3, 3)], 10),
    );

    advance_to_player_first_main_satisfying_cleanup(&service, &mut game, "player-1");

    let attacker_id = CardInstanceId::new("game-indestructible-combat-player-1-0");
    service
        .cast_spell(
            &mut game,
            CastSpellCommand::new(PlayerId::new("player-1"), attacker_id.clone()),
        )
        .expect("casting the indestructible attacker should succeed");
    resolve_top_stack_with_passes(&service, &mut game);

    advance_to_player_first_main_satisfying_cleanup(&service, &mut game, "player-2");
    let blocker_id = CardInstanceId::new("game-indestructible-combat-player-2-0");
    service
        .cast_spell(
            &mut game,
            CastSpellCommand::new(PlayerId::new("player-2"), blocker_id.clone()),
        )
        .expect("casting the blocker should succeed");
    resolve_top_stack_with_passes(&service, &mut game);

    advance_to_player_first_main_satisfying_cleanup(&service, &mut game, "player-1");
    advance_turn_raw(&service, &mut game);
    close_empty_priority_window(&service, &mut game);
    advance_turn_raw(&service, &mut game);

    service
        .declare_attackers(
            &mut game,
            DeclareAttackersCommand::new(PlayerId::new("player-1"), vec![attacker_id.clone()]),
        )
        .expect("declaring attackers should succeed");
    close_empty_priority_window(&service, &mut game);
    service
        .declare_blockers(
            &mut game,
            DeclareBlockersCommand::new(
                PlayerId::new("player-2"),
                vec![(blocker_id, attacker_id.clone())],
            ),
        )
        .expect("declaring blockers should succeed");
    close_empty_priority_window(&service, &mut game);

    let outcome = service
        .resolve_combat_damage(
            &mut game,
            ResolveCombatDamageCommand::new(PlayerId::new("player-1")),
        )
        .expect("combat damage should resolve");

    assert!(outcome
        .creatures_died
        .iter()
        .all(|event| event.card_id != attacker_id));
    assert!(game.players()[0].battlefield_card(&attacker_id).is_some());
    assert_eq!(
        game.players()[0]
            .battlefield_card(&attacker_id)
            .expect("indestructible attacker should remain on the battlefield")
            .damage(),
        3
    );
}
