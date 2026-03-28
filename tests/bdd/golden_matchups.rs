//! BDD coverage for golden playable matchup scenarios.

use {
    crate::world::{support, GameplayWorld},
    cucumber::{given, then, when},
};

#[given("Alice has a sacrifice outlet artifact on the battlefield in first main")]
fn alice_has_a_sacrifice_outlet_artifact_on_the_battlefield_in_first_main(
    world: &mut GameplayWorld,
) {
    world.setup_black_red_sacrifice_outlet_in_first_main();
}

#[then("the tracked sacrifice artifact is in Alice's graveyard")]
fn the_tracked_sacrifice_artifact_is_in_alices_graveyard(world: &mut GameplayWorld) {
    let artifact_id = world
        .tracked_card_id
        .as_ref()
        .expect("tracked artifact should exist");
    assert!(world.graveyard_contains("Alice", artifact_id));
    assert!(!world.battlefield_contains("Alice", artifact_id));
}

#[given("Bob is in first main with a discard spell while Alice holds a creature card")]
fn bob_is_in_first_main_with_a_discard_spell_while_alice_holds_a_creature_card(
    world: &mut GameplayWorld,
) {
    world.setup_black_red_discard_pressure_in_first_main();
}

#[when("Bob casts the discard spell targeting Alice")]
fn bob_casts_the_discard_spell_targeting_alice(world: &mut GameplayWorld) {
    world.cast_tracked_targeted_player_spell("Bob", "Alice");
}

#[when("Bob casts the discard spell targeting Alice and choosing her tracked creature card")]
fn bob_casts_the_discard_spell_targeting_alice_and_choosing_her_tracked_creature_card(
    world: &mut GameplayWorld,
) {
    world.cast_tracked_targeted_player_spell_choosing_tracked_blocker_card("Bob", "Alice");
}

#[then("Alice's tracked creature card is in her graveyard")]
fn alices_tracked_creature_card_is_in_her_graveyard(world: &mut GameplayWorld) {
    let creature_id = world
        .tracked_blocker_id
        .as_ref()
        .expect("tracked creature should exist");
    assert!(world.graveyard_contains("Alice", creature_id));
    assert!(!world.hand_contains("Alice", creature_id));
}

#[given(
    "Bob is in first main with removal while Alice has a creature on the battlefield and recursion in hand"
)]
fn bob_is_in_first_main_with_removal_while_alice_has_a_creature_on_the_battlefield_and_recursion_in_hand(
    world: &mut GameplayWorld,
) {
    world.setup_black_red_removal_and_recursion();
}

#[when("Bob casts the destroy-creature instant spell targeting Alice's creature")]
fn bob_casts_the_destroy_creature_instant_spell_targeting_alices_creature(
    world: &mut GameplayWorld,
) {
    world.cast_tracked_targeted_creature_spell("Bob");
}

#[when("Alice reaches first main with the recursion spell available")]
fn alice_reaches_first_main_with_the_recursion_spell_available(world: &mut GameplayWorld) {
    let service = support::create_service();
    support::advance_to_player_first_main_satisfying_cleanup(
        &service,
        world.game_mut(),
        "player-1",
    );
    assert_eq!(world.game().phase(), &demonictutor::Phase::FirstMain);
}

#[when("Alice casts the recursion spell targeting her graveyard creature")]
fn alice_casts_the_recursion_spell_targeting_her_graveyard_creature(world: &mut GameplayWorld) {
    world.cast_tracked_response_targeted_graveyard_card_spell("Alice");
}

#[then("Alice's tracked creature card returns to her hand")]
fn alices_tracked_creature_card_returns_to_her_hand(world: &mut GameplayWorld) {
    let creature_id = world
        .tracked_blocker_id
        .as_ref()
        .expect("tracked creature should exist");
    assert!(world.hand_contains("Alice", creature_id));
    assert!(!world.graveyard_contains("Alice", creature_id));
}

#[when("Alice casts the recovered creature spell")]
fn alice_casts_the_recovered_creature_spell(world: &mut GameplayWorld) {
    world.cast_tracked_blocker_as_spell("Alice");
}

#[then("Alice's recovered creature enters the battlefield")]
fn alices_recovered_creature_enters_the_battlefield(world: &mut GameplayWorld) {
    let creature_id = world
        .tracked_blocker_id
        .as_ref()
        .expect("tracked creature should exist");
    assert!(world.battlefield_contains("Alice", creature_id));
}

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
