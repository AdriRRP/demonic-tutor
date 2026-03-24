//! BDD coverage for bdd zones.

use {
    crate::world::GameplayWorld,
    cucumber::{given, then, when},
};

#[given("Alice controls a creature on the battlefield")]
fn alice_controls_creature_on_battlefield(world: &mut GameplayWorld) {
    world.setup_creature_on_battlefield("Alice");
}

#[when("a spell or ability exiles that creature")]
fn spell_or_ability_exiles_creature(world: &mut GameplayWorld) {
    world.exile_tracked_card("Alice", true);
}

#[then("that creature is no longer on the battlefield")]
fn creature_no_longer_on_battlefield(world: &mut GameplayWorld) {
    let card_id = world.tracked_card_id.as_ref().unwrap();
    assert!(!world.battlefield_contains("Alice", card_id));
}

#[then("that creature enters Alice's exile zone")]
fn creature_enters_alice_exile_zone(world: &mut GameplayWorld) {
    let card_id = world.tracked_card_id.as_ref().unwrap();
    assert!(world.exile_contains("Alice", card_id));
}

#[then("the game emits CardExiled")]
fn game_emits_card_exiled(world: &mut GameplayWorld) {
    assert!(world.last_card_exiled.is_some());
}

#[given("a creature is in Bob's graveyard")]
fn creature_in_bob_graveyard(world: &mut GameplayWorld) {
    world.setup_creature_in_graveyard("Bob");
}

#[when("a spell or ability exiles that creature from the graveyard")]
fn spell_or_ability_exiles_creature_from_graveyard(world: &mut GameplayWorld) {
    world.exile_tracked_card("Bob", false);
}

#[then("that creature leaves the graveyard")]
fn creature_leaves_graveyard(world: &mut GameplayWorld) {
    let card_id = world.tracked_card_id.as_ref().unwrap();
    assert!(!world.graveyard_contains("Bob", card_id));
}

#[then("that creature enters Bob's exile zone")]
fn creature_enters_bob_exile_zone(world: &mut GameplayWorld) {
    let card_id = world.tracked_card_id.as_ref().unwrap();
    assert!(world.exile_contains("Bob", card_id));
}

#[given("a creature is in Alice's exile zone")]
fn creature_in_alice_exile_zone(world: &mut GameplayWorld) {
    world.setup_creature_in_exile("Alice");
}

#[then("that creature is not in Alice's battlefield")]
fn creature_not_in_alice_battlefield(world: &mut GameplayWorld) {
    let card_id = world.tracked_card_id.as_ref().unwrap();
    assert!(!world.battlefield_contains("Alice", card_id));
}

#[then("that creature is not in Alice's graveyard")]
fn creature_not_in_alice_graveyard(world: &mut GameplayWorld) {
    let card_id = world.tracked_card_id.as_ref().unwrap();
    assert!(!world.graveyard_contains("Alice", card_id));
}

#[then("that creature is not in Alice's hand")]
fn creature_not_in_alice_hand(world: &mut GameplayWorld) {
    let card_id = world.tracked_card_id.as_ref().unwrap();
    assert!(!world.hand_contains("Alice", card_id));
}

#[then("that creature is not in Alice's library")]
fn creature_not_in_alice_library(world: &mut GameplayWorld) {
    let card_id = world.tracked_card_id.as_ref().unwrap();
    let alice = world.player("Alice");
    assert!(!alice.library_contains(card_id));
}
