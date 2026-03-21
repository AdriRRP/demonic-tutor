use cucumber::then;
use demonictutor::SpellCastOutcome;

use crate::world::GameplayWorld;

#[then("the card leaves Alice's hand")]
fn the_card_leaves_alices_hand(world: &mut GameplayWorld) {
    let card_id = world
        .tracked_card_id
        .as_ref()
        .expect("tracked card should exist");
    assert!(!world.hand_contains("Alice", card_id));
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
