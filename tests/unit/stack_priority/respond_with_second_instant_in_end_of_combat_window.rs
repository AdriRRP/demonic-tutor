#![allow(clippy::expect_used)]
#![allow(clippy::unwrap_used)]

use crate::support::{
    filled_library, instant_card, pass_priority_to_non_active_player_in_end_of_combat,
    setup_two_player_game, vanilla_creature,
};
use demonictutor::{CardDefinitionId, CastSpellCommand, LibraryCard, Phase, PlayerId};

#[test]
fn responding_player_can_cast_a_second_instant_before_passing_in_end_of_combat() {
    let (service, mut game) = setup_two_player_game(
        "game-respond-second-end-of-combat",
        filled_library(
            vec![LibraryCard::creature(
                CardDefinitionId::new("attacker"),
                0,
                3,
                3,
            )],
            10,
        ),
        filled_library(
            vec![
                vanilla_creature("bob-buffer"),
                instant_card("end-of-combat-response-a", 0),
                instant_card("end-of-combat-response-b", 0),
            ],
            10,
        ),
    );

    pass_priority_to_non_active_player_in_end_of_combat(&service, &mut game);

    assert_eq!(game.phase(), &Phase::EndOfCombat);
    assert_eq!(
        game.priority().unwrap().current_holder(),
        &PlayerId::new("player-2")
    );

    let bob_first = game
        .players()
        .iter()
        .find(|player| player.id() == &PlayerId::new("player-2"))
        .expect("player-2 should exist")
        .hand_card_by_definition(&CardDefinitionId::new("end-of-combat-response-a"))
        .expect("first response instant should exist in Bob's hand")
        .id()
        .clone();
    let bob_second = game
        .players()
        .iter()
        .find(|player| player.id() == &PlayerId::new("player-2"))
        .expect("player-2 should exist")
        .hand_card_by_definition(&CardDefinitionId::new("end-of-combat-response-b"))
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
