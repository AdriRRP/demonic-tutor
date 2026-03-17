#![allow(clippy::expect_used)]
#![allow(clippy::missing_const_for_fn)]
#![allow(clippy::needless_pass_by_ref_mut)]
#![allow(clippy::needless_pass_by_value)]
#![allow(clippy::panic)]
#![allow(clippy::unreachable)]
#![allow(clippy::unwrap_used)]

#[path = "bdd/cleanup.rs"]
mod cleanup;
#[path = "bdd/combat.rs"]
mod combat;
#[path = "bdd/draw_effects.rs"]
mod draw_effects;
#[path = "bdd/life.rs"]
mod life;
#[path = "bdd/spell_casting.rs"]
mod spell_casting;
#[path = "bdd/state_based_actions.rs"]
mod state_based_actions;
#[path = "bdd/turn_progression.rs"]
mod turn_progression;
#[path = "bdd/world.rs"]
mod world;

use cucumber::World as _;
use world::GameplayWorld;

#[tokio::main]
async fn main() {
    GameplayWorld::run("features/turn-flow/lose_on_empty_draw.feature").await;
    GameplayWorld::run("features/turn-flow/turn_progression.feature").await;
    GameplayWorld::run("features/turn-flow/draw_multiple_cards.feature").await;
    GameplayWorld::run("features/spells/cast_creature_spell.feature").await;
    GameplayWorld::run("features/state-based-actions/zero_toughness_creature_dies.feature").await;
    GameplayWorld::run("features/combat/combat_damage_marking.feature").await;
    GameplayWorld::run("features/combat/single_blocker_per_attacker.feature").await;
    GameplayWorld::run("features/combat/creature_destruction.feature").await;
    GameplayWorld::run("features/turn-flow/cleanup_damage_removal.feature").await;
    GameplayWorld::run("features/turn-flow/cleanup_hand_size_discard.feature").await;
    GameplayWorld::run("features/life/lose_on_zero_life.feature").await;
}
