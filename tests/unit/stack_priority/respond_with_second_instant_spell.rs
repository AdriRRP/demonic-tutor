#![allow(clippy::unwrap_used)]

//! Unit coverage for unit stack priority respond with second instant spell.

use {
    crate::support::{
        advance_to_first_main_satisfying_cleanup, filled_library, instant_card, land_card,
        setup_two_player_game, vanilla_creature,
    },
    demonictutor::{CardInstanceId, CastSpellCommand, PlayLandCommand, PlayerId, TapLandCommand},
};

#[test]
fn responding_player_can_cast_a_second_instant_before_passing_on_existing_stack() {
    let (service, mut game) = setup_two_player_game(
        "game-respond-second-instant",
        filled_library(
            vec![vanilla_creature("grizzly-bears"), land_card("forest")],
            10,
        ),
        filled_library(
            vec![instant_card("shock-a", 0), instant_card("shock-b", 0)],
            10,
        ),
    );

    advance_to_first_main_satisfying_cleanup(&service, &mut game);

    let alice_land = CardInstanceId::new("game-respond-second-instant-player-1-1");
    service
        .play_land(
            &mut game,
            PlayLandCommand::new(PlayerId::new("player-1"), alice_land.clone()),
        )
        .unwrap();
    service
        .tap_land(
            &mut game,
            TapLandCommand::new(PlayerId::new("player-1"), alice_land),
        )
        .unwrap();

    let alice_spell = CardInstanceId::new("game-respond-second-instant-player-1-0");
    service
        .cast_spell(
            &mut game,
            CastSpellCommand::new(PlayerId::new("player-1"), alice_spell.clone()),
        )
        .unwrap();
    service
        .pass_priority(
            &mut game,
            demonictutor::PassPriorityCommand::new(PlayerId::new("player-1")),
        )
        .unwrap();

    let bob_first = CardInstanceId::new("game-respond-second-instant-player-2-0");
    let bob_second = CardInstanceId::new("game-respond-second-instant-player-2-1");
    service
        .cast_spell(
            &mut game,
            CastSpellCommand::new(PlayerId::new("player-2"), bob_first.clone()),
        )
        .unwrap();
    service
        .cast_spell(
            &mut game,
            CastSpellCommand::new(PlayerId::new("player-2"), bob_second.clone()),
        )
        .unwrap();

    assert_eq!(game.stack().len(), 3);
    assert_eq!(game.stack().objects()[0].source_card_id(), &alice_spell);
    assert_eq!(game.stack().objects()[1].source_card_id(), &bob_first);
    assert_eq!(game.stack().objects()[2].source_card_id(), &bob_second);
    assert_eq!(
        game.priority().unwrap().current_holder(),
        &PlayerId::new("player-2")
    );

    let first_holder = game.priority().unwrap().current_holder().clone();
    service
        .pass_priority(
            &mut game,
            demonictutor::PassPriorityCommand::new(first_holder),
        )
        .unwrap();
    let second_holder = game.priority().unwrap().current_holder().clone();
    let top_resolution = service
        .pass_priority(
            &mut game,
            demonictutor::PassPriorityCommand::new(second_holder),
        )
        .unwrap();
    let spell_cast = top_resolution.spell_cast.unwrap();
    assert_eq!(spell_cast.card_id, bob_second);
    assert_eq!(game.stack().len(), 2);
    assert_eq!(game.stack().objects()[0].source_card_id(), &alice_spell);
    assert_eq!(game.stack().objects()[1].source_card_id(), &bob_first);
    assert_eq!(
        game.priority().unwrap().current_holder(),
        &PlayerId::new("player-1")
    );
}
