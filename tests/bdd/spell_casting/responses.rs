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

#[given("Alice has cast a creature spell and Bob has priority with two instant cards in hand")]
fn alice_has_cast_a_creature_spell_and_bob_has_priority_with_two_instant_cards_in_hand(
    world: &mut GameplayWorld,
) {
    world.setup_spell_response_stack_with_two_instants();
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

#[when("Bob casts the instant response spell")]
fn bob_casts_the_instant_response_spell(world: &mut GameplayWorld) {
    world.cast_tracked_response_spell("Bob");
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
        error.contains("cannot be cast with sorcery-speed timing"),
        "unexpected error: {error}"
    );
}

#[given("Bob has priority in Upkeep with an instant card in hand")]
fn bob_has_priority_in_upkeep_with_an_instant_card_in_hand(world: &mut GameplayWorld) {
    world
        .setup_non_active_priority_window_with_instant("bdd-upkeep-response-window", Phase::Upkeep);
}

#[given("Bob has priority in Draw with an instant card in hand")]
fn bob_has_priority_in_draw_with_an_instant_card_in_hand(world: &mut GameplayWorld) {
    world.setup_non_active_priority_window_with_instant("bdd-draw-response-window", Phase::Draw);
}

#[given("Bob has priority in FirstMain with an instant card in hand")]
fn bob_has_priority_in_first_main_with_an_instant_card_in_hand(world: &mut GameplayWorld) {
    world.setup_non_active_priority_window_with_instant(
        "bdd-first-main-response-window",
        Phase::FirstMain,
    );
}

#[given("Bob has priority in SecondMain with an instant card in hand")]
fn bob_has_priority_in_second_main_with_an_instant_card_in_hand(world: &mut GameplayWorld) {
    world.setup_non_active_priority_window_with_instant(
        "bdd-second-main-response-window",
        Phase::SecondMain,
    );
}

#[given("Bob has priority in FirstMain with an artifact card in hand")]
fn bob_has_priority_in_first_main_with_an_artifact_card_in_hand(world: &mut GameplayWorld) {
    world.setup_non_active_priority_window_with_artifact(
        "bdd-first-main-artifact-priority-window",
        Phase::FirstMain,
    );
}

#[given("Bob has priority in EndStep with an instant card in hand")]
fn bob_has_priority_in_end_step_with_an_instant_card_in_hand(world: &mut GameplayWorld) {
    world.setup_non_active_priority_window_with_instant(
        "bdd-end-step-response-window",
        Phase::EndStep,
    );
}

#[given("Bob has priority at the beginning of Combat with an instant card in hand")]
fn bob_has_priority_at_the_beginning_of_combat_with_an_instant_card_in_hand(
    world: &mut GameplayWorld,
) {
    world.setup_non_active_priority_when_entering_combat_with_instant();
}

#[given("Bob has priority at the beginning of Combat with two instant cards in hand")]
fn bob_has_priority_at_the_beginning_of_combat_with_two_instant_cards_in_hand(
    world: &mut GameplayWorld,
) {
    world.setup_non_active_priority_when_entering_combat_with_two_instants();
}

#[given("Bob has priority after attackers are declared with an instant card in hand")]
fn bob_has_priority_after_attackers_are_declared_with_an_instant_card_in_hand(
    world: &mut GameplayWorld,
) {
    world.setup_non_active_priority_after_attackers_declared_with_instant();
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

#[given("Bob has priority after blockers are declared with an instant card in hand")]
fn bob_has_priority_after_blockers_are_declared_with_an_instant_card_in_hand(
    world: &mut GameplayWorld,
) {
    world.setup_non_active_priority_after_blockers_declared_with_instant();
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

#[given("Bob has priority after combat damage with an instant card in hand")]
fn bob_has_priority_after_combat_damage_with_an_instant_card_in_hand(world: &mut GameplayWorld) {
    world.setup_non_active_priority_after_combat_damage_with_instant();
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

#[when("Bob casts the instant spell")]
fn bob_casts_the_instant_spell(world: &mut GameplayWorld) {
    world.cast_tracked_response_spell("Bob");
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
        assert_eq!(object.controller_id(), &GameplayWorld::player_id("Bob"));
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
    assert_eq!(
        world.game().stack().objects()[1].controller_id(),
        &GameplayWorld::player_id("Bob")
    );
    assert_eq!(
        world.game().stack().objects()[2].controller_id(),
        &GameplayWorld::player_id("Bob")
    );
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
