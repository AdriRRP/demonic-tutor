#![allow(clippy::unwrap_used)]

use crate::support::{
    advance_to_player_first_main_satisfying_cleanup, filled_library, setup_two_player_game,
};
use demonictutor::{
    CardDefinitionId, CardError, CardInstanceId, CastSpellCommand, DeclareBlockersCommand,
    DomainError, GameError, LibraryCard, PlayerId, ResolveCombatDamageCommand,
};

#[test]
fn declare_blockers_fails_when_target_creature_is_not_attacking() {
    let (service, mut game) = setup_two_player_game(
        "game-1",
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

    let attacker_id = CardInstanceId::new("game-1-player-1-0");
    let blocker_id = CardInstanceId::new("game-1-player-2-0");

    advance_to_player_first_main_satisfying_cleanup(&service, &mut game, "player-1");
    service
        .cast_spell(
            &mut game,
            CastSpellCommand::new(PlayerId::new("player-1"), attacker_id.clone()),
        )
        .unwrap();

    advance_to_player_first_main_satisfying_cleanup(&service, &mut game, "player-2");
    service
        .cast_spell(
            &mut game,
            CastSpellCommand::new(PlayerId::new("player-2"), blocker_id.clone()),
        )
        .unwrap();

    advance_to_player_first_main_satisfying_cleanup(&service, &mut game, "player-1");
    service
        .advance_turn(&mut game, demonictutor::AdvanceTurnCommand::new())
        .unwrap();

    let error = service
        .declare_blockers(
            &mut game,
            DeclareBlockersCommand::new(
                PlayerId::new("player-2"),
                vec![(blocker_id, attacker_id.clone())],
            ),
        )
        .unwrap_err();

    assert_eq!(
        error,
        DomainError::Card(CardError::NotAttacking(attacker_id))
    );
}

#[test]
fn resolve_combat_damage_fails_when_no_attackers_were_declared() {
    let (service, mut game) = setup_two_player_game(
        "game-1",
        filled_library(Vec::new(), 10),
        filled_library(Vec::new(), 10),
    );

    advance_to_player_first_main_satisfying_cleanup(&service, &mut game, "player-1");
    service
        .advance_turn(&mut game, demonictutor::AdvanceTurnCommand::new())
        .unwrap();

    let error = service
        .resolve_combat_damage(
            &mut game,
            ResolveCombatDamageCommand::new(PlayerId::new("player-1")),
        )
        .unwrap_err();

    assert_eq!(error, DomainError::Game(GameError::NoAttackersDeclared));
}

#[test]
fn declare_blockers_fails_when_the_same_blocker_is_assigned_more_than_once() {
    let (service, mut game) = setup_two_player_game(
        "game-1",
        filled_library(
            vec![
                LibraryCard::creature(CardDefinitionId::new("attacker-a"), 0, 2, 2),
                LibraryCard::creature(CardDefinitionId::new("attacker-b"), 0, 2, 2),
            ],
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

    let left_attacker_id = CardInstanceId::new("game-1-player-1-0");
    let right_attacker_id = CardInstanceId::new("game-1-player-1-1");
    let blocker_id = CardInstanceId::new("game-1-player-2-0");

    advance_to_player_first_main_satisfying_cleanup(&service, &mut game, "player-1");
    service
        .cast_spell(
            &mut game,
            CastSpellCommand::new(PlayerId::new("player-1"), left_attacker_id.clone()),
        )
        .unwrap();
    service
        .cast_spell(
            &mut game,
            CastSpellCommand::new(PlayerId::new("player-1"), right_attacker_id.clone()),
        )
        .unwrap();

    advance_to_player_first_main_satisfying_cleanup(&service, &mut game, "player-2");
    service
        .cast_spell(
            &mut game,
            CastSpellCommand::new(PlayerId::new("player-2"), blocker_id.clone()),
        )
        .unwrap();

    advance_to_player_first_main_satisfying_cleanup(&service, &mut game, "player-1");
    service
        .advance_turn(&mut game, demonictutor::AdvanceTurnCommand::new())
        .unwrap();
    service
        .declare_attackers(
            &mut game,
            demonictutor::DeclareAttackersCommand::new(
                PlayerId::new("player-1"),
                vec![left_attacker_id.clone(), right_attacker_id.clone()],
            ),
        )
        .unwrap();

    let error = service
        .declare_blockers(
            &mut game,
            DeclareBlockersCommand::new(
                PlayerId::new("player-2"),
                vec![
                    (blocker_id.clone(), left_attacker_id),
                    (blocker_id.clone(), right_attacker_id),
                ],
            ),
        )
        .unwrap_err();

    assert_eq!(
        error,
        DomainError::Game(GameError::DuplicateBlockerAssignment(blocker_id))
    );
}
