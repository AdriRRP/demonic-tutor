#![allow(clippy::expect_used)]
#![allow(clippy::missing_const_for_fn)]
#![allow(clippy::needless_pass_by_ref_mut)]
#![allow(clippy::needless_pass_by_value)]
#![allow(clippy::panic)]
#![allow(clippy::unreachable)]
#![allow(clippy::unwrap_used)]

#[path = "../unit/support/mod.rs"]
mod support;

use cucumber::{given, then, when, World as _};
use demonictutor::{
    AdvanceTurnCommand, CardDrawn, DealOpeningHandsCommand, DrawKind, Game, GameId, Phase,
    PlayerId, StartGameCommand, TurnProgressed,
};

#[derive(Debug, Default, cucumber::World)]
struct TurnWorld {
    game: Option<Game>,
    last_turn_progressed: Option<TurnProgressed>,
    last_card_drawn: Option<CardDrawn>,
    last_error: Option<String>,
    pre_advance_hand_size: Option<usize>,
    post_advance_hand_size: Option<usize>,
}

impl TurnWorld {
    fn game(&self) -> &Game {
        self.game
            .as_ref()
            .expect("world game should be initialized")
    }

    fn game_mut(&mut self) -> &mut Game {
        self.game
            .as_mut()
            .expect("world game should be initialized")
    }

    fn player_id(alias: &str) -> PlayerId {
        match alias {
            "Alice" => PlayerId::new("player-1"),
            "Bob" => PlayerId::new("player-2"),
            _ => panic!("unknown player alias: {alias}"),
        }
    }

    fn player_hand_size(&self, alias: &str) -> usize {
        let player_id = Self::player_id(alias);
        self.game()
            .players()
            .iter()
            .find(|player| player.id() == &player_id)
            .unwrap_or_else(|| panic!("player should exist: {player_id}"))
            .hand()
            .cards()
            .len()
    }

    fn player_library_size(&self, alias: &str) -> usize {
        let player_id = Self::player_id(alias);
        self.game()
            .players()
            .iter()
            .find(|player| player.id() == &player_id)
            .unwrap_or_else(|| panic!("player should exist: {player_id}"))
            .library()
            .len()
    }

    fn setup_game_at(&mut self, target_phase: Phase, target_player: &str, target_turn: u32) {
        let (mut game, _) = Game::start(StartGameCommand::new(
            GameId::new("bdd-turn-progression"),
            vec![
                support::player_deck("player-1", "deck-1"),
                support::player_deck("player-2", "deck-2"),
            ],
        ))
        .expect("game should start");

        game.deal_opening_hands(&DealOpeningHandsCommand::new(vec![
            support::player_library("player-1", support::filled_library(Vec::new(), 40)),
            support::player_library("player-2", support::filled_library(Vec::new(), 40)),
        ]))
        .expect("opening hands should be dealt");

        let target_player = Self::player_id(target_player);
        for _ in 0..64 {
            if game.phase() == &target_phase
                && game.active_player() == &target_player
                && game.turn_number() == target_turn
            {
                self.game = Some(game);
                self.last_turn_progressed = None;
                self.last_card_drawn = None;
                self.last_error = None;
                self.pre_advance_hand_size = None;
                self.post_advance_hand_size = None;
                return;
            }

            let (_, draw_event) = game
                .advance_turn(AdvanceTurnCommand::new())
                .expect("phase setup should succeed");
            self.last_card_drawn = draw_event;
        }

        panic!(
            "failed to reach target state: phase={target_phase:?}, player={target_player}, turn={target_turn}"
        );
    }
}

#[given(expr = "a two-player game is in {word}")]
fn a_two_player_game_is_in_phase(world: &mut TurnWorld, phase: String) {
    let phase = match phase.as_str() {
        "EndStep" => Phase::EndStep,
        "Draw" => Phase::Draw,
        other => panic!("unsupported phase in BDD pilot: {other}"),
    };

    let (player, turn) = match phase {
        Phase::EndStep => ("Alice", 3),
        Phase::Draw => ("Alice", 1),
        _ => unreachable!(),
    };

    world.setup_game_at(phase, player, turn);
}

#[given(expr = "{word} is the active player")]
fn the_active_player_is(world: &mut TurnWorld, player: String) {
    assert_eq!(world.game().active_player(), &TurnWorld::player_id(&player));
}

#[given(expr = "the current turn number is {int}")]
fn the_current_turn_number_is(world: &mut TurnWorld, turn_number: u32) {
    assert_eq!(world.game().turn_number(), turn_number);
}

#[given(expr = "{word} has at least one card in her library")]
fn player_has_at_least_one_card_in_library(world: &mut TurnWorld, player: String) {
    assert!(world.player_library_size(&player) >= 1);
}

#[when("the game advances the turn")]
fn the_game_advances_the_turn(world: &mut TurnWorld) {
    world.pre_advance_hand_size = Some(world.player_hand_size("Alice"));

    match world.game_mut().advance_turn(AdvanceTurnCommand::new()) {
        Ok((turn_progressed, card_drawn)) => {
            world.last_turn_progressed = Some(turn_progressed);
            world.last_card_drawn = card_drawn;
            world.last_error = None;
        }
        Err(error) => {
            world.last_error = Some(error.to_string());
            world.last_turn_progressed = None;
            world.last_card_drawn = None;
        }
    }

    world.post_advance_hand_size = Some(world.player_hand_size("Alice"));
}

#[then(expr = "{word} becomes the active player")]
fn player_becomes_the_active_player(world: &mut TurnWorld, player: String) {
    assert_eq!(world.game().active_player(), &TurnWorld::player_id(&player));
}

#[then(expr = "the turn number becomes {int}")]
fn the_turn_number_becomes(world: &mut TurnWorld, turn_number: u32) {
    assert_eq!(world.game().turn_number(), turn_number);
}

#[then(expr = "the phase becomes {word}")]
fn the_phase_becomes(world: &mut TurnWorld, phase: String) {
    let expected = match phase.as_str() {
        "Untap" => Phase::Untap,
        "FirstMain" => Phase::FirstMain,
        other => panic!("unsupported phase assertion in BDD pilot: {other}"),
    };

    assert_eq!(world.game().phase(), &expected);
}

#[then("the game emits TurnProgressed")]
fn the_game_emits_turn_progressed(world: &mut TurnWorld) {
    assert!(
        world.last_error.is_none(),
        "unexpected error: {:?}",
        world.last_error
    );
    assert!(world.last_turn_progressed.is_some());
}

#[then(expr = "{word} draws one card")]
fn player_draws_one_card(world: &mut TurnWorld, player: String) {
    let pre = world
        .pre_advance_hand_size
        .expect("pre-advance hand size recorded");
    let post = world
        .post_advance_hand_size
        .expect("post-advance hand size recorded");

    assert_eq!(player, "Alice");
    assert_eq!(post, pre + 1);
}

#[then(expr = "the game emits CardDrawn with draw kind {word}")]
fn the_game_emits_card_drawn_with_kind(world: &mut TurnWorld, draw_kind: String) {
    let expected = match draw_kind.as_str() {
        "TurnStep" => DrawKind::TurnStep,
        other => panic!("unsupported draw kind assertion in BDD pilot: {other}"),
    };

    let event = world
        .last_card_drawn
        .as_ref()
        .expect("expected a CardDrawn event");
    assert_eq!(event.draw_kind, expected);
}

#[tokio::main]
async fn main() {
    TurnWorld::run("features/turn-flow/turn_progression.feature").await;
}
