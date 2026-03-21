use cucumber::{given, then, when};

use crate::world::GameplayWorld;

#[given("Alice is the active player in FirstMain with at least two cards in her library for an explicit draw effect")]
fn alice_is_in_first_main_with_two_cards_for_explicit_draw(world: &mut GameplayWorld) {
    world.setup_first_main_with_library_size(2);
}

#[given("Alice is the active player in FirstMain and Bob has at least two cards in his library for an explicit draw effect")]
fn bob_is_targetable_for_explicit_draw(world: &mut GameplayWorld) {
    world.setup_first_main_with_library_size(20);
    assert_eq!(world.player_library_size("Bob"), 13);
}

#[given("Alice is the active player in FirstMain with only one card in her library for an explicit draw effect")]
fn alice_is_in_first_main_with_one_card_for_explicit_draw(world: &mut GameplayWorld) {
    world.setup_first_main_with_library_size(1);
}

#[when(expr = "Alice draws {int} cards through an explicit draw effect")]
fn alice_draws_cards_through_an_explicit_draw_effect(world: &mut GameplayWorld, count: u32) {
    world.draw_cards_effect("Alice", count);
}

#[when(expr = "Alice makes Bob draw {int} cards through an explicit draw effect")]
fn alice_makes_bob_draw_cards_through_an_explicit_draw_effect(
    world: &mut GameplayWorld,
    count: u32,
) {
    world.draw_cards_effect("Bob", count);
}

#[then(expr = "Alice draws {int} cards from the explicit effect")]
fn alice_draws_cards_from_the_explicit_effect(world: &mut GameplayWorld, count: usize) {
    assert_eq!(world.last_cards_drawn.len(), count);
}

#[then(expr = "Bob draws {int} cards from the explicit effect")]
fn bob_draws_cards_from_the_explicit_effect(world: &mut GameplayWorld, count: usize) {
    assert_eq!(world.last_cards_drawn.len(), count);
    assert!(world
        .last_cards_drawn
        .iter()
        .all(|event| event.player_id == GameplayWorld::player_id("Bob")));
}

#[then(expr = "the game emits {int} CardDrawn events with draw kind {word}")]
fn the_game_emits_card_drawn_events_with_draw_kind(
    world: &mut GameplayWorld,
    count: usize,
    draw_kind: String,
) {
    assert_eq!(world.last_cards_drawn.len(), count);
    let expected_kind = match draw_kind.as_str() {
        "ExplicitEffect" => demonictutor::DrawKind::ExplicitEffect,
        other => panic!("unsupported draw kind assertion in BDD suite: {other}"),
    };

    assert!(world
        .last_cards_drawn
        .iter()
        .all(|event| event.draw_kind == expected_kind));
}
