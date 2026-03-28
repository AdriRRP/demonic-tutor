//! BDD coverage for golden playable matchup scenarios.

use {
    crate::world::GameplayWorld,
    cucumber::{given, then, when},
};

#[given(
    "Alice has a flying attacker and a bounce spell after attackers while Bob has a flying blocker"
)]
fn alice_has_a_flying_attacker_and_a_bounce_spell_after_attackers_while_bob_has_a_flying_blocker(
    world: &mut GameplayWorld,
) {
    world.setup_white_blue_tempo_bounce_after_attackers();
}

#[given(
    "Alice has a flying attacker and a pump spell after blockers while Bob has blocked with a flying creature"
)]
fn alice_has_a_flying_attacker_and_a_pump_spell_after_blockers_while_bob_has_blocked_with_a_flying_creature(
    world: &mut GameplayWorld,
) {
    world.setup_white_blue_tempo_pump_after_blockers();
}

#[when("Alice casts the bounce spell targeting Bob's blocker")]
fn alice_casts_the_bounce_spell_targeting_bobs_blocker(world: &mut GameplayWorld) {
    world.cast_tracked_targeted_permanent_spell("Alice");
}

#[then("Bob's blocker returns to his hand")]
fn bobs_blocker_returns_to_his_hand(world: &mut GameplayWorld) {
    let blocker_id = world
        .tracked_blocker_id
        .as_ref()
        .expect("tracked blocker should exist");
    assert!(world.hand_contains("Bob", blocker_id));
    assert!(!world.battlefield_contains("Bob", blocker_id));
}

#[when("both players pass through blockers without declaring blockers")]
fn both_players_pass_through_blockers_without_declaring_blockers(world: &mut GameplayWorld) {
    world.close_current_priority_window();
    world.advance_turn();
}

#[then("Bob loses 2 life from the flying attack")]
fn bob_loses_2_life_from_the_flying_attack(world: &mut GameplayWorld) {
    assert_eq!(world.player_life("Bob"), 18);
}
