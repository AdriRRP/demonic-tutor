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
#[path = "bdd/spell_casting/mod.rs"]
mod spell_casting;
#[path = "bdd/stack_foundation.rs"]
mod stack_foundation;
#[path = "bdd/state_based_actions.rs"]
mod state_based_actions;
#[path = "bdd/turn_progression.rs"]
mod turn_progression;
#[path = "bdd/world/mod.rs"]
mod world;
#[path = "bdd/zones.rs"]
mod zones;

use cucumber::World as _;
use world::GameplayWorld;

#[allow(clippy::future_not_send)]
async fn run_features(features: &[&str]) {
    for feature in features {
        GameplayWorld::run(feature).await;
    }
}

#[allow(clippy::future_not_send)]
async fn run_stack_features() {
    run_features(&[
        "features/stack/stack_foundation.feature",
        "features/stack/respond_with_instant_spell.feature",
        "features/stack/respond_with_paid_instant_spell.feature",
        "features/stack/tap_land_for_mana_does_not_use_the_stack.feature",
        "features/stack/respond_in_upkeep_window.feature",
        "features/stack/respond_in_draw_window.feature",
        "features/stack/respond_in_first_main_window.feature",
        "features/stack/respond_in_second_main_window.feature",
        "features/stack/respond_in_end_step_window.feature",
        "features/stack/respond_in_beginning_of_combat_window.feature",
        "features/stack/respond_after_attackers.feature",
        "features/stack/respond_with_second_instant_in_declare_blockers_window.feature",
        "features/stack/respond_after_blockers.feature",
        "features/stack/respond_with_second_instant_in_combat_damage_window.feature",
        "features/stack/respond_with_second_instant_in_end_of_combat_window.feature",
        "features/stack/respond_after_combat_damage.feature",
        "features/stack/respond_with_second_instant_spell.feature",
        "features/stack/respond_with_second_instant_in_upkeep_window.feature",
        "features/stack/respond_with_second_instant_in_draw_window.feature",
        "features/stack/respond_with_second_instant_in_beginning_of_combat_window.feature",
        "features/stack/respond_with_second_instant_in_end_step_window.feature",
        "features/stack/respond_with_second_instant_in_first_main_window.feature",
        "features/stack/respond_with_second_instant_in_second_main_window.feature",
        "features/stack/cast_instant_in_upkeep_window.feature",
        "features/stack/cast_second_instant_in_upkeep_window.feature",
        "features/stack/cast_instant_in_draw_window.feature",
        "features/stack/cast_second_instant_in_draw_window.feature",
        "features/stack/cast_instant_in_first_main_window.feature",
        "features/stack/targeted_instant_spell.feature",
        "features/stack/cast_sorcery_in_main_window.feature",
        "features/stack/cast_creature_in_second_main_window.feature",
        "features/stack/cast_artifact_in_main_window.feature",
        "features/stack/cast_enchantment_in_main_window.feature",
        "features/stack/cast_planeswalker_in_main_window.feature",
        "features/stack/cast_second_instant_in_first_main_window.feature",
        "features/stack/cast_instant_in_second_main_window.feature",
        "features/stack/reject_sorcery_response.feature",
        "features/stack/reject_planeswalker_response.feature",
        "features/stack/sorcery_speed_spells_require_active_player_priority.feature",
        "features/stack/cast_second_instant_in_second_main_window.feature",
        "features/stack/cast_instant_in_end_step_window.feature",
        "features/stack/cast_second_instant_in_end_step_window.feature",
        "features/stack/cast_instant_in_beginning_of_combat_window.feature",
        "features/stack/cast_second_instant_in_beginning_of_combat_window.feature",
        "features/stack/cast_instant_after_attackers.feature",
        "features/stack/cast_second_instant_after_attackers.feature",
        "features/stack/cast_instant_after_blockers.feature",
        "features/stack/cast_second_instant_after_blockers.feature",
        "features/stack/cast_instant_after_combat_damage.feature",
        "features/stack/cast_second_instant_after_combat_damage.feature",
        "features/stack/respond_with_flash_artifact_spell.feature",
        "features/stack/respond_with_flash_enchantment_spell.feature",
        "features/stack/cast_flash_artifact_in_beginning_of_combat_window.feature",
        "features/stack/cast_flash_artifact_after_blockers.feature",
        "features/stack/cast_flash_artifact_after_combat_damage.feature",
        "features/stack/cast_flash_enchantment_in_beginning_of_combat_window.feature",
        "features/stack/cast_flash_enchantment_after_blockers.feature",
        "features/stack/cast_flash_enchantment_after_combat_damage.feature",
        "features/stack/cast_flash_creature_after_blockers.feature",
        "features/stack/cast_flash_creature_after_combat_damage.feature",
        "features/stack/cast_own_turn_priority_artifact_in_upkeep_window.feature",
        "features/stack/cast_own_turn_priority_artifact_in_beginning_of_combat_window.feature",
        "features/stack/cast_own_turn_priority_artifact_after_attackers.feature",
        "features/stack/cast_own_turn_priority_artifact_after_blockers.feature",
        "features/stack/cast_own_turn_priority_artifact_after_combat_damage.feature",
        "features/stack/cast_own_turn_priority_enchantment_in_upkeep_window.feature",
        "features/stack/cast_own_turn_priority_enchantment_in_beginning_of_combat_window.feature",
        "features/stack/cast_own_turn_priority_enchantment_after_attackers.feature",
        "features/stack/cast_own_turn_priority_enchantment_after_blockers.feature",
        "features/stack/cast_own_turn_priority_enchantment_after_combat_damage.feature",
        "features/stack/target_blocking_creature_spell.feature",
        "features/stack/target_opponent_player_spell.feature",
        "features/stack/target_controlled_creature_spell_outside_combat.feature",
        "features/stack/target_controlled_attacking_creature_spell.feature",
        "features/stack/target_controlled_blocking_creature_spell.feature",
        "features/stack/target_opponents_blocking_creature_spell.feature",
        "features/stack/target_opponents_attacking_creature_spell.feature",
        "features/stack/reject_own_turn_priority_artifact_response.feature",
        "features/stack/reject_own_turn_priority_enchantment_response.feature",
    ])
    .await;
}

#[allow(clippy::future_not_send)]
async fn run_turn_flow_features() {
    run_features(&[
        "features/turn-flow/upkeep_priority_window.feature",
        "features/turn-flow/draw_priority_window.feature",
        "features/turn-flow/main_phase_priority_window.feature",
        "features/turn-flow/end_step_priority_window.feature",
        "features/turn-flow/mana_pool_clears_on_phase_advance.feature",
        "features/turn-flow/lose_on_empty_draw.feature",
        "features/turn-flow/turn_progression.feature",
        "features/turn-flow/draw_multiple_cards.feature",
        "features/turn-flow/cleanup_damage_removal.feature",
        "features/turn-flow/cleanup_hand_size_discard.feature",
    ])
    .await;
}

#[allow(clippy::future_not_send)]
async fn run_other_features() {
    run_features(&[
        "features/spells/cast_creature_spell.feature",
        "features/combat/combat_priority_windows.feature",
        "features/combat/beginning_of_combat_priority_window.feature",
        "features/combat/post_combat_damage_priority_window.feature",
        "features/state-based-actions/zero_toughness_creature_dies.feature",
        "features/combat/combat_damage_marking.feature",
        "features/combat/single_blocker_per_attacker.feature",
        "features/combat/creature_destruction.feature",
        "features/life/adjust_player_life_effect.feature",
        "features/life/lose_on_zero_life.feature",
        "features/combat/keyword_abilities.feature",
        "features/zones/exile_zone.feature",
    ])
    .await;
}

#[tokio::main]
async fn main() {
    run_stack_features().await;
    run_turn_flow_features().await;
    run_other_features().await;
}
