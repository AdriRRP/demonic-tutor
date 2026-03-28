//! BDD coverage for bdd turn progression.

use {
    crate::world::GameplayWorld,
    cucumber::{given, then, when},
    demonictutor::{DrawKind, GameEndReason, Phase},
};

#[given(expr = "a two-player game is in {word}")]
fn a_two_player_game_is_in_phase(world: &mut GameplayWorld, phase: String) {
    let phase = GameplayWorld::phase_from_name(&phase);

    let (player, turn) = match phase {
        Phase::EndStep => ("Alice", 3),
        Phase::Untap
        | Phase::Upkeep
        | Phase::Draw
        | Phase::FirstMain
        | Phase::BeginningOfCombat
        | Phase::DeclareAttackers
        | Phase::DeclareBlockers
        | Phase::CombatDamage
        | Phase::EndOfCombat
        | Phase::SecondMain
        | Phase::Setup => ("Alice", 1),
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

#[given(expr = "{word} has no cards in her library")]
fn player_has_no_cards_in_her_library(world: &mut GameplayWorld, player: String) {
    world.setup_upkeep_with_empty_library();
    assert_eq!(player, "Alice");
    assert_eq!(world.player_library_size(&player), 0);
}

#[given("Alice is the active player in Upkeep with a land on the battlefield")]
fn alice_is_the_active_player_in_upkeep_with_a_land_on_the_battlefield(world: &mut GameplayWorld) {
    world.setup_upkeep_with_land_on_battlefield();
    assert_eq!(world.game().phase(), &Phase::Upkeep);
    assert_eq!(
        world.game().active_player(),
        &GameplayWorld::player_id("Alice")
    );
}

#[when("the game advances the turn")]
fn the_game_advances_the_turn(world: &mut GameplayWorld) {
    world.advance_turn();
}

#[when("Alice taps her land for mana")]
fn alice_taps_her_land_for_mana(world: &mut GameplayWorld) {
    world.tap_tracked_land_for_mana("Alice");
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

#[then(expr = "the game emits GameEnded with reason {word}")]
fn the_game_emits_game_ended_with_reason(world: &mut GameplayWorld, reason: String) {
    let expected = match reason.as_str() {
        "EmptyLibraryDraw" => GameEndReason::EmptyLibraryDraw,
        "ZeroLife" => GameEndReason::ZeroLife,
        "SimultaneousZeroLife" => GameEndReason::SimultaneousZeroLife,
        other => panic!("unsupported game-end reason in BDD suite: {other}"),
    };

    let event = world
        .last_game_ended
        .as_ref()
        .expect("expected a GameEnded event");
    assert_eq!(event.reason, expected);
}

#[then(expr = "{word} loses the game")]
fn player_loses_the_game(world: &mut GameplayWorld, player: String) {
    let event = world
        .last_game_ended
        .as_ref()
        .expect("expected a GameEnded event");
    assert_eq!(event.loser_id, Some(GameplayWorld::player_id(&player)));
}

#[then(expr = "{word} wins the game")]
fn player_wins_the_game(world: &mut GameplayWorld, player: String) {
    let event = world
        .last_game_ended
        .as_ref()
        .expect("expected a GameEnded event");
    assert_eq!(event.winner_id, Some(GameplayWorld::player_id(&player)));
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

#[then(expr = "{word} has {int} mana")]
fn player_has_mana(world: &mut GameplayWorld, player: String, amount: u32) {
    assert_eq!(world.player(&player).mana(), amount);
}
