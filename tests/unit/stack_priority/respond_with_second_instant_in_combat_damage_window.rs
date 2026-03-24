#![allow(clippy::expect_used)]
#![allow(clippy::unwrap_used)]

//! Unit coverage for unit stack priority respond with second instant in combat damage window.

use {
    crate::support::{
        advance_to_player_phase_satisfying_cleanup, filled_library, instant_card,
        setup_two_player_game,
    },
    demonictutor::{
        CardDefinitionId, CastSpellCommand, DeclareBlockersCommand, PassPriorityCommand, Phase,
        PlayerId,
    },
};

#[test]
fn responding_player_can_cast_a_second_instant_before_passing_in_combat_damage() {
    let (service, mut game) = setup_two_player_game(
        "game-respond-second-combat-damage",
        filled_library(Vec::new(), 10),
        filled_library(
            vec![
                instant_card("combat-damage-response-a", 0),
                instant_card("combat-damage-response-b", 0),
            ],
            10,
        ),
    );

    advance_to_player_phase_satisfying_cleanup(
        &service,
        &mut game,
        "player-1",
        Phase::DeclareBlockers,
    );
    service
        .declare_blockers(
            &mut game,
            DeclareBlockersCommand::new(PlayerId::new("player-2"), Vec::new()),
        )
        .unwrap();
    service
        .pass_priority(
            &mut game,
            PassPriorityCommand::new(PlayerId::new("player-1")),
        )
        .unwrap();

    assert_eq!(game.phase(), &Phase::CombatDamage);
    assert_eq!(
        game.priority().unwrap().current_holder(),
        &PlayerId::new("player-2")
    );

    let bob_first = game
        .players()
        .iter()
        .find(|player| player.id() == &PlayerId::new("player-2"))
        .expect("player-2 should exist")
        .hand_card_by_definition(&CardDefinitionId::new("combat-damage-response-a"))
        .expect("first response instant should exist in Bob's hand")
        .id()
        .clone();
    let bob_second = game
        .players()
        .iter()
        .find(|player| player.id() == &PlayerId::new("player-2"))
        .expect("player-2 should exist")
        .hand_card_by_definition(&CardDefinitionId::new("combat-damage-response-b"))
        .expect("second response instant should exist in Bob's hand")
        .id()
        .clone();

    service
        .cast_spell(
            &mut game,
            CastSpellCommand::new(PlayerId::new("player-2"), bob_first),
        )
        .unwrap();
    service
        .cast_spell(
            &mut game,
            CastSpellCommand::new(PlayerId::new("player-2"), bob_second.clone()),
        )
        .unwrap();

    assert_eq!(game.stack().len(), 2);
    assert_eq!(game.stack().top().unwrap().source_card_id(), &bob_second);
    assert_eq!(
        game.priority().unwrap().current_holder(),
        &PlayerId::new("player-2")
    );
}
