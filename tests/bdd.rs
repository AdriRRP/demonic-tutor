#![allow(clippy::expect_used)]
#![allow(clippy::missing_const_for_fn)]
#![allow(clippy::needless_pass_by_ref_mut)]
#![allow(clippy::needless_pass_by_value)]
#![allow(clippy::panic)]
#![allow(clippy::unreachable)]
#![allow(clippy::unwrap_used)]

#[path = "bdd/combat.rs"]
mod combat;
#[path = "bdd/spell_casting.rs"]
mod spell_casting;
#[path = "bdd/turn_progression.rs"]
mod turn_progression;
#[path = "bdd/world.rs"]
mod world;

use cucumber::World as _;
use world::GameplayWorld;

#[tokio::main]
async fn main() {
    GameplayWorld::run("features/turn-flow/turn_progression.feature").await;
    GameplayWorld::run("features/spells/cast_creature_spell.feature").await;
    GameplayWorld::run("features/combat/combat_damage_marking.feature").await;
    GameplayWorld::run("features/combat/creature_destruction.feature").await;
    GameplayWorld::run("features/turn-flow/cleanup_damage_removal.feature").await;
}
