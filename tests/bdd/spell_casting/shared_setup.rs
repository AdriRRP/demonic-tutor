use cucumber::{given, when};
use demonictutor::CardType;

use crate::world::GameplayWorld;

#[given(expr = "{word} is the active player in {word}")]
fn player_is_active_in_phase(world: &mut GameplayWorld, player: String, phase: String) {
    let expected_phase = GameplayWorld::phase_from_name(&phase);
    if !world.is_initialized() {
        world.setup_turn_state_satisfying_cleanup(expected_phase, &player, 1);
    }
    assert_eq!(player, "Alice");
    assert_eq!(
        world.game().active_player(),
        &GameplayWorld::player_id(&player)
    );
    assert_eq!(world.game().phase(), &expected_phase);
}

#[given("Alice has a creature card in hand with valid power and toughness")]
fn alice_has_a_creature_card_in_hand_with_valid_power_and_toughness(world: &mut GameplayWorld) {
    world.setup_cast_creature_spell();

    let card_id = world
        .tracked_card_id
        .as_ref()
        .expect("tracked creature card should exist");
    let card = world
        .player("Alice")
        .hand_card(card_id)
        .expect("creature card should be in hand");

    assert_eq!(card.card_type(), &CardType::Creature);
    assert_eq!(card.creature_stats(), Some((2, 2)));
}

#[given("Alice has enough mana to pay its cost")]
fn alice_has_enough_mana_to_pay_its_cost(world: &mut GameplayWorld) {
    world.ensure_tracked_land_provides_mana();
    assert!(world.player("Alice").mana() >= 1);
}

#[given("Alice has a land card in hand")]
fn alice_has_a_land_card_in_hand(world: &mut GameplayWorld) {
    world.setup_cast_land_as_spell();
    let card_id = world
        .tracked_card_id
        .as_ref()
        .expect("tracked land card should exist");
    let card = world
        .player("Alice")
        .hand_card(card_id)
        .expect("land card should be in hand");

    assert_eq!(card.card_type(), &CardType::Land);
}

#[given("Alice is the active player in FirstMain with a green instant card in hand and priority")]
fn alice_is_the_active_player_in_first_main_with_a_green_instant_card_in_hand_and_priority(
    world: &mut GameplayWorld,
) {
    world.setup_cast_green_instant_with_forest();
    assert_eq!(
        world.game().phase(),
        &GameplayWorld::phase_from_name("FirstMain")
    );
    assert_eq!(
        world.game().active_player(),
        &GameplayWorld::player_id("Alice")
    );
}

#[given("Alice is the active player in FirstMain with a green instant card in hand and only a mountain available")]
fn alice_is_the_active_player_in_first_main_with_a_green_instant_card_in_hand_and_only_a_mountain_available(
    world: &mut GameplayWorld,
) {
    world.setup_cast_green_instant_with_mountain();
    assert_eq!(
        world.game().phase(),
        &GameplayWorld::phase_from_name("FirstMain")
    );
    assert_eq!(
        world.game().active_player(),
        &GameplayWorld::player_id("Alice")
    );
}

#[given(
    "Alice is the active player in FirstMain with a mixed green instant card in hand and priority"
)]
fn alice_is_the_active_player_in_first_main_with_a_mixed_green_instant_card_in_hand_and_priority(
    world: &mut GameplayWorld,
) {
    world.setup_cast_mixed_green_instant();
    assert_eq!(
        world.game().phase(),
        &GameplayWorld::phase_from_name("FirstMain")
    );
    assert_eq!(
        world.game().active_player(),
        &GameplayWorld::player_id("Alice")
    );
}

#[given(
    "Alice is the active player in FirstMain with a double green instant card in hand and priority"
)]
fn alice_is_the_active_player_in_first_main_with_a_double_green_instant_card_in_hand_and_priority(
    world: &mut GameplayWorld,
) {
    world.setup_cast_double_green_instant();
    assert_eq!(
        world.game().phase(),
        &GameplayWorld::phase_from_name("FirstMain")
    );
    assert_eq!(
        world.game().active_player(),
        &GameplayWorld::player_id("Alice")
    );
}

#[given(
    "Alice is the active player in FirstMain with a mixed green instant card in hand and only red and white mana available"
)]
fn alice_is_the_active_player_in_first_main_with_a_mixed_green_instant_card_in_hand_and_only_red_and_white_mana_available(
    world: &mut GameplayWorld,
) {
    world.setup_cast_mixed_green_instant_without_green();
    assert_eq!(
        world.game().phase(),
        &GameplayWorld::phase_from_name("FirstMain")
    );
    assert_eq!(
        world.game().active_player(),
        &GameplayWorld::player_id("Alice")
    );
}

#[given("Alice has only red mana available to pay its cost")]
fn alice_has_only_red_mana_available_to_pay_its_cost(world: &mut GameplayWorld) {
    world.ensure_tracked_land_provides_mana();
    assert_eq!(world.player("Alice").mana_pool().red(), 1);
    assert_eq!(world.player("Alice").mana(), 1);
}

#[when("Alice tries to cast the card as a spell")]
fn alice_tries_to_cast_the_card_as_a_spell(world: &mut GameplayWorld) {
    world.try_cast_tracked_spell("Alice");
}
