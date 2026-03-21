use cucumber::{given, then, when};
use demonictutor::Phase;

use crate::world::GameplayWorld;

#[given("Alice has cast a creature spell and still holds priority with Bob's instant in hand")]
fn alice_has_cast_a_creature_spell_and_still_holds_priority_with_bobs_instant_in_hand(
    world: &mut GameplayWorld,
) {
    world.setup_spell_response_stack();
    world.ensure_tracked_land_provides_mana();
    world.cast_tracked_spell("Alice");
    let priority = world
        .game()
        .priority()
        .expect("priority window should be open");
    assert_eq!(
        priority.current_holder(),
        &GameplayWorld::player_id("Alice")
    );
}

#[given(
    "Alice has cast an instant spell and still holds priority with Bob's creature card in hand"
)]
fn alice_has_cast_an_instant_spell_and_still_holds_priority_with_bobs_creature_card_in_hand(
    world: &mut GameplayWorld,
) {
    world.setup_invalid_noninstant_response();
    world.cast_tracked_spell("Alice");
    let priority = world
        .game()
        .priority()
        .expect("priority window should be open");
    assert_eq!(
        priority.current_holder(),
        &GameplayWorld::player_id("Alice")
    );
}

#[given("Alice has cast an instant spell and still holds priority with Bob's sorcery card in hand")]
fn alice_has_cast_an_instant_spell_and_still_holds_priority_with_bobs_sorcery_card_in_hand(
    world: &mut GameplayWorld,
) {
    world.setup_invalid_sorcery_response();
    world.cast_tracked_spell("Alice");
    let priority = world
        .game()
        .priority()
        .expect("priority window should be open");
    assert_eq!(
        priority.current_holder(),
        &GameplayWorld::player_id("Alice")
    );
}

#[given(
    "Alice has cast an instant spell and still holds priority with Bob's planeswalker card in hand"
)]
fn alice_has_cast_an_instant_spell_and_still_holds_priority_with_bobs_planeswalker_card_in_hand(
    world: &mut GameplayWorld,
) {
    world.setup_invalid_planeswalker_response();
    world.cast_tracked_spell("Alice");
    let priority = world
        .game()
        .priority()
        .expect("priority window should be open");
    assert_eq!(
        priority.current_holder(),
        &GameplayWorld::player_id("Alice")
    );
}

#[given(
    "Alice has cast an instant spell and still holds priority with Bob's own-turn artifact card in hand"
)]
fn alice_has_cast_an_instant_spell_and_still_holds_priority_with_bobs_own_turn_artifact_card_in_hand(
    world: &mut GameplayWorld,
) {
    world.setup_invalid_own_turn_artifact_response();
    world.cast_tracked_spell("Alice");
    let priority = world
        .game()
        .priority()
        .expect("priority window should be open");
    assert_eq!(
        priority.current_holder(),
        &GameplayWorld::player_id("Alice")
    );
}

#[given("Bob has priority in FirstMain with an artifact card in hand")]
fn bob_has_priority_in_first_main_with_an_artifact_card_in_hand(world: &mut GameplayWorld) {
    world.setup_non_active_priority_window_with_artifact(
        "bdd-first-main-artifact-priority-window",
        Phase::FirstMain,
    );
}

#[when("Bob tries to cast the creature response spell")]
fn bob_tries_to_cast_the_creature_response_spell(world: &mut GameplayWorld) {
    world.try_cast_tracked_response_spell("Bob");
}

#[when("Bob tries to cast the sorcery response spell")]
fn bob_tries_to_cast_the_sorcery_response_spell(world: &mut GameplayWorld) {
    world.try_cast_tracked_response_spell("Bob");
}

#[when("Bob tries to cast the planeswalker response spell")]
fn bob_tries_to_cast_the_planeswalker_response_spell(world: &mut GameplayWorld) {
    world.try_cast_tracked_response_spell("Bob");
}

#[when("Bob tries to cast the artifact spell")]
fn bob_tries_to_cast_the_artifact_spell(world: &mut GameplayWorld) {
    world.try_cast_tracked_response_spell("Bob");
}

#[then("the action is rejected because the spell timing is not legal in the current window")]
fn the_action_is_rejected_because_the_spell_timing_is_not_legal_in_the_current_window(
    world: &mut GameplayWorld,
) {
    let error = world
        .last_error
        .as_ref()
        .expect("response cast should be rejected");
    assert!(
        error.contains("active-player empty-main-phase casting permission"),
        "unexpected error: {error}"
    );
}

#[then("the action is rejected because the spell only supports open-priority casting during its controller's turn")]
fn the_action_is_rejected_because_the_spell_only_supports_open_priority_casting_during_its_controllers_turn(
    world: &mut GameplayWorld,
) {
    let error = world
        .last_error
        .as_ref()
        .expect("response cast should be rejected");
    assert!(
        error.contains("own-turn open-priority casting permission"),
        "unexpected error: {error}"
    );
}
