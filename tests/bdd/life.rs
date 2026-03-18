use cucumber::{given, then, when};

use crate::world::GameplayWorld;

#[given(expr = "{word} has {int} life")]
fn player_has_specific_life(world: &mut GameplayWorld, player: String, life: u32) {
    world.setup_player_at_life(&player, life);
    assert_eq!(world.player_life(&player), life);
}

#[when(expr = "{word} loses 1 life")]
fn player_loses_one_life(world: &mut GameplayWorld, player: String) {
    world.adjust_life(&player, -1);
}

#[when(expr = "{word} makes {word} lose {int} life")]
fn player_makes_other_player_lose_life(
    world: &mut GameplayWorld,
    caster: String,
    target: String,
    amount: i32,
) {
    world.adjust_player_life_effect(&caster, &target, -amount);
}

#[when(expr = "{word} makes {word} gain {int} life")]
fn player_makes_other_player_gain_life(
    world: &mut GameplayWorld,
    caster: String,
    target: String,
    amount: i32,
) {
    world.adjust_player_life_effect(&caster, &target, amount);
}

#[then(expr = "{word} has {int} life")]
fn player_has_life(world: &mut GameplayWorld, player: String, life: u32) {
    assert_eq!(world.player_life(&player), life);
}
