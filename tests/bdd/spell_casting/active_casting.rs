use cucumber::{given, then, when};
use demonictutor::Phase;

use crate::world::GameplayWorld;

#[given("Alice is the active player in SecondMain with a creature card in hand and priority")]
fn alice_is_the_active_player_in_second_main_with_a_creature_card_in_hand_and_priority(
    world: &mut GameplayWorld,
) {
    world.setup_active_priority_window_with_creature(
        "bdd-second-main-creature-window",
        Phase::SecondMain,
    );
}

#[when("Alice casts the creature spell")]
fn alice_casts_the_creature_spell(world: &mut GameplayWorld) {
    world.cast_tracked_spell("Alice");
}

#[when("Alice casts the sorcery spell")]
fn alice_casts_the_sorcery_spell(world: &mut GameplayWorld) {
    world.cast_tracked_spell("Alice");
}

#[when("Alice casts the artifact spell")]
fn alice_casts_the_artifact_spell(world: &mut GameplayWorld) {
    world.cast_tracked_spell("Alice");
}

#[when("Alice casts the enchantment spell")]
fn alice_casts_the_enchantment_spell(world: &mut GameplayWorld) {
    world.cast_tracked_spell("Alice");
}

#[when("Alice casts the planeswalker spell")]
fn alice_casts_the_planeswalker_spell(world: &mut GameplayWorld) {
    world.cast_tracked_spell("Alice");
}

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

#[given("Alice is the active player in FirstMain with a sorcery card in hand and priority")]
fn alice_is_the_active_player_in_first_main_with_a_sorcery_card_in_hand_and_priority(
    world: &mut GameplayWorld,
) {
    world.setup_active_priority_window_with_sorcery(
        "bdd-first-main-sorcery-window",
        Phase::FirstMain,
    );
}

#[given("Alice is the active player in FirstMain with an artifact card in hand and priority")]
fn alice_is_the_active_player_in_first_main_with_an_artifact_card_in_hand_and_priority(
    world: &mut GameplayWorld,
) {
    world.setup_active_priority_window_with_artifact(
        "bdd-first-main-artifact-window",
        Phase::FirstMain,
    );
}

#[given("Alice is the active player in FirstMain with an enchantment card in hand and priority")]
fn alice_is_the_active_player_in_first_main_with_an_enchantment_card_in_hand_and_priority(
    world: &mut GameplayWorld,
) {
    world.setup_active_priority_window_with_enchantment(
        "bdd-first-main-enchantment-window",
        Phase::FirstMain,
    );
}

#[given("Alice is the active player in FirstMain with a planeswalker card in hand and priority")]
fn alice_is_the_active_player_in_first_main_with_a_planeswalker_card_in_hand_and_priority(
    world: &mut GameplayWorld,
) {
    world.setup_active_priority_window_with_planeswalker(
        "bdd-first-main-planeswalker-window",
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

#[given("Alice is the active player in SecondMain with a sorcery card in hand and priority")]
fn alice_is_the_active_player_in_second_main_with_a_sorcery_card_in_hand_and_priority(
    world: &mut GameplayWorld,
) {
    world.setup_active_priority_window_with_sorcery(
        "bdd-second-main-sorcery-window",
        Phase::SecondMain,
    );
}

#[given("Alice is the active player in SecondMain with an artifact card in hand and priority")]
fn alice_is_the_active_player_in_second_main_with_an_artifact_card_in_hand_and_priority(
    world: &mut GameplayWorld,
) {
    world.setup_active_priority_window_with_artifact(
        "bdd-second-main-artifact-window",
        Phase::SecondMain,
    );
}

#[given("Alice is the active player in SecondMain with an enchantment card in hand and priority")]
fn alice_is_the_active_player_in_second_main_with_an_enchantment_card_in_hand_and_priority(
    world: &mut GameplayWorld,
) {
    world.setup_active_priority_window_with_enchantment(
        "bdd-second-main-enchantment-window",
        Phase::SecondMain,
    );
}

#[given("Alice is the active player in SecondMain with a planeswalker card in hand and priority")]
fn alice_is_the_active_player_in_second_main_with_a_planeswalker_card_in_hand_and_priority(
    world: &mut GameplayWorld,
) {
    world.setup_active_priority_window_with_planeswalker(
        "bdd-second-main-planeswalker-window",
        Phase::SecondMain,
    );
}

#[given("Alice is the active player in EndStep with an instant card in hand and priority")]
fn alice_is_the_active_player_in_end_step_with_an_instant_card_in_hand_and_priority(
    world: &mut GameplayWorld,
) {
    world.setup_active_priority_window_with_instant("bdd-end-step-instant-window", Phase::EndStep);
}

#[given("Alice is the active player in Upkeep with two instant cards in hand and priority")]
fn alice_is_the_active_player_in_upkeep_with_two_instant_cards_in_hand_and_priority(
    world: &mut GameplayWorld,
) {
    world.setup_active_priority_window_with_two_instants("bdd-upkeep-two-instants", Phase::Upkeep);
}

#[given("Alice is the active player in Draw with two instant cards in hand and priority")]
fn alice_is_the_active_player_in_draw_with_two_instant_cards_in_hand_and_priority(
    world: &mut GameplayWorld,
) {
    world.setup_active_priority_window_with_two_instants("bdd-draw-two-instants", Phase::Draw);
}

#[given("Alice is the active player in FirstMain with two instant cards in hand and priority")]
fn alice_is_the_active_player_in_first_main_with_two_instant_cards_in_hand_and_priority(
    world: &mut GameplayWorld,
) {
    world.setup_active_priority_window_with_two_instants(
        "bdd-first-main-two-instants",
        Phase::FirstMain,
    );
}

#[given("Alice is the active player in SecondMain with two instant cards in hand and priority")]
fn alice_is_the_active_player_in_second_main_with_two_instant_cards_in_hand_and_priority(
    world: &mut GameplayWorld,
) {
    world.setup_active_priority_window_with_two_instants(
        "bdd-second-main-two-instants",
        Phase::SecondMain,
    );
}

#[given("Alice is the active player in EndStep with two instant cards in hand and priority")]
fn alice_is_the_active_player_in_end_step_with_two_instant_cards_in_hand_and_priority(
    world: &mut GameplayWorld,
) {
    world.setup_active_priority_window_with_two_instants(
        "bdd-end-step-two-instants",
        Phase::EndStep,
    );
}

#[given("Alice is at the beginning of Combat with two instant cards in hand and priority")]
fn alice_is_at_the_beginning_of_combat_with_two_instant_cards_in_hand_and_priority(
    world: &mut GameplayWorld,
) {
    world.setup_priority_when_entering_combat_with_two_instants();
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

#[given("Alice has declared attackers and still has two instant cards in hand with priority")]
fn alice_has_declared_attackers_and_still_has_two_instant_cards_in_hand_with_priority(
    world: &mut GameplayWorld,
) {
    world.setup_priority_after_attackers_declared_with_two_instants();
}

#[given("Bob has declared blockers and Alice still has an instant card in hand with priority")]
fn bob_has_declared_blockers_and_alice_still_has_an_instant_card_in_hand_with_priority(
    world: &mut GameplayWorld,
) {
    world.setup_priority_after_blockers_declared_with_instant();
}

#[given("Bob has declared blockers and Alice still has two instant cards in hand with priority")]
fn bob_has_declared_blockers_and_alice_still_has_two_instant_cards_in_hand_with_priority(
    world: &mut GameplayWorld,
) {
    world.setup_priority_after_blockers_declared_with_two_instants();
}

#[given("combat damage has resolved and Alice still has an instant card in hand with priority")]
fn combat_damage_has_resolved_and_alice_still_has_an_instant_card_in_hand_with_priority(
    world: &mut GameplayWorld,
) {
    world.setup_priority_after_combat_damage_with_instant();
}

#[given("combat damage has resolved and Alice still has two instant cards in hand with priority")]
fn combat_damage_has_resolved_and_alice_still_has_two_instant_cards_in_hand_with_priority(
    world: &mut GameplayWorld,
) {
    world.setup_priority_after_combat_damage_with_two_instants();
}

#[when("Alice casts the instant spell")]
fn alice_casts_the_instant_spell(world: &mut GameplayWorld) {
    world.cast_tracked_spell("Alice");
}

#[when("Alice casts the second instant spell")]
fn alice_casts_the_second_instant_spell(world: &mut GameplayWorld) {
    world.cast_tracked_response_spell("Alice");
}

#[then("the stack contains 2 spells controlled by Alice")]
fn the_stack_contains_two_spells_controlled_by_alice(world: &mut GameplayWorld) {
    assert_eq!(world.game().stack().len(), 2);
    for object in world.game().stack().objects() {
        assert_eq!(object.controller_id(), &GameplayWorld::player_id("Alice"));
    }
}
