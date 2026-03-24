#![allow(clippy::expect_used)]

//! Unit coverage for unit combat first strike.

use {
    crate::support,
    demonictutor::{
        CastSpellCommand, DeclareAttackersCommand, DeclareBlockersCommand, KeywordAbility, PlayerId,
    },
};

#[test]
fn first_strike_attacker_kills_blocker_before_normal_retaliation() {
    let (service, mut game) = support::setup_two_player_game(
        "game-first-strike-combat",
        support::filled_library(
            vec![support::creature_card_with_keyword(
                "attacker",
                0,
                2,
                2,
                KeywordAbility::FirstStrike,
            )],
            20,
        ),
        support::filled_library(vec![support::creature_card("blocker", 0, 2, 2)], 20),
    );

    support::advance_to_player_first_main_satisfying_cleanup(&service, &mut game, "player-1");
    let attacker_id = game.players()[0]
        .hand_card_at(0)
        .expect("attacker should exist in hand")
        .id()
        .clone();
    service
        .cast_spell(
            &mut game,
            CastSpellCommand::new(PlayerId::new("player-1"), attacker_id.clone()),
        )
        .expect("attacker cast should succeed");
    support::resolve_top_stack_with_passes(&service, &mut game);

    support::advance_to_player_first_main_satisfying_cleanup(&service, &mut game, "player-2");
    let blocker_id = game.players()[1]
        .hand_card_at(0)
        .expect("blocker should exist in hand")
        .id()
        .clone();
    service
        .cast_spell(
            &mut game,
            CastSpellCommand::new(PlayerId::new("player-2"), blocker_id.clone()),
        )
        .expect("blocker cast should succeed");
    support::resolve_top_stack_with_passes(&service, &mut game);

    support::advance_to_player_first_main_satisfying_cleanup(&service, &mut game, "player-1");
    support::advance_turn_raw(&service, &mut game);
    support::close_empty_priority_window(&service, &mut game);
    support::advance_turn_raw(&service, &mut game);

    service
        .declare_attackers(
            &mut game,
            DeclareAttackersCommand::new(PlayerId::new("player-1"), vec![attacker_id.clone()]),
        )
        .expect("declare attackers should succeed");
    support::close_empty_priority_window(&service, &mut game);

    service
        .declare_blockers(
            &mut game,
            DeclareBlockersCommand::new(
                PlayerId::new("player-2"),
                vec![(blocker_id.clone(), attacker_id.clone())],
            ),
        )
        .expect("declare blockers should succeed");
    support::close_empty_priority_window(&service, &mut game);

    let outcome = service
        .resolve_combat_damage(
            &mut game,
            demonictutor::ResolveCombatDamageCommand::new(PlayerId::new("player-1")),
        )
        .expect("combat damage should resolve");

    let attacker = game.players()[0]
        .battlefield_card(&attacker_id)
        .expect("attacker should survive on battlefield");
    assert_eq!(attacker.damage(), 0);
    assert!(game.players()[1].graveyard_contains(&blocker_id));
    assert!(outcome
        .combat_damage_resolved
        .damage_events
        .iter()
        .any(|event| event.source == attacker_id && event.damage_amount == 2));
}
