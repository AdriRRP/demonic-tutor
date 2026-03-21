use cucumber::{given, when};
use demonictutor::{CardInstance, CardType};

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
        .hand()
        .cards()
        .iter()
        .find(|card: &&CardInstance| card.id() == card_id)
        .expect("creature card should be in hand");

    assert_eq!(card.card_type(), &CardType::Creature);
    assert_eq!(card.creature_stats(), Some((2, 2)));
}

#[given("Alice has enough mana to pay its cost")]
fn alice_has_enough_mana_to_pay_its_cost(world: &mut GameplayWorld) {
    world.ensure_tracked_land_provides_mana();
    assert_eq!(world.player("Alice").mana(), 1);
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
        .hand()
        .cards()
        .iter()
        .find(|card: &&CardInstance| card.id() == card_id)
        .expect("land card should be in hand");

    assert_eq!(card.card_type(), &CardType::Land);
}

#[when("Alice tries to cast the card as a spell")]
fn alice_tries_to_cast_the_card_as_a_spell(world: &mut GameplayWorld) {
    world.try_cast_tracked_spell("Alice");
}
