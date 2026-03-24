//! BDD coverage for bdd state based actions.

use {
    crate::world::GameplayWorld,
    cucumber::{given, then, when},
};

#[given("Alice has a creature card in hand with zero toughness")]
fn alice_has_a_creature_card_in_hand_with_zero_toughness(world: &mut GameplayWorld) {
    world.setup_cast_zero_toughness_creature_spell();
}

#[when("Alice casts the zero-toughness creature spell")]
fn alice_casts_the_zero_toughness_creature_spell(world: &mut GameplayWorld) {
    world.cast_tracked_spell("Alice");
}

#[then("the card is not on Alice's battlefield")]
fn the_card_is_not_on_alices_battlefield(world: &mut GameplayWorld) {
    let card_id = world
        .tracked_card_id
        .as_ref()
        .expect("tracked zero-toughness card should exist");
    assert!(!world.battlefield_contains("Alice", card_id));
}

#[then("the card enters Alice's graveyard")]
fn the_card_enters_alices_graveyard(world: &mut GameplayWorld) {
    let card_id = world
        .tracked_card_id
        .as_ref()
        .expect("tracked zero-toughness card should exist");
    assert!(world.graveyard_contains("Alice", card_id));
}
