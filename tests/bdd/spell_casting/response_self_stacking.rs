//! BDD coverage for bdd spell casting response self stacking.

use {
    crate::world::GameplayWorld,
    cucumber::{given, then, when},
    demonictutor::Phase,
};

#[given("Alice has cast a creature spell and Bob has priority with two instant cards in hand")]
fn alice_has_cast_a_creature_spell_and_bob_has_priority_with_two_instant_cards_in_hand(
    world: &mut GameplayWorld,
) {
    world.setup_spell_response_stack_with_two_instants();
}

#[given("Bob has priority at the beginning of Combat with two instant cards in hand")]
fn bob_has_priority_at_the_beginning_of_combat_with_two_instant_cards_in_hand(
    world: &mut GameplayWorld,
) {
    world.setup_non_active_priority_when_entering_combat_with_two_instants();
}

#[given("Bob has priority after attackers are declared with two instant cards in hand")]
fn bob_has_priority_after_attackers_are_declared_with_two_instant_cards_in_hand(
    world: &mut GameplayWorld,
) {
    world.setup_non_active_priority_after_attackers_declared_with_two_instants();
}

#[given("Bob has priority in DeclareBlockers with two instant cards in hand")]
fn bob_has_priority_in_declare_blockers_with_two_instant_cards_in_hand(world: &mut GameplayWorld) {
    world.setup_non_active_priority_in_declare_blockers_with_two_instants();
}

#[given("Bob has priority after blockers are declared with two instant cards in hand")]
fn bob_has_priority_after_blockers_are_declared_with_two_instant_cards_in_hand(
    world: &mut GameplayWorld,
) {
    world.setup_non_active_priority_after_blockers_declared_with_two_instants();
}

#[given("Bob has priority in CombatDamage with two instant cards in hand")]
fn bob_has_priority_in_combat_damage_with_two_instant_cards_in_hand(world: &mut GameplayWorld) {
    world.setup_non_active_priority_in_combat_damage_with_two_instants();
}

#[given("Bob has priority in EndOfCombat with two instant cards in hand")]
fn bob_has_priority_in_end_of_combat_with_two_instant_cards_in_hand(world: &mut GameplayWorld) {
    world.setup_non_active_priority_in_end_of_combat_with_two_instants();
}

#[given("Bob has priority in Upkeep with two instant cards in hand")]
fn bob_has_priority_in_upkeep_with_two_instant_cards_in_hand(world: &mut GameplayWorld) {
    world.setup_non_active_priority_window_with_two_instants(
        "bdd-upkeep-response-two-instants",
        Phase::Upkeep,
    );
}

#[given("Bob has priority in Draw with two instant cards in hand")]
fn bob_has_priority_in_draw_with_two_instant_cards_in_hand(world: &mut GameplayWorld) {
    world.setup_non_active_priority_window_with_two_instants(
        "bdd-draw-response-two-instants",
        Phase::Draw,
    );
}

#[given("Bob has priority in FirstMain with two instant cards in hand")]
fn bob_has_priority_in_first_main_with_two_instant_cards_in_hand(world: &mut GameplayWorld) {
    world.setup_non_active_priority_window_with_two_instants(
        "bdd-first-main-response-two-instants",
        Phase::FirstMain,
    );
}

#[given("Bob has priority in SecondMain with two instant cards in hand")]
fn bob_has_priority_in_second_main_with_two_instant_cards_in_hand(world: &mut GameplayWorld) {
    world.setup_non_active_priority_window_with_two_instants(
        "bdd-second-main-response-two-instants",
        Phase::SecondMain,
    );
}

#[given("Bob has priority in EndStep with two instant cards in hand")]
fn bob_has_priority_in_end_step_with_two_instant_cards_in_hand(world: &mut GameplayWorld) {
    world.setup_non_active_priority_window_with_two_instants(
        "bdd-end-step-response-two-instants",
        Phase::EndStep,
    );
}

#[when("Bob casts the first instant response spell")]
fn bob_casts_the_first_instant_response_spell(world: &mut GameplayWorld) {
    world.cast_tracked_response_spell("Bob");
}

#[when("Bob casts the second instant response spell")]
fn bob_casts_the_second_instant_response_spell(world: &mut GameplayWorld) {
    world.cast_tracked_second_response_spell("Bob");
}

#[then("the stack contains 2 spells controlled by Bob")]
fn the_stack_contains_two_spells_controlled_by_bob(world: &mut GameplayWorld) {
    assert_eq!(world.game().stack().len(), 2);
    for object in world.game().stack().objects() {
        assert_eq!(object.controller_index(), 1);
    }
}

#[then("the stack contains Alice's original spell below two spells controlled by Bob")]
fn the_stack_contains_alices_original_spell_below_two_spells_controlled_by_bob(
    world: &mut GameplayWorld,
) {
    let original_spell = world
        .tracked_card_id
        .as_ref()
        .expect("tracked original spell should exist");
    assert_eq!(world.game().stack().len(), 3);
    assert_eq!(
        world.game().stack().objects()[0].source_card_id(),
        original_spell
    );
    assert_eq!(world.game().stack().objects()[1].controller_index(), 1);
    assert_eq!(world.game().stack().objects()[2].controller_index(), 1);
}

#[then("Bob's original response remains on the stack")]
fn bobs_original_response_remains_on_the_stack(world: &mut GameplayWorld) {
    let tracked_card_id = world
        .tracked_response_card_id
        .as_ref()
        .expect("original response tracked card should exist");
    assert_eq!(world.game().stack().len(), 1);
    assert_eq!(
        world
            .game()
            .stack()
            .top()
            .expect("stack should contain original response")
            .source_card_id(),
        tracked_card_id
    );
}

#[then("Bob's original response remains on the stack above Alice's original spell")]
fn bobs_original_response_remains_on_the_stack_above_alices_original_spell(
    world: &mut GameplayWorld,
) {
    let original_response = world
        .tracked_response_card_id
        .as_ref()
        .expect("tracked original response should exist");
    let original_spell = world
        .tracked_card_id
        .as_ref()
        .expect("tracked original spell should exist");
    assert_eq!(world.game().stack().len(), 2);
    assert_eq!(
        world.game().stack().objects()[0].source_card_id(),
        original_spell
    );
    assert_eq!(
        world.game().stack().objects()[1].source_card_id(),
        original_response
    );
}
