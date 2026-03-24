//! BDD coverage for bdd spell casting active instant windows.

use {
    crate::world::GameplayWorld,
    cucumber::{given, when},
    demonictutor::Phase,
};

#[given("Alice is the active player in Upkeep with an instant card in hand and priority")]
fn alice_is_the_active_player_in_upkeep_with_an_instant_card_in_hand_and_priority(
    world: &mut GameplayWorld,
) {
    world.setup_active_priority_window_with_instant("bdd-upkeep-instant-window", Phase::Upkeep);
}

#[given("Alice is the active player in Draw with an instant card in hand and priority")]
fn alice_is_the_active_player_in_draw_with_an_instant_card_in_hand_and_priority(
    world: &mut GameplayWorld,
) {
    world.setup_active_priority_window_with_instant("bdd-draw-instant-window", Phase::Draw);
}

#[given("Alice is the active player in FirstMain with an instant card in hand and priority")]
fn alice_is_the_active_player_in_first_main_with_an_instant_card_in_hand_and_priority(
    world: &mut GameplayWorld,
) {
    world.setup_active_priority_window_with_instant(
        "bdd-first-main-instant-window",
        Phase::FirstMain,
    );
}

#[given("Alice is the active player in SecondMain with an instant card in hand and priority")]
fn alice_is_the_active_player_in_second_main_with_an_instant_card_in_hand_and_priority(
    world: &mut GameplayWorld,
) {
    world.setup_active_priority_window_with_instant(
        "bdd-second-main-instant-window",
        Phase::SecondMain,
    );
}

#[given("Alice is the active player in EndStep with an instant card in hand and priority")]
fn alice_is_the_active_player_in_end_step_with_an_instant_card_in_hand_and_priority(
    world: &mut GameplayWorld,
) {
    world.setup_active_priority_window_with_instant("bdd-end-step-instant-window", Phase::EndStep);
}

#[given("Alice is at the beginning of Combat with an instant card in hand and priority")]
fn alice_is_at_the_beginning_of_combat_with_an_instant_card_in_hand_and_priority(
    world: &mut GameplayWorld,
) {
    world.setup_priority_when_entering_combat_with_instant();
}

#[given("Alice has declared attackers and still has an instant card in hand with priority")]
fn alice_has_declared_attackers_and_still_has_an_instant_card_in_hand_with_priority(
    world: &mut GameplayWorld,
) {
    world.setup_priority_after_attackers_declared_with_instant();
}

#[given("Bob has declared blockers and Alice still has an instant card in hand with priority")]
fn bob_has_declared_blockers_and_alice_still_has_an_instant_card_in_hand_with_priority(
    world: &mut GameplayWorld,
) {
    world.setup_priority_after_blockers_declared_with_instant();
}

#[given("combat damage has resolved and Alice still has an instant card in hand with priority")]
fn combat_damage_has_resolved_and_alice_still_has_an_instant_card_in_hand_with_priority(
    world: &mut GameplayWorld,
) {
    world.setup_priority_after_combat_damage_with_instant();
}

#[when("Alice casts the instant spell")]
fn alice_casts_the_instant_spell(world: &mut GameplayWorld) {
    world.cast_tracked_spell("Alice");
}
