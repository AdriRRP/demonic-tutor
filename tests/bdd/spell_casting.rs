use cucumber::{given, then, when};
use demonictutor::{CardInstance, CardType, SpellCastOutcome};

use crate::world::GameplayWorld;

#[given(expr = "{word} is the active player in {word}")]
fn player_is_active_in_phase(world: &mut GameplayWorld, player: String, phase: String) {
    let expected_phase = GameplayWorld::phase_from_name(&phase);
    if !world.is_initialized() {
        world.setup_turn_state(expected_phase, &player, 1);
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

#[when("Alice casts the creature spell")]
fn alice_casts_the_creature_spell(world: &mut GameplayWorld) {
    world.cast_tracked_spell("Alice");
}

#[then("the card leaves Alice's hand")]
fn the_card_leaves_alices_hand(world: &mut GameplayWorld) {
    let card_id = world
        .tracked_card_id
        .as_ref()
        .expect("tracked card should exist");
    assert!(!world.hand_contains("Alice", card_id));
}

#[then("the card enters Alice's battlefield")]
fn the_card_enters_alices_battlefield(world: &mut GameplayWorld) {
    let card_id = world
        .tracked_card_id
        .as_ref()
        .expect("tracked card should exist");
    assert!(world.battlefield_contains("Alice", card_id));
}

#[then("the card has summoning sickness")]
fn the_card_has_summoning_sickness(world: &mut GameplayWorld) {
    assert!(world.tracked_card("Alice").has_summoning_sickness());
}

#[then(expr = "the game emits SpellCast with outcome {word}")]
fn the_game_emits_spell_cast_with_outcome(world: &mut GameplayWorld, outcome: String) {
    let expected = match outcome.as_str() {
        "EnteredBattlefield" => SpellCastOutcome::EnteredBattlefield,
        "ResolvedToGraveyard" => SpellCastOutcome::ResolvedToGraveyard,
        other => panic!("unsupported spell outcome in BDD suite: {other}"),
    };

    let event = world
        .last_spell_cast
        .as_ref()
        .expect("expected a SpellCast event");
    assert!(matches!(
        (&event.outcome, expected),
        (
            SpellCastOutcome::EnteredBattlefield,
            SpellCastOutcome::EnteredBattlefield
        ) | (
            SpellCastOutcome::ResolvedToGraveyard,
            SpellCastOutcome::ResolvedToGraveyard
        )
    ));
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
    world.cast_tracked_spell("Alice");
}

#[then("the action is rejected")]
fn the_action_is_rejected(world: &mut GameplayWorld) {
    assert!(world.last_error.is_some());
}

#[then("the land remains in Alice's hand")]
fn the_land_remains_in_alices_hand(world: &mut GameplayWorld) {
    let card_id = world
        .tracked_card_id
        .as_ref()
        .expect("tracked land should exist");
    assert!(world.hand_contains("Alice", card_id));
}
