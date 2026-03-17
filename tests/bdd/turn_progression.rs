use cucumber::{given, then, when};
use demonictutor::{DrawKind, Phase};

use crate::world::GameplayWorld;

#[given(expr = "a two-player game is in {word}")]
fn a_two_player_game_is_in_phase(world: &mut GameplayWorld, phase: String) {
    let phase = GameplayWorld::phase_from_name(&phase);

    let (player, turn) = match phase {
        Phase::EndStep => ("Alice", 3),
        Phase::Draw => ("Alice", 1),
        _ => panic!("unsupported phase in turn-progression feature"),
    };

    world.setup_turn_state_satisfying_cleanup(phase, player, turn);
}

#[given(expr = "{word} is the active player")]
fn the_active_player_is(world: &mut GameplayWorld, player: String) {
    assert_eq!(
        world.game().active_player(),
        &GameplayWorld::player_id(&player)
    );
}

#[given(expr = "the current turn number is {int}")]
fn the_current_turn_number_is(world: &mut GameplayWorld, turn_number: u32) {
    assert_eq!(world.game().turn_number(), turn_number);
}

#[given(expr = "{word} has at least one card in her library")]
fn player_has_at_least_one_card_in_library(world: &mut GameplayWorld, player: String) {
    assert!(world.player_library_size(&player) >= 1);
}

#[when("the game advances the turn")]
fn the_game_advances_the_turn(world: &mut GameplayWorld) {
    world.advance_turn();
}

#[then(expr = "{word} becomes the active player")]
fn player_becomes_the_active_player(world: &mut GameplayWorld, player: String) {
    assert_eq!(
        world.game().active_player(),
        &GameplayWorld::player_id(&player)
    );
}

#[then(expr = "the turn number becomes {int}")]
fn the_turn_number_becomes(world: &mut GameplayWorld, turn_number: u32) {
    assert_eq!(world.game().turn_number(), turn_number);
}

#[then(expr = "the phase becomes {word}")]
fn the_phase_becomes(world: &mut GameplayWorld, phase: String) {
    assert_eq!(
        world.game().phase(),
        &GameplayWorld::phase_from_name(&phase)
    );
}

#[then("the game emits TurnProgressed")]
fn the_game_emits_turn_progressed(world: &mut GameplayWorld) {
    assert!(
        world.last_error.is_none(),
        "unexpected error: {:?}",
        world.last_error
    );
    assert!(world.last_turn_progressed.is_some());
}

#[then(expr = "{word} draws one card")]
fn player_draws_one_card(world: &mut GameplayWorld, player: String) {
    let pre = world
        .pre_advance_hand_size
        .expect("pre-advance hand size should exist");
    let post = world
        .post_advance_hand_size
        .expect("post-advance hand size should exist");

    assert_eq!(player, "Alice");
    assert_eq!(post, pre + 1);
}

#[then(expr = "the game emits CardDrawn with draw kind {word}")]
fn the_game_emits_card_drawn_with_kind(world: &mut GameplayWorld, draw_kind: String) {
    let expected = match draw_kind.as_str() {
        "TurnStep" => DrawKind::TurnStep,
        other => panic!("unsupported draw kind assertion in BDD suite: {other}"),
    };

    let event = world
        .last_card_drawn
        .as_ref()
        .expect("expected a CardDrawn event");
    assert_eq!(event.draw_kind, expected);
}
