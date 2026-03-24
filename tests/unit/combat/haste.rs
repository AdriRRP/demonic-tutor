#![allow(clippy::unwrap_used)]
#![allow(clippy::panic)]
#![allow(clippy::expect_used)]

use crate::support;
use demonictutor::{CastSpellCommand, DeclareAttackersCommand, KeywordAbility, PlayerId};

#[test]
fn haste_creature_can_attack_on_the_turn_it_enters() {
    let (service, mut game) = support::setup_two_player_game(
        "game-haste-attack",
        support::filled_library(
            vec![support::creature_card_with_keyword(
                "haste-attacker",
                0,
                2,
                2,
                KeywordAbility::Haste,
            )],
            10,
        ),
        support::filled_library(Vec::new(), 10),
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
        .unwrap();
    support::resolve_top_stack_with_passes(&service, &mut game);

    support::advance_to_phase_satisfying_cleanup(
        &service,
        &mut game,
        demonictutor::Phase::DeclareAttackers,
    );

    let result = service.declare_attackers(
        &mut game,
        DeclareAttackersCommand::new(PlayerId::new("player-1"), vec![attacker_id.clone()]),
    );

    assert!(
        result.is_ok(),
        "haste creature should attack immediately: {result:?}"
    );
    assert!(game.players()[0]
        .battlefield_card(&attacker_id)
        .expect("attacker should be on battlefield")
        .is_attacking());
}
