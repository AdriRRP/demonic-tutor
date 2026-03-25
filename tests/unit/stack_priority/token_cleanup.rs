#![allow(clippy::panic)]
#![allow(clippy::unwrap_used)]

use {
    crate::support::{
        advance_to_first_main_satisfying_cleanup, cast_spell_and_resolve,
        create_vanilla_creature_token_sorcery_card, filled_library, player,
        return_target_permanent_to_hand_instant_card, setup_two_player_game,
        targeted_destroy_creature_instant_card,
    },
    demonictutor::{CastSpellCommand, PassPriorityCommand, PlayerId, SpellTarget},
};

#[test]
fn token_that_dies_does_not_persist_in_graveyard() {
    let (service, mut game) = setup_two_player_game(
        "game-token-dies-cleanup",
        filled_library(
            vec![
                create_vanilla_creature_token_sorcery_card("raise-one", 0, 1, 1),
                targeted_destroy_creature_instant_card("doom-pebble", 0),
            ],
            10,
        ),
        filled_library(Vec::new(), 10),
    );
    advance_to_first_main_satisfying_cleanup(&service, &mut game);

    let token_spell_id = player(&game, "player-1")
        .hand_card_by_definition(&demonictutor::CardDefinitionId::new("raise-one"))
        .unwrap()
        .id()
        .clone();
    cast_spell_and_resolve(&service, &mut game, "player-1", token_spell_id);

    let token_id = player(&game, "player-1")
        .battlefield_card_at(0)
        .unwrap()
        .id()
        .clone();
    let destroy_id = player(&game, "player-1")
        .hand_card_by_definition(&demonictutor::CardDefinitionId::new("doom-pebble"))
        .unwrap()
        .id()
        .clone();
    service
        .cast_spell(
            &mut game,
            CastSpellCommand::new(PlayerId::new("player-1"), destroy_id)
                .with_target(SpellTarget::Creature(token_id.clone())),
        )
        .unwrap();
    service
        .pass_priority(
            &mut game,
            PassPriorityCommand::new(PlayerId::new("player-1")),
        )
        .unwrap();
    service
        .pass_priority(
            &mut game,
            PassPriorityCommand::new(PlayerId::new("player-2")),
        )
        .unwrap();

    let owner = player(&game, "player-1");
    assert!(owner.battlefield().is_empty());
    assert!(!owner.owns_card(&token_id));
    assert_eq!(owner.graveyard_size(), 2);
}

#[test]
fn token_that_is_bounced_does_not_persist_in_hand() {
    let (service, mut game) = setup_two_player_game(
        "game-token-bounce-cleanup",
        filled_library(
            vec![
                create_vanilla_creature_token_sorcery_card("raise-one", 0, 1, 1),
                return_target_permanent_to_hand_instant_card("lift-away", 0),
            ],
            10,
        ),
        filled_library(Vec::new(), 10),
    );
    advance_to_first_main_satisfying_cleanup(&service, &mut game);

    let token_spell_id = player(&game, "player-1")
        .hand_card_by_definition(&demonictutor::CardDefinitionId::new("raise-one"))
        .unwrap()
        .id()
        .clone();
    cast_spell_and_resolve(&service, &mut game, "player-1", token_spell_id);

    let token_id = player(&game, "player-1")
        .battlefield_card_at(0)
        .unwrap()
        .id()
        .clone();
    let bounce_id = player(&game, "player-1")
        .hand_card_by_definition(&demonictutor::CardDefinitionId::new("lift-away"))
        .unwrap()
        .id()
        .clone();
    service
        .cast_spell(
            &mut game,
            CastSpellCommand::new(PlayerId::new("player-1"), bounce_id)
                .with_target(SpellTarget::Permanent(token_id.clone())),
        )
        .unwrap();
    service
        .pass_priority(
            &mut game,
            PassPriorityCommand::new(PlayerId::new("player-1")),
        )
        .unwrap();
    service
        .pass_priority(
            &mut game,
            PassPriorityCommand::new(PlayerId::new("player-2")),
        )
        .unwrap();

    let owner = player(&game, "player-1");
    assert!(owner.battlefield().is_empty());
    assert!(!owner.owns_card(&token_id));
    assert!(owner.hand_card(&token_id).is_none());
}
