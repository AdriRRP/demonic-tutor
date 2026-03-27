#![allow(clippy::unwrap_used)]
#![allow(clippy::panic)]
#![allow(clippy::expect_used)]

//! Unit coverage for unit combat defender.

use {
    crate::support,
    demonictutor::{CastSpellCommand, DeclareAttackersCommand, KeywordAbility, PlayerId},
};

#[test]
fn defender_creature_cannot_attack() {
    let (service, mut game) = support::setup_two_player_game(
        "game-defender-cannot-attack",
        support::filled_library(
            vec![support::creature_card_with_keyword(
                "wall-lite",
                0,
                0,
                4,
                KeywordAbility::Defender,
            )],
            10,
        ),
        support::filled_library(Vec::new(), 10),
    );

    support::advance_to_player_first_main_satisfying_cleanup(&service, &mut game, "player-1");
    let defender_id = game.players()[0]
        .hand_card_at(0)
        .expect("defender should exist in hand")
        .id()
        .clone();

    service
        .cast_spell(
            &mut game,
            CastSpellCommand::new(PlayerId::new("player-1"), defender_id.clone()),
        )
        .unwrap();
    support::resolve_top_stack_with_passes(&service, &mut game);

    support::advance_to_player_phase_satisfying_cleanup(
        &service,
        &mut game,
        "player-2",
        demonictutor::Phase::Untap,
    );
    support::advance_to_player_phase_satisfying_cleanup(
        &service,
        &mut game,
        "player-1",
        demonictutor::Phase::DeclareAttackers,
    );

    let result = service.declare_attackers(
        &mut game,
        DeclareAttackersCommand::new(PlayerId::new("player-1"), vec![defender_id.clone()]),
    );

    assert!(
        matches!(
            result,
            Err(demonictutor::DomainError::Card(demonictutor::CardError::CannotAttack {
                ref card,
                ..
            })) if card == &defender_id
        ),
        "defender creature should not be attackable: {result:?}"
    );
}
