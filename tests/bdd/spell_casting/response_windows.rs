use cucumber::{given, when};
use demonictutor::Phase;

use crate::world::GameplayWorld;

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

#[given("Bob has priority after attackers are declared with an instant card in hand")]
fn bob_has_priority_after_attackers_are_declared_with_an_instant_card_in_hand(
    world: &mut GameplayWorld,
) {
    world.setup_non_active_priority_after_attackers_declared_with_instant();
}

#[given("Bob has priority after blockers are declared with an instant card in hand")]
fn bob_has_priority_after_blockers_are_declared_with_an_instant_card_in_hand(
    world: &mut GameplayWorld,
) {
    world.setup_non_active_priority_after_blockers_declared_with_instant();
}

#[given("Bob has priority after combat damage with an instant card in hand")]
fn bob_has_priority_after_combat_damage_with_an_instant_card_in_hand(world: &mut GameplayWorld) {
    world.setup_non_active_priority_after_combat_damage_with_instant();
}

#[given("Alice has cast a creature spell and Bob can pay for an instant response with a land on the battlefield")]
fn alice_has_cast_a_creature_spell_and_bob_can_pay_for_an_instant_response_with_a_land_on_the_battlefield(
    world: &mut GameplayWorld,
) {
    world.setup_spell_response_stack_with_mana_paid_instant();
    world.cast_tracked_spell("Alice");
    world.pass_priority("Alice");
    let priority = world
        .game()
        .priority()
        .expect("response window should be open");
    assert_eq!(priority.current_holder(), &GameplayWorld::player_id("Bob"));
}

#[when("Bob casts the instant response spell")]
fn bob_casts_the_instant_response_spell(world: &mut GameplayWorld) {
    world.cast_tracked_response_spell("Bob");
}

#[when("Bob taps his land for mana")]
fn bob_taps_his_land_for_mana(world: &mut GameplayWorld) {
    world.tap_tracked_response_land_for_mana("Bob");
}

#[when("Bob casts the instant spell")]
fn bob_casts_the_instant_spell(world: &mut GameplayWorld) {
    world.cast_tracked_response_spell("Bob");
}
