#![allow(clippy::expect_used)]
#![allow(clippy::missing_const_for_fn)]
#![allow(clippy::large_stack_frames)]
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
#[path = "bdd/stack_foundation.rs"]
mod stack_foundation;
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
    GameplayWorld::run("features/stack/stack_foundation.feature").await;
    GameplayWorld::run("features/stack/respond_with_instant_spell.feature").await;
    GameplayWorld::run("features/stack/respond_in_upkeep_window.feature").await;
    GameplayWorld::run("features/stack/respond_in_draw_window.feature").await;
    GameplayWorld::run("features/stack/respond_in_first_main_window.feature").await;
    GameplayWorld::run("features/stack/respond_in_second_main_window.feature").await;
    GameplayWorld::run("features/stack/respond_in_end_step_window.feature").await;
    GameplayWorld::run("features/stack/respond_in_beginning_of_combat_window.feature").await;
    GameplayWorld::run("features/stack/respond_after_attackers.feature").await;
    GameplayWorld::run("features/stack/respond_after_blockers.feature").await;
    GameplayWorld::run("features/stack/respond_after_combat_damage.feature").await;
    GameplayWorld::run("features/stack/cast_instant_in_upkeep_window.feature").await;
    GameplayWorld::run("features/stack/cast_second_instant_in_upkeep_window.feature").await;
    GameplayWorld::run("features/stack/cast_instant_in_draw_window.feature").await;
    GameplayWorld::run("features/stack/cast_second_instant_in_draw_window.feature").await;
    GameplayWorld::run("features/stack/cast_instant_in_second_main_window.feature").await;
    GameplayWorld::run("features/stack/cast_second_instant_in_second_main_window.feature").await;
    GameplayWorld::run("features/stack/cast_instant_in_end_step_window.feature").await;
    GameplayWorld::run("features/stack/cast_instant_in_beginning_of_combat_window.feature").await;
    GameplayWorld::run("features/stack/cast_instant_after_attackers.feature").await;
    GameplayWorld::run("features/stack/cast_instant_after_blockers.feature").await;
    GameplayWorld::run("features/stack/cast_instant_after_combat_damage.feature").await;
    GameplayWorld::run("features/turn-flow/upkeep_priority_window.feature").await;
    GameplayWorld::run("features/turn-flow/draw_priority_window.feature").await;
    GameplayWorld::run("features/turn-flow/main_phase_priority_window.feature").await;
    GameplayWorld::run("features/turn-flow/end_step_priority_window.feature").await;
    GameplayWorld::run("features/turn-flow/lose_on_empty_draw.feature").await;
    GameplayWorld::run("features/turn-flow/turn_progression.feature").await;
    GameplayWorld::run("features/turn-flow/draw_multiple_cards.feature").await;
    GameplayWorld::run("features/spells/cast_creature_spell.feature").await;
    GameplayWorld::run("features/combat/combat_priority_windows.feature").await;
    GameplayWorld::run("features/combat/beginning_of_combat_priority_window.feature").await;
    GameplayWorld::run("features/combat/post_combat_damage_priority_window.feature").await;
    GameplayWorld::run("features/state-based-actions/zero_toughness_creature_dies.feature").await;
    GameplayWorld::run("features/combat/combat_damage_marking.feature").await;
    GameplayWorld::run("features/combat/single_blocker_per_attacker.feature").await;
    GameplayWorld::run("features/combat/creature_destruction.feature").await;
    GameplayWorld::run("features/turn-flow/cleanup_damage_removal.feature").await;
    GameplayWorld::run("features/turn-flow/cleanup_hand_size_discard.feature").await;
    GameplayWorld::run("features/life/lose_on_zero_life.feature").await;
}
