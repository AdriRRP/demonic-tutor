use cucumber::{given, then, when};

use crate::world::GameplayWorld;

#[given(expr = "{word} has 1 life")]
fn player_has_one_life(world: &mut GameplayWorld, player: String) {
    world.setup_player_at_life(&player, 1);
    assert_eq!(world.player_life(&player), 1);
}

#[when(expr = "{word} loses 1 life")]
fn player_loses_one_life(world: &mut GameplayWorld, player: String) {
    world.adjust_life(&player, -1);
}

#[then(expr = "{word} has 0 life")]
fn player_has_zero_life(world: &mut GameplayWorld, player: String) {
    assert_eq!(world.player_life(&player), 0);
}
