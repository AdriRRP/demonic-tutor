use cucumber::{given, then, when};

use crate::world::GameplayWorld;

#[given("Alice is the active player in EndStep and has eight cards in hand")]
fn alice_is_active_in_end_step_and_has_eight_cards_in_hand(world: &mut GameplayWorld) {
    world.setup_end_step_with_eight_cards_in_hand();
    assert_eq!(
        world.game().active_player(),
        &GameplayWorld::player_id("Alice")
    );
}

#[when("Alice tries to advance the turn")]
fn alice_tries_to_advance_the_turn(world: &mut GameplayWorld) {
    world.advance_turn();
}

#[then("the action is rejected because cleanup discard is still required")]
fn the_action_is_rejected_because_cleanup_discard_is_still_required(world: &mut GameplayWorld) {
    assert!(world.last_error.is_some());
    assert_eq!(world.game().phase(), &demonictutor::Phase::EndStep);
}

#[then("Alice still has eight cards in hand")]
fn alice_still_has_eight_cards_in_hand(world: &mut GameplayWorld) {
    assert_eq!(world.player_hand_size("Alice"), 8);
}

#[when("Alice discards one card for cleanup")]
fn alice_discards_one_card_for_cleanup(world: &mut GameplayWorld) {
    world.discard_tracked_card("Alice");
}

#[then("the discarded card leaves Alice's hand")]
fn the_discarded_card_leaves_alices_hand(world: &mut GameplayWorld) {
    let card_id = world
        .tracked_card_id
        .as_ref()
        .expect("tracked discard card should exist");
    assert!(!world.hand_contains("Alice", card_id));
}

#[then("the discarded card enters Alice's graveyard")]
fn the_discarded_card_enters_alices_graveyard(world: &mut GameplayWorld) {
    let card_id = world
        .tracked_card_id
        .as_ref()
        .expect("tracked discard card should exist");
    assert!(world.graveyard_contains("Alice", card_id));
}

#[then("the game emits CardDiscarded")]
fn the_game_emits_card_discarded(world: &mut GameplayWorld) {
    assert!(world.last_card_discarded.is_some());
}

#[then("Alice has seven cards in hand")]
fn alice_has_seven_cards_in_hand(world: &mut GameplayWorld) {
    assert_eq!(world.player_hand_size("Alice"), 7);
}
