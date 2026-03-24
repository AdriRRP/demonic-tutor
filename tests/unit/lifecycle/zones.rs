#![allow(clippy::panic)]
#![allow(clippy::unwrap_used)]

//! Unit coverage for unit lifecycle zones.

use {
    crate::support::{self},
    demonictutor::{ExileCardCommand, PlayerId},
};

#[test]
fn test_exile_from_battlefield() {
    let alice = PlayerId::new("player-1");

    let (service, mut game) = support::setup_two_player_game(
        "test-exile-battlefield",
        support::filled_library(vec![support::creature_card("to-exile", 0, 2, 2)], 40),
        support::filled_library(Vec::new(), 40),
    );

    support::advance_to_player_first_main_satisfying_cleanup(&service, &mut game, "player-1");

    let card_id = game.players()[0]
        .hand_card_at(0)
        .unwrap_or_else(|| panic!("hand card should exist"))
        .id()
        .clone();

    // Cast creature
    let cast_result = service.cast_spell(
        &mut game,
        demonictutor::CastSpellCommand::new(alice.clone(), card_id.clone()),
    );
    assert!(
        cast_result.is_ok(),
        "setup creature cast should succeed: {cast_result:?}"
    );
    support::resolve_top_stack_with_passes(&service, &mut game);

    assert!(game.players()[0].battlefield_contains(&card_id));

    // Exile creature from battlefield
    let exile_result = service.exile_card(
        &mut game,
        &ExileCardCommand::new(alice, card_id.clone(), true),
    );
    assert!(
        exile_result.is_ok(),
        "exile from battlefield should succeed: {exile_result:?}"
    );
    assert!(matches!(&exile_result, Ok(event) if event.card_id == card_id));
    assert!(!game.players()[0].battlefield_contains(&card_id));
    assert!(game.players()[0].exile_contains(&card_id));
}

#[test]
fn test_exile_from_graveyard() {
    let alice = PlayerId::new("player-1");

    let (service, mut game) = support::setup_two_player_game(
        "test-exile-graveyard",
        support::filled_library(vec![support::creature_card("to-exile", 0, 0, 0)], 40),
        support::filled_library(Vec::new(), 40),
    );

    support::advance_to_player_first_main_satisfying_cleanup(&service, &mut game, "player-1");

    let card_id = game.players()[0]
        .hand_card_at(0)
        .unwrap_or_else(|| panic!("hand card should exist"))
        .id()
        .clone();

    // A zero-toughness creature resolves to the battlefield and then immediately dies to SBA review.
    let cast_result = service.cast_spell(
        &mut game,
        demonictutor::CastSpellCommand::new(alice.clone(), card_id.clone()),
    );
    assert!(
        cast_result.is_ok(),
        "setup zero-toughness creature cast should succeed: {cast_result:?}"
    );
    support::resolve_top_stack_with_passes(&service, &mut game);

    assert!(game.players()[0].graveyard_contains(&card_id));

    // Exile card from graveyard
    let exile_result = service.exile_card(
        &mut game,
        &ExileCardCommand::new(alice, card_id.clone(), false),
    );
    assert!(
        exile_result.is_ok(),
        "exile from graveyard should succeed: {exile_result:?}"
    );
    assert!(matches!(&exile_result, Ok(event) if event.card_id == card_id));
    assert!(!game.players()[0].graveyard_contains(&card_id));
    assert!(game.players()[0].exile_contains(&card_id));
}
