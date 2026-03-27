#![allow(clippy::expect_used)]
#![allow(clippy::panic)]

use crate::support::{
    advance_to_first_main_satisfying_cleanup, cast_spell_and_resolve,
    create_keyworded_creature_token_sorcery_card, create_vanilla_creature_token_sorcery_card,
    filled_library, player, setup_two_player_game,
};
use demonictutor::KeywordAbility;

#[test]
fn resolving_supported_token_spell_creates_one_vanilla_creature_on_battlefield() {
    let (service, mut game) = setup_two_player_game(
        "game-token-resolution",
        filled_library(
            vec![
                create_vanilla_creature_token_sorcery_card("raise-one", 0, 1, 1),
                create_vanilla_creature_token_sorcery_card("raise-one", 0, 1, 1),
                create_vanilla_creature_token_sorcery_card("raise-one", 0, 1, 1),
                create_vanilla_creature_token_sorcery_card("raise-one", 0, 1, 1),
                create_vanilla_creature_token_sorcery_card("raise-one", 0, 1, 1),
                create_vanilla_creature_token_sorcery_card("raise-one", 0, 1, 1),
                create_vanilla_creature_token_sorcery_card("raise-one", 0, 1, 1),
            ],
            10,
        ),
        filled_library(Vec::new(), 10),
    );
    advance_to_first_main_satisfying_cleanup(&service, &mut game);
    let spell_id = player(&game, "player-1")
        .hand_card_at(0)
        .expect("token spell should be in hand")
        .id()
        .clone();
    cast_spell_and_resolve(&service, &mut game, "player-1", spell_id);

    let active_player = player(&game, "player-1");
    let token = active_player
        .battlefield_card_at(0)
        .expect("token should exist on battlefield");

    assert!(token.is_token());
    assert_eq!(token.creature_stats(), Some((1, 1)));
    assert_eq!(active_player.battlefield().len(), 1);
    assert_eq!(active_player.graveyard_size(), 1);
}

#[test]
fn resolving_supported_token_spell_can_create_one_keyworded_creature_token() {
    let (service, mut game) = setup_two_player_game(
        "game-keyword-token-resolution",
        filled_library(
            vec![create_keyworded_creature_token_sorcery_card(
                "raise-flyer",
                0,
                1,
                1,
                KeywordAbility::Flying,
            )],
            10,
        ),
        filled_library(Vec::new(), 10),
    );
    advance_to_first_main_satisfying_cleanup(&service, &mut game);
    let spell_id = player(&game, "player-1")
        .hand_card_at(0)
        .expect("token spell should be in hand")
        .id()
        .clone();
    cast_spell_and_resolve(&service, &mut game, "player-1", spell_id);

    let active_player = player(&game, "player-1");
    let token = active_player
        .battlefield_card_at(0)
        .expect("token should exist on battlefield");

    assert!(token.is_token());
    assert_eq!(token.creature_stats(), Some((1, 1)));
    assert!(token.has_flying());
}
