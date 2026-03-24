#![allow(clippy::expect_used)]

//! Unit coverage for unit combat blocking legality.

use {
    crate::support,
    demonictutor::{CastSpellCommand, DeclareAttackersCommand, DeclareBlockersCommand, PlayerId},
};

fn cast_spell_and_resolve_for_player(
    service: &support::TestService,
    game: &mut demonictutor::Game,
    player_id: &str,
    card_id: demonictutor::CardInstanceId,
) {
    let result = service.cast_spell(
        game,
        CastSpellCommand::new(PlayerId::new(player_id), card_id),
    );
    assert!(
        result.is_ok(),
        "creature setup cast should succeed: {result:?}"
    );
    support::resolve_top_stack_with_passes(service, game);
}

fn advance_to_combat_after_battlefield_setup(
    service: &support::TestService,
    game: &mut demonictutor::Game,
    attacker_id: demonictutor::CardInstanceId,
) {
    support::advance_to_player_first_main_satisfying_cleanup(service, game, "player-1");
    support::advance_to_phase_satisfying_cleanup(
        service,
        game,
        demonictutor::Phase::DeclareAttackers,
    );
    let result = service.declare_attackers(
        game,
        DeclareAttackersCommand::new(PlayerId::new("player-1"), vec![attacker_id]),
    );
    assert!(
        result.is_ok(),
        "declaring attackers for combat setup should succeed: {result:?}"
    );
    support::close_empty_priority_window(service, game);
}

#[test]
fn test_flying_blocking_legality() {
    let bob = PlayerId::new("player-2");

    let (service, mut game) = support::setup_two_player_game(
        "test-flying",
        support::filled_library(
            vec![support::creature_card_with_keywords(
                "attacker", 0, 2, 2, true, false,
            )],
            40,
        ),
        support::filled_library(
            vec![
                support::creature_card_with_keywords("non-flying", 0, 2, 2, false, false),
                support::creature_card_with_keywords("flying-blocker", 0, 2, 2, true, false),
                support::creature_card_with_keywords("reach-blocker", 0, 2, 2, false, true),
            ],
            40,
        ),
    );

    support::advance_to_player_first_main_satisfying_cleanup(&service, &mut game, "player-1");
    let attacker_id = game.players()[0]
        .hand_card_at(0)
        .expect("attacker should exist in hand")
        .id()
        .clone();
    cast_spell_and_resolve_for_player(&service, &mut game, "player-1", attacker_id.clone());

    support::advance_to_player_first_main_satisfying_cleanup(&service, &mut game, "player-2");
    let non_flying_id = game.players()[1]
        .hand_card_at(0)
        .expect("non-flying blocker should exist in hand")
        .id()
        .clone();
    let flying_blocker_id = game.players()[1]
        .hand_card_at(1)
        .expect("flying blocker should exist in hand")
        .id()
        .clone();
    cast_spell_and_resolve_for_player(&service, &mut game, "player-2", non_flying_id.clone());
    cast_spell_and_resolve_for_player(&service, &mut game, "player-2", flying_blocker_id.clone());

    advance_to_combat_after_battlefield_setup(&service, &mut game, attacker_id.clone());

    let res = service.declare_blockers(
        &mut game,
        DeclareBlockersCommand::new(bob.clone(), vec![(non_flying_id, attacker_id.clone())]),
    );
    assert!(matches!(
        &res,
        Err(error) if error.to_string().contains("cannot block flying creature")
    ));

    let result = service.declare_blockers(
        &mut game,
        DeclareBlockersCommand::new(bob, vec![(flying_blocker_id, attacker_id)]),
    );
    assert!(result.is_ok(), "flying should block flying: {result:?}");
}

#[test]
fn test_reach_blocking_legality() {
    let bob = PlayerId::new("player-2");

    let (service, mut game) = support::setup_two_player_game(
        "test-reach",
        support::filled_library(
            vec![support::creature_card_with_keywords(
                "attacker", 0, 2, 2, true, false,
            )],
            40,
        ),
        support::filled_library(
            vec![support::creature_card_with_keywords(
                "reach-blocker",
                0,
                2,
                2,
                false,
                true,
            )],
            40,
        ),
    );

    support::advance_to_player_first_main_satisfying_cleanup(&service, &mut game, "player-1");
    let attacker_id = game.players()[0]
        .hand_card_at(0)
        .expect("attacker should exist in hand")
        .id()
        .clone();
    cast_spell_and_resolve_for_player(&service, &mut game, "player-1", attacker_id.clone());

    support::advance_to_player_first_main_satisfying_cleanup(&service, &mut game, "player-2");
    let reach_blocker_id = game.players()[1]
        .hand_card_at(0)
        .expect("reach blocker should exist in hand")
        .id()
        .clone();
    cast_spell_and_resolve_for_player(&service, &mut game, "player-2", reach_blocker_id.clone());

    advance_to_combat_after_battlefield_setup(&service, &mut game, attacker_id.clone());

    let result = service.declare_blockers(
        &mut game,
        DeclareBlockersCommand::new(bob, vec![(reach_blocker_id, attacker_id)]),
    );
    assert!(result.is_ok(), "reach should block flying: {result:?}");
}

#[test]
fn test_non_flying_blocking_legality() {
    let bob = PlayerId::new("player-2");

    let (service, mut game) = support::setup_two_player_game(
        "test-non-flying",
        support::filled_library(
            vec![support::creature_card_with_keywords(
                "attacker", 0, 2, 2, false, false,
            )],
            40,
        ),
        support::filled_library(
            vec![support::creature_card_with_keywords(
                "blocker", 0, 2, 2, false, false,
            )],
            40,
        ),
    );

    support::advance_to_player_first_main_satisfying_cleanup(&service, &mut game, "player-1");
    let attacker_id = game.players()[0]
        .hand_card_at(0)
        .expect("attacker should exist in hand")
        .id()
        .clone();
    cast_spell_and_resolve_for_player(&service, &mut game, "player-1", attacker_id.clone());

    support::advance_to_player_first_main_satisfying_cleanup(&service, &mut game, "player-2");
    let blocker_id = game.players()[1]
        .hand_card_at(0)
        .expect("blocker should exist in hand")
        .id()
        .clone();
    cast_spell_and_resolve_for_player(&service, &mut game, "player-2", blocker_id.clone());

    advance_to_combat_after_battlefield_setup(&service, &mut game, attacker_id.clone());

    let result = service.declare_blockers(
        &mut game,
        DeclareBlockersCommand::new(bob, vec![(blocker_id, attacker_id)]),
    );
    assert!(
        result.is_ok(),
        "non-flying should block non-flying: {result:?}"
    );
}
