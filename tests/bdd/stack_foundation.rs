use cucumber::{given, then};

use crate::world::GameplayWorld;

#[given("a new two-player game has started")]
fn a_new_two_player_game_has_started(world: &mut GameplayWorld) {
    world.setup_started_game("bdd-stack-foundation");
}

#[then("the stack is empty")]
fn the_stack_is_empty(world: &mut GameplayWorld) {
    assert!(world.game().stack().is_empty());
    assert_eq!(world.game().stack().len(), 0);
}

#[then("no priority window is open")]
fn no_priority_window_is_open(world: &mut GameplayWorld) {
    assert!(world.game().priority().is_none());
    assert!(!world.game().has_open_priority_window());
}
