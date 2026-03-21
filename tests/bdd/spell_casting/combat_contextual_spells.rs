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

#[given(
    "Bob has priority after attackers are declared with an opponent-attacking-creature instant spell in hand"
)]
fn bob_has_priority_after_attackers_are_declared_with_an_opponent_attacking_creature_instant_spell_in_hand(
    world: &mut GameplayWorld,
) {
    world.setup_non_active_priority_after_attackers_declared_with_opponents_attacking_spell();
}

#[given(
    "Bob has priority after attackers are declared with a nonlethal opponent-attacking-creature instant spell in hand"
)]
fn bob_has_priority_after_attackers_are_declared_with_a_nonlethal_opponent_attacking_creature_instant_spell_in_hand(
    world: &mut GameplayWorld,
) {
    world.setup_non_active_priority_after_attackers_declared_with_nonlethal_opponents_attacking_spell();
}

#[given(
    "Alice has priority after attackers are declared with a controlled-attacking-creature instant spell in hand"
)]
fn alice_has_priority_after_attackers_are_declared_with_a_controlled_attacking_creature_instant_spell_in_hand(
    world: &mut GameplayWorld,
) {
    world.setup_priority_after_attackers_declared_with_controlled_attacking_spell();
}

#[given(
    "Alice has priority after attackers are declared with a nonlethal controlled-attacking-creature instant spell in hand"
)]
fn alice_has_priority_after_attackers_are_declared_with_a_nonlethal_controlled_attacking_creature_instant_spell_in_hand(
    world: &mut GameplayWorld,
) {
    world.setup_priority_after_attackers_declared_with_nonlethal_controlled_attacking_spell();
}

#[given(
    "Bob has priority after attackers are declared with a controlled-attacking-creature instant spell in hand"
)]
fn bob_has_priority_after_attackers_are_declared_with_a_controlled_attacking_creature_instant_spell_in_hand(
    world: &mut GameplayWorld,
) {
    world.setup_non_active_priority_after_attackers_declared_with_controlled_attacking_spell();
}

#[given(
    "Bob has priority after blockers are declared with a controlled-blocking-creature instant spell in hand"
)]
fn bob_has_priority_after_blockers_are_declared_with_a_controlled_blocking_creature_instant_spell_in_hand(
    world: &mut GameplayWorld,
) {
    world.setup_non_active_priority_after_blockers_declared_with_controlled_blocking_spell();
}

#[given(
    "Bob has priority after blockers are declared with a nonlethal controlled-blocking-creature instant spell in hand"
)]
fn bob_has_priority_after_blockers_are_declared_with_a_nonlethal_controlled_blocking_creature_instant_spell_in_hand(
    world: &mut GameplayWorld,
) {
    world
        .setup_non_active_priority_after_blockers_declared_with_nonlethal_controlled_blocking_spell(
        );
}

#[given(
    "Bob has priority after blockers are declared with an opponent-attacking-creature instant spell in hand"
)]
fn bob_has_priority_after_blockers_are_declared_with_an_opponent_attacking_creature_instant_spell_in_hand(
    world: &mut GameplayWorld,
) {
    world.setup_non_active_priority_after_blockers_declared_with_opponents_attacking_spell();
}

#[given(
    "Alice has priority after blockers are declared with an opponent-blocking-creature instant spell in hand"
)]
fn alice_has_priority_after_blockers_are_declared_with_an_opponent_blocking_creature_instant_spell_in_hand(
    world: &mut GameplayWorld,
) {
    world.setup_priority_after_blockers_declared_with_opponents_blocking_spell();
}

#[given(
    "Alice has priority after blockers are declared with a nonlethal opponent-blocking-creature instant spell in hand"
)]
fn alice_has_priority_after_blockers_are_declared_with_a_nonlethal_opponent_blocking_creature_instant_spell_in_hand(
    world: &mut GameplayWorld,
) {
    world.setup_priority_after_blockers_declared_with_nonlethal_opponents_blocking_spell();
}

#[given(
    "Bob has priority after blockers are declared with an opponent-blocking-creature instant spell in hand"
)]
fn bob_has_priority_after_blockers_are_declared_with_an_opponent_blocking_creature_instant_spell_in_hand(
    world: &mut GameplayWorld,
) {
    world.setup_non_active_priority_after_blockers_declared_with_opponents_blocking_spell();
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

#[when("Bob casts the controlled-blocking-creature instant spell targeting his blocker")]
fn bob_casts_the_controlled_blocking_creature_instant_spell_targeting_his_blocker(
    world: &mut GameplayWorld,
) {
    world.cast_tracked_targeted_response_spell_at_blocker("Bob");
}

#[when("Bob casts the controlled-blocking-creature instant spell targeting Alice's attacker")]
fn bob_casts_the_controlled_blocking_creature_instant_spell_targeting_alices_attacker(
    world: &mut GameplayWorld,
) {
    world.try_cast_tracked_targeted_response_spell_at_attacker("Bob");
}

#[when("Bob casts the opponent-attacking-creature instant spell targeting Alice's attacker")]
fn bob_casts_the_opponent_attacking_creature_instant_spell_targeting_alices_attacker(
    world: &mut GameplayWorld,
) {
    world.cast_tracked_targeted_response_spell_at_attacker("Bob");
}

#[when("Bob casts the opponent-attacking-creature instant spell targeting his blocker")]
fn bob_casts_the_opponent_attacking_creature_instant_spell_targeting_his_blocker(
    world: &mut GameplayWorld,
) {
    world.try_cast_tracked_targeted_response_spell_at_blocker("Bob");
}

#[when("Alice casts the controlled-attacking-creature instant spell targeting her attacker")]
fn alice_casts_the_controlled_attacking_creature_instant_spell_targeting_her_attacker(
    world: &mut GameplayWorld,
) {
    world.cast_tracked_targeted_attacker_spell("Alice");
}

#[when("Bob casts the controlled-attacking-creature instant spell targeting Alice's attacker")]
fn bob_casts_the_controlled_attacking_creature_instant_spell_targeting_alices_attacker(
    world: &mut GameplayWorld,
) {
    world.try_cast_tracked_targeted_response_spell_at_attacker("Bob");
}

#[when("Alice casts the opponent-blocking-creature instant spell targeting Bob's blocker")]
fn alice_casts_the_opponent_blocking_creature_instant_spell_targeting_bobs_blocker(
    world: &mut GameplayWorld,
) {
    world.cast_tracked_targeted_creature_spell("Alice");
}

#[when("Bob casts the opponent-blocking-creature instant spell targeting his blocker")]
fn bob_casts_the_opponent_blocking_creature_instant_spell_targeting_his_blocker(
    world: &mut GameplayWorld,
) {
    world.try_cast_tracked_targeted_response_spell_at_blocker("Bob");
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

#[then("casting fails because the creature target is not legal for the spell")]
fn casting_fails_because_the_creature_target_is_not_legal_for_the_spell(world: &mut GameplayWorld) {
    assert!(world
        .last_error
        .as_ref()
        .is_some_and(|error| error.contains("cannot use the provided target")));
}

#[then("Bob's blocker has 1 damage marked and remains blocking")]
fn bobs_blocker_has_1_damage_marked_and_remains_blocking(world: &mut GameplayWorld) {
    let blocker = world.tracked_blocker();
    assert_eq!(blocker.damage(), 1);
    assert!(blocker.is_blocking());
}

#[then("Alice's attacker has 1 damage marked and remains attacking")]
fn alices_attacker_has_1_damage_marked_and_remains_attacking(world: &mut GameplayWorld) {
    let attacker = world.tracked_attacker();
    assert_eq!(attacker.damage(), 1);
    assert!(attacker.is_attacking());
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

#[then("Alice's attacker dies")]
fn alices_attacker_dies(world: &mut GameplayWorld) {
    assert_eq!(world.last_creature_died.len(), 1);
    let attacker_id = world
        .tracked_attacker_id
        .as_ref()
        .expect("tracked attacker should exist");
    assert_eq!(world.last_creature_died[0].card_id, *attacker_id);
}
