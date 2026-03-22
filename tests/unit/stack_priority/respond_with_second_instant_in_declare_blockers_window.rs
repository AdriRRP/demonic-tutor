#![allow(clippy::unwrap_used)]
#![allow(clippy::expect_used)]

use crate::support::{
    advance_to_player_phase_satisfying_cleanup, filled_library, instant_card, setup_two_player_game,
};
use demonictutor::{
    CardDefinitionId, CastSpellCommand, DeclareAttackersCommand, PassPriorityCommand, Phase,
    PlayerId,
};

#[test]
fn responding_player_can_cast_a_second_instant_before_passing_in_declare_blockers() {
    let (service, mut game) = setup_two_player_game(
        "game-respond-second-declare-blockers",
        filled_library(Vec::new(), 10),
        filled_library(
            vec![
                instant_card("declare-blockers-response-a", 0),
                instant_card("declare-blockers-response-b", 0),
            ],
            10,
        ),
    );

    advance_to_player_phase_satisfying_cleanup(
        &service,
        &mut game,
        "player-1",
        Phase::DeclareAttackers,
    );
    service
        .declare_attackers(
            &mut game,
            DeclareAttackersCommand::new(PlayerId::new("player-1"), Vec::new()),
        )
        .unwrap();
    service
        .pass_priority(
            &mut game,
            PassPriorityCommand::new(PlayerId::new("player-1")),
        )
        .unwrap();

    assert_eq!(game.phase(), &Phase::DeclareBlockers);
    assert_eq!(
        game.priority().unwrap().current_holder(),
        &PlayerId::new("player-2")
    );

    let bob_first = game
        .players()
        .iter()
        .find(|player| player.id() == &PlayerId::new("player-2"))
        .expect("player-2 should exist")
        .hand_card_by_definition(&CardDefinitionId::new("declare-blockers-response-a"))
        .expect("first response instant should exist in Bob's hand")
        .id()
        .clone();
    let bob_second = game
        .players()
        .iter()
        .find(|player| player.id() == &PlayerId::new("player-2"))
        .expect("player-2 should exist")
        .hand_card_by_definition(&CardDefinitionId::new("declare-blockers-response-b"))
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
