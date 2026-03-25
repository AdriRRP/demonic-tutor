#![allow(clippy::expect_used)]

//! Unit coverage for unit combat deathtouch.

use {
    crate::support,
    demonictutor::{
        CastSpellCommand, DeclareAttackersCommand, DeclareBlockersCommand, KeywordAbility, PlayerId,
    },
};

#[test]
fn one_combat_damage_from_deathtouch_is_lethal() {
    let (service, mut game) = support::setup_two_player_game(
        "game-deathtouch-combat",
        support::filled_library(
            vec![support::creature_card_with_keyword(
                "attacker",
                0,
                1,
                1,
                KeywordAbility::Deathtouch,
            )],
            20,
        ),
        support::filled_library(vec![support::creature_card("blocker", 0, 3, 3)], 20),
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
                vec![(blocker_id.clone(), attacker_id)],
            ),
        )
        .expect("declare blockers should succeed");
    support::close_empty_priority_window(&service, &mut game);

    service
        .resolve_combat_damage(
            &mut game,
            demonictutor::ResolveCombatDamageCommand::new(PlayerId::new("player-1")),
        )
        .expect("combat damage should resolve");

    assert!(game.players()[1].graveyard_contains(&blocker_id));
}

#[test]
fn zero_combat_damage_from_deathtouch_is_not_lethal() {
    let (service, mut game) = support::setup_two_player_game(
        "game-deathtouch-zero-damage",
        support::filled_library(vec![support::creature_card("attacker", 0, 3, 3)], 20),
        support::filled_library(
            vec![support::creature_card_with_keyword(
                "blocker",
                0,
                0,
                3,
                KeywordAbility::Deathtouch,
            )],
            20,
        ),
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
                vec![(blocker_id, attacker_id.clone())],
            ),
        )
        .expect("declare blockers should succeed");
    support::close_empty_priority_window(&service, &mut game);

    service
        .resolve_combat_damage(
            &mut game,
            demonictutor::ResolveCombatDamageCommand::new(PlayerId::new("player-1")),
        )
        .expect("combat damage should resolve");

    assert!(
        game.players()[0].battlefield_card(&attacker_id).is_some(),
        "attacker should survive because deathtouch dealt zero damage"
    );
}
