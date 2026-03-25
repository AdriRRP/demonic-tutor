#![allow(clippy::expect_used)]

//! Unit coverage for unit combat double strike.

use {
    crate::support,
    demonictutor::{
        domain::play::events::DamageTarget, CastSpellCommand, DeclareAttackersCommand,
        DeclareBlockersCommand, KeywordAbility, PlayerId,
    },
};

#[test]
fn unblocked_double_strike_attacker_deals_damage_in_both_passes() {
    let (service, mut game) = support::setup_two_player_game(
        "game-double-strike-unblocked",
        support::filled_library(
            vec![support::creature_card_with_keyword(
                "attacker",
                0,
                2,
                2,
                KeywordAbility::DoubleStrike,
            )],
            20,
        ),
        support::filled_library(Vec::new(), 20),
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
    support::advance_to_player_first_main_satisfying_cleanup(&service, &mut game, "player-1");
    support::advance_turn_raw(&service, &mut game);
    support::close_empty_priority_window(&service, &mut game);
    support::advance_turn_raw(&service, &mut game);

    service
        .declare_attackers(
            &mut game,
            DeclareAttackersCommand::new(PlayerId::new("player-1"), vec![attacker_id]),
        )
        .expect("declare attackers should succeed");
    support::close_empty_priority_window(&service, &mut game);
    service
        .declare_blockers(
            &mut game,
            DeclareBlockersCommand::new(PlayerId::new("player-2"), Vec::new()),
        )
        .expect("declare empty blockers should succeed");
    support::close_empty_priority_window(&service, &mut game);

    let outcome = service
        .resolve_combat_damage(
            &mut game,
            demonictutor::ResolveCombatDamageCommand::new(PlayerId::new("player-1")),
        )
        .expect("combat damage should resolve");

    assert_eq!(game.players()[1].life(), 16);
    let player_damage_events = outcome
        .combat_damage_resolved
        .damage_events
        .iter()
        .filter(|event| {
            matches!(&event.target, DamageTarget::Player(player_id) if player_id == &PlayerId::new("player-2"))
        })
        .count();
    assert_eq!(player_damage_events, 2);
}

#[test]
fn double_strike_creature_removed_in_first_pass_does_not_deal_second_pass_damage() {
    let (service, mut game) = support::setup_two_player_game(
        "game-double-strike-removed",
        support::filled_library(
            vec![support::creature_card_with_keyword(
                "attacker",
                0,
                2,
                2,
                KeywordAbility::DoubleStrike,
            )],
            20,
        ),
        support::filled_library(
            vec![support::creature_card_with_keyword(
                "blocker",
                0,
                2,
                2,
                KeywordAbility::FirstStrike,
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

    let outcome = service
        .resolve_combat_damage(
            &mut game,
            demonictutor::ResolveCombatDamageCommand::new(PlayerId::new("player-1")),
        )
        .expect("combat damage should resolve");

    assert!(game.players()[0].graveyard_contains(&attacker_id));
    let player_damage_events = outcome
        .combat_damage_resolved
        .damage_events
        .iter()
        .filter(|event| matches!(&event.target, DamageTarget::Player(_)))
        .count();
    assert_eq!(player_damage_events, 0);
}

#[test]
fn blocked_attacker_does_not_become_unblocked_after_first_strike_kills_blocker() {
    let (service, mut game) = support::setup_two_player_game(
        "game-double-strike-remains-blocked",
        support::filled_library(
            vec![support::creature_card_with_keyword(
                "attacker",
                0,
                2,
                2,
                KeywordAbility::DoubleStrike,
            )],
            20,
        ),
        support::filled_library(vec![support::creature_card("blocker", 0, 1, 1)], 20),
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

    let outcome = service
        .resolve_combat_damage(
            &mut game,
            demonictutor::ResolveCombatDamageCommand::new(PlayerId::new("player-1")),
        )
        .expect("combat damage should resolve");

    assert!(game.players()[1].graveyard_contains(&blocker_id));
    assert_eq!(game.players()[1].life(), 20);
    let player_damage_events = outcome
        .combat_damage_resolved
        .damage_events
        .iter()
        .filter(|event| matches!(&event.target, DamageTarget::Player(_)))
        .count();
    assert_eq!(player_damage_events, 0);
}
