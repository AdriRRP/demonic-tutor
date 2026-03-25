#![allow(clippy::expect_used)]

//! Unit coverage for unit combat lifelink.

use crate::support;
use demonictutor::{
    CardDefinitionId, CastSpellCommand, DeclareAttackersCommand, DeclareBlockersCommand,
    KeywordAbility, KeywordAbilitySet, LibraryCard, PlayerId, ResolveCombatDamageCommand,
};

#[test]
fn unblocked_lifelink_attacker_gains_life_for_controller() {
    let (service, mut game) = support::setup_two_player_game(
        "test-lifelink-unblocked",
        support::filled_library(
            vec![LibraryCard::creature_with_keywords(
                CardDefinitionId::new("lifelink-attacker"),
                0,
                3,
                3,
                KeywordAbilitySet::only(KeywordAbility::Lifelink).with(KeywordAbility::Haste),
            )],
            40,
        ),
        support::filled_library(Vec::new(), 40),
    );

    support::advance_to_player_first_main_satisfying_cleanup(&service, &mut game, "player-1");
    let attacker_id = game.players()[0]
        .hand_card_at(0)
        .expect("player 1 should have the lifelink attacker in hand")
        .id()
        .clone();
    service
        .cast_spell(
            &mut game,
            CastSpellCommand::new(PlayerId::new("player-1"), attacker_id.clone()),
        )
        .expect("casting the lifelink attacker should succeed");
    support::resolve_top_stack_with_passes(&service, &mut game);

    support::advance_to_phase_satisfying_cleanup(
        &service,
        &mut game,
        demonictutor::Phase::DeclareAttackers,
    );
    service
        .declare_attackers(
            &mut game,
            DeclareAttackersCommand::new(PlayerId::new("player-1"), vec![attacker_id]),
        )
        .expect("declaring the lifelink attacker should succeed");
    support::close_empty_priority_window(&service, &mut game);
    service
        .declare_blockers(
            &mut game,
            DeclareBlockersCommand::new(PlayerId::new("player-2"), Vec::new()),
        )
        .expect("declaring no blockers should succeed");
    support::close_empty_priority_window(&service, &mut game);

    let outcome = service
        .resolve_combat_damage(
            &mut game,
            ResolveCombatDamageCommand::new(PlayerId::new("player-1")),
        )
        .expect("resolving combat damage should succeed");

    assert_eq!(support::player(&game, "player-1").life(), 23);
    assert_eq!(support::player(&game, "player-2").life(), 17);
    assert_eq!(outcome.life_changed.len(), 2);
    assert!(outcome
        .life_changed
        .iter()
        .any(|event| event.player_id == PlayerId::new("player-1") && event.to_life == 23));
}

#[test]
fn lifelink_blocker_gains_life_when_dealing_combat_damage() {
    let (service, mut game) = support::setup_two_player_game(
        "test-lifelink-blocker",
        support::filled_library(vec![support::creature_card("attacker", 0, 3, 3)], 40),
        support::filled_library(
            vec![support::creature_card_with_keyword(
                "lifelink-blocker",
                0,
                2,
                2,
                KeywordAbility::Lifelink,
            )],
            40,
        ),
    );

    support::advance_to_player_first_main_satisfying_cleanup(&service, &mut game, "player-1");
    let attacker_id = game.players()[0]
        .hand_card_at(0)
        .expect("player 1 should have the attacker in hand")
        .id()
        .clone();
    service
        .cast_spell(
            &mut game,
            CastSpellCommand::new(PlayerId::new("player-1"), attacker_id.clone()),
        )
        .expect("casting the attacker should succeed");
    support::resolve_top_stack_with_passes(&service, &mut game);

    support::advance_to_player_first_main_satisfying_cleanup(&service, &mut game, "player-2");
    let blocker_id = game.players()[1]
        .hand_card_at(0)
        .expect("player 2 should have the lifelink blocker in hand")
        .id()
        .clone();
    service
        .cast_spell(
            &mut game,
            CastSpellCommand::new(PlayerId::new("player-2"), blocker_id.clone()),
        )
        .expect("casting the blocker should succeed");
    support::resolve_top_stack_with_passes(&service, &mut game);

    support::advance_to_player_first_main_satisfying_cleanup(&service, &mut game, "player-1");
    support::advance_to_phase_satisfying_cleanup(
        &service,
        &mut game,
        demonictutor::Phase::DeclareAttackers,
    );
    service
        .declare_attackers(
            &mut game,
            DeclareAttackersCommand::new(PlayerId::new("player-1"), vec![attacker_id.clone()]),
        )
        .expect("declaring attackers should succeed");
    support::close_empty_priority_window(&service, &mut game);
    service
        .declare_blockers(
            &mut game,
            DeclareBlockersCommand::new(PlayerId::new("player-2"), vec![(blocker_id, attacker_id)]),
        )
        .expect("declaring blockers should succeed");
    support::close_empty_priority_window(&service, &mut game);

    let outcome = service
        .resolve_combat_damage(
            &mut game,
            ResolveCombatDamageCommand::new(PlayerId::new("player-1")),
        )
        .expect("resolving combat damage should succeed");

    assert_eq!(support::player(&game, "player-2").life(), 22);
    assert!(outcome
        .life_changed
        .iter()
        .any(|event| event.player_id == PlayerId::new("player-2") && event.to_life == 22));
}
