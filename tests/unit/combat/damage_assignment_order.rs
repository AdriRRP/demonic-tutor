#![allow(clippy::expect_used)]

//! Unit coverage for unit combat damage assignment order.

use {
    crate::support,
    demonictutor::{
        domain::play::events::DamageTarget, CardDefinitionId, CastSpellCommand,
        DeclareAttackersCommand, DeclareBlockersCommand, KeywordAbility, LibraryCard, PlayerId,
    },
};

#[test]
fn attacker_assigns_damage_to_multiple_blockers_in_declared_order() {
    let (service, mut game) = support::setup_two_player_game(
        "game-damage-order",
        support::filled_library(
            vec![support::creature_card("attacker", 0, 4, 4)],
            20,
        ),
        support::filled_library(
            vec![
                support::creature_card("small-blocker", 0, 2, 2),
                support::creature_card("large-blocker", 0, 3, 3),
            ],
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
    let small_blocker_id = game.players()[1]
        .hand_card_at(0)
        .expect("first blocker should exist in hand")
        .id()
        .clone();
    let large_blocker_id = game.players()[1]
        .hand_card_at(1)
        .expect("second blocker should exist in hand")
        .id()
        .clone();
    service
        .cast_spell(
            &mut game,
            CastSpellCommand::new(PlayerId::new("player-2"), small_blocker_id.clone()),
        )
        .expect("small blocker cast should succeed");
    support::resolve_top_stack_with_passes(&service, &mut game);
    service
        .cast_spell(
            &mut game,
            CastSpellCommand::new(PlayerId::new("player-2"), large_blocker_id.clone()),
        )
        .expect("large blocker cast should succeed");
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
                vec![
                    (small_blocker_id.clone(), attacker_id.clone()),
                    (large_blocker_id.clone(), attacker_id.clone()),
                ],
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

    assert!(game.players()[1].graveyard_contains(&small_blocker_id));
    let surviving_large_blocker = game.players()[1]
        .battlefield_card(&large_blocker_id)
        .expect("second blocker should survive");
    assert_eq!(surviving_large_blocker.damage(), 2);
    assert!(
        outcome
            .combat_damage_resolved
            .damage_events
            .iter()
            .any(|event| {
                matches!(&event.target, DamageTarget::Creature(card_id) if card_id == &small_blocker_id)
                    && event.damage_amount == 2
            })
    );
}

#[test]
fn trample_uses_declared_blocker_order_before_assigning_excess_to_player() {
    let (service, mut game) = support::setup_two_player_game(
        "game-trample-order",
        support::filled_library(
            vec![LibraryCard::creature_with_keywords(
                CardDefinitionId::new("attacker"),
                0,
                6,
                6,
                demonictutor::KeywordAbilitySet::only(KeywordAbility::Trample),
            )],
            20,
        ),
        support::filled_library(
            vec![
                support::creature_card("left-blocker", 0, 2, 2),
                support::creature_card("right-blocker", 0, 2, 2),
            ],
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
    let left_blocker_id = game.players()[1]
        .hand_card_at(0)
        .expect("left blocker should exist in hand")
        .id()
        .clone();
    let right_blocker_id = game.players()[1]
        .hand_card_at(1)
        .expect("right blocker should exist in hand")
        .id()
        .clone();
    service
        .cast_spell(
            &mut game,
            CastSpellCommand::new(PlayerId::new("player-2"), left_blocker_id.clone()),
        )
        .expect("left blocker cast should succeed");
    support::resolve_top_stack_with_passes(&service, &mut game);
    service
        .cast_spell(
            &mut game,
            CastSpellCommand::new(PlayerId::new("player-2"), right_blocker_id.clone()),
        )
        .expect("right blocker cast should succeed");
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
                vec![
                    (left_blocker_id.clone(), attacker_id.clone()),
                    (right_blocker_id.clone(), attacker_id.clone()),
                ],
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

    assert_eq!(game.players()[1].life(), 18);
    assert!(
        outcome
            .combat_damage_resolved
            .damage_events
            .iter()
            .any(|event| {
                matches!(&event.target, DamageTarget::Player(player_id) if player_id == &PlayerId::new("player-2"))
                    && event.damage_amount == 2
            })
    );
}
