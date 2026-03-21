use cucumber::{given, then, when};

use crate::world::GameplayWorld;

#[given(
    "Bob has declared blockers and Alice still has a flash creature card in hand with priority"
)]
fn bob_has_declared_blockers_and_alice_still_has_a_flash_creature_card_in_hand_with_priority(
    world: &mut GameplayWorld,
) {
    world.setup_priority_after_blockers_declared_with_flash_creature();
}

#[given(
    "combat damage has resolved and Alice still has a flash creature card in hand with priority"
)]
fn combat_damage_has_resolved_and_alice_still_has_a_flash_creature_card_in_hand_with_priority(
    world: &mut GameplayWorld,
) {
    world.setup_priority_after_combat_damage_with_flash_creature();
}

#[when("Alice casts the flash creature spell")]
fn alice_casts_the_flash_creature_spell(world: &mut GameplayWorld) {
    world.cast_tracked_spell("Alice");
}

#[given("Alice is the active player in FirstMain with a blocking-creature instant spell in hand")]
fn alice_is_the_active_player_in_first_main_with_a_blocking_creature_instant_spell_in_hand(
    world: &mut GameplayWorld,
) {
    world.setup_blocking_creature_player_target_spell();
}

#[given(
    "Alice is the active player in FirstMain with a blocking-creature instant spell and Bob's creature on the battlefield"
)]
fn alice_is_the_active_player_in_first_main_with_a_blocking_creature_instant_spell_and_bobs_creature_on_the_battlefield(
    world: &mut GameplayWorld,
) {
    world.setup_targeted_blocking_creature_spell();
}

#[given(
    "Bob has declared blockers and Alice still has a blocking-creature instant spell in hand with priority"
)]
fn bob_has_declared_blockers_and_alice_still_has_a_blocking_creature_instant_spell_in_hand_with_priority(
    world: &mut GameplayWorld,
) {
    world.setup_priority_after_blockers_declared_with_blocking_creature_spell();
}

#[given(
    "Bob has declared blockers and Alice still has a nonlethal blocking-creature instant spell in hand with priority"
)]
fn bob_has_declared_blockers_and_alice_still_has_a_nonlethal_blocking_creature_instant_spell_in_hand_with_priority(
    world: &mut GameplayWorld,
) {
    world.setup_priority_after_blockers_declared_with_nonlethal_blocking_creature_spell();
}

#[when("Alice casts the blocking-creature instant spell targeting Bob")]
fn alice_casts_the_blocking_creature_instant_spell_targeting_bob(world: &mut GameplayWorld) {
    world.try_cast_tracked_targeted_player_spell("Alice", "Bob");
}

#[when("Alice casts the blocking-creature instant spell targeting Bob's creature")]
fn alice_casts_the_blocking_creature_instant_spell_targeting_bobs_creature(
    world: &mut GameplayWorld,
) {
    world.try_cast_tracked_targeted_creature_spell("Alice");
}

#[when("Alice casts the blocking-creature instant spell targeting Bob's blocker")]
fn alice_casts_the_blocking_creature_instant_spell_targeting_bobs_blocker(
    world: &mut GameplayWorld,
) {
    world.cast_tracked_targeted_creature_spell("Alice");
}

#[then("casting fails because the spell only supports creature targets")]
fn casting_fails_because_the_spell_only_supports_creature_targets(world: &mut GameplayWorld) {
    assert!(world
        .last_error
        .as_ref()
        .is_some_and(|error| error.contains("cannot use the provided target")));
}

#[then("casting fails because the target creature is not currently blocking")]
fn casting_fails_because_the_target_creature_is_not_currently_blocking(world: &mut GameplayWorld) {
    assert!(world
        .last_error
        .as_ref()
        .is_some_and(|error| error.contains("cannot use the provided target")));
}

#[then("Bob's blocker dies")]
fn bobs_blocker_dies(world: &mut GameplayWorld) {
    assert_eq!(world.last_creature_died.len(), 1);
    let blocker_id = world
        .tracked_blocker_id
        .as_ref()
        .expect("tracked blocker should exist");
    assert_eq!(world.last_creature_died[0].card_id, *blocker_id);
}

#[then("Bob's blocker has 1 damage marked and remains blocking")]
fn bobs_blocker_has_1_damage_marked_and_remains_blocking(world: &mut GameplayWorld) {
    let blocker = world.tracked_blocker();
    assert_eq!(blocker.damage(), 1);
    assert!(blocker.is_blocking());
}
