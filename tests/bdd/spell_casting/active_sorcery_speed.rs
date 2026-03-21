use cucumber::{given, when};
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

#[given("Alice is the active player in Upkeep with an own-turn-priority artifact card in hand and priority")]
fn alice_is_the_active_player_in_upkeep_with_an_own_turn_priority_artifact_card_in_hand_and_priority(
    world: &mut GameplayWorld,
) {
    world.setup_active_priority_window_with_own_turn_artifact(
        "bdd-upkeep-own-turn-artifact-window",
        Phase::Upkeep,
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
