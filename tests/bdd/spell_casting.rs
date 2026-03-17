use cucumber::{given, then, when};
use demonictutor::{CardInstance, CardType, SpellCastOutcome};

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

#[given("Alice has cast a creature spell and still holds priority with Bob's instant in hand")]
fn alice_has_cast_a_creature_spell_and_still_holds_priority_with_bobs_instant_in_hand(
    world: &mut GameplayWorld,
) {
    world.setup_spell_response_stack();
    world.ensure_tracked_land_provides_mana();
    world.cast_tracked_spell("Alice");
    alice_has_priority(world);
}

#[given(
    "Alice has cast an instant spell and still holds priority with Bob's creature card in hand"
)]
fn alice_has_cast_an_instant_spell_and_still_holds_priority_with_bobs_creature_card_in_hand(
    world: &mut GameplayWorld,
) {
    world.setup_invalid_noninstant_response();
    world.cast_tracked_spell("Alice");
    alice_has_priority(world);
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

#[when("Bob casts the instant response spell")]
fn bob_casts_the_instant_response_spell(world: &mut GameplayWorld) {
    world.cast_tracked_response_spell("Bob");
}

#[when("Bob tries to cast the creature response spell")]
fn bob_tries_to_cast_the_creature_response_spell(world: &mut GameplayWorld) {
    world.cast_tracked_response_spell("Bob");
}

#[when(expr = "{word} passes priority")]
fn player_passes_priority(world: &mut GameplayWorld, player: String) {
    world.pass_priority(&player);
}

#[given("Alice has priority")]
#[then("Alice has priority")]
fn alice_has_priority(world: &mut GameplayWorld) {
    let priority = world
        .game()
        .priority()
        .expect("priority window should be open");
    assert_eq!(
        priority.current_holder(),
        &GameplayWorld::player_id("Alice")
    );
}

#[then("the card leaves Alice's hand")]
fn the_card_leaves_alices_hand(world: &mut GameplayWorld) {
    let card_id = world
        .tracked_card_id
        .as_ref()
        .expect("tracked card should exist");
    assert!(!world.hand_contains("Alice", card_id));
}

#[then("the spell is on the stack under Alice's control")]
fn the_spell_is_on_the_stack_under_alices_control(world: &mut GameplayWorld) {
    let event = world
        .last_spell_put_on_stack
        .as_ref()
        .expect("expected a SpellPutOnStack event");
    let top = world
        .game()
        .stack()
        .top()
        .expect("stack should contain a spell");
    assert_eq!(event.player_id, GameplayWorld::player_id("Alice"));
    assert_eq!(top.controller_id(), &GameplayWorld::player_id("Alice"));
}

#[then("Bob's instant is on top of the stack under Bob's control")]
fn bobs_instant_is_on_top_of_the_stack_under_bobs_control(world: &mut GameplayWorld) {
    let top = world
        .game()
        .stack()
        .top()
        .expect("stack should contain a top spell");
    assert_eq!(top.controller_id(), &GameplayWorld::player_id("Bob"));
}

#[then("Alice has priority again")]
fn alice_has_priority_again(world: &mut GameplayWorld) {
    let priority = world
        .game()
        .priority()
        .expect("priority window should be open");
    assert_eq!(
        priority.current_holder(),
        &GameplayWorld::player_id("Alice")
    );
}

#[then("Alice's original spell remains on the stack")]
fn alices_original_spell_remains_on_the_stack(world: &mut GameplayWorld) {
    let tracked_card_id = world
        .tracked_card_id
        .as_ref()
        .expect("original tracked card should exist");
    assert_eq!(world.game().stack().len(), 1);
    assert_eq!(
        world
            .game()
            .stack()
            .top()
            .expect("stack should contain original spell")
            .source_card_id(),
        tracked_card_id
    );
}

#[then("the spell has not resolved yet")]
fn the_spell_has_not_resolved_yet(world: &mut GameplayWorld) {
    assert!(world.last_spell_cast.is_none());
}

#[given("Bob has priority")]
#[then("Bob has priority")]
fn bob_has_priority(world: &mut GameplayWorld) {
    let priority = world
        .game()
        .priority()
        .expect("priority window should be open after casting");
    assert_eq!(priority.current_holder(), &GameplayWorld::player_id("Bob"));
}

#[then("the game emits SpellPutOnStack")]
fn the_game_emits_spell_put_on_stack(world: &mut GameplayWorld) {
    assert!(world.last_spell_put_on_stack.is_some());
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

#[then("the game emits StackTopResolved")]
fn the_game_emits_stack_top_resolved(world: &mut GameplayWorld) {
    assert!(world.last_stack_top_resolved.is_some());
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

#[then("the action is rejected because only instant responses are currently supported")]
fn the_action_is_rejected_because_only_instant_responses_are_currently_supported(
    world: &mut GameplayWorld,
) {
    let error = world
        .last_error
        .as_ref()
        .expect("response cast should be rejected");
    assert!(
        error.contains("only supports instant response spells"),
        "unexpected error: {error}"
    );
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
