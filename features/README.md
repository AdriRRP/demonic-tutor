# Gameplay Features — DemonicTutor

This directory contains repository-owned Gherkin specifications for DemonicTutor gameplay.

These files are not a literal transcription of the Magic Comprehensive Rules. They describe the **observable behavior the repository supports or intentionally tracks historically**.

## What feature files are for

Feature files exist to make gameplay behavior:

- readable
- reviewable
- traceable to slices and rule areas
- harder to accidentally regress during refactors

Some features are executable with `cucumber-rs`; others are kept as reference or historical artifacts when that gives the repository a clearer semantic record.

## Status model

Each feature should begin with metadata comments describing:

- `status`
- `rules`
- `slices`

Example:

```gherkin
# status: implemented
# rules: 601.1, 601.2
# slices: cast-spell.md, pay-mana-cost.md
```

Allowed status values:

- `implemented`
- `proposed`
- `historical`

## Execution

Executable BDD suite:

```bash
cargo test --test bdd
```

Conventional unit and integration-style behavior tests:

```bash
cargo test --test unit
```

Repository-wide validation:

```bash
./scripts/check-all.sh
```

## Feature organization

The directory is grouped by gameplay area rather than by testing technology.

### `features/stack/`

Stack and priority behavior:

- stack foundation
- active-player instant casting in supported windows
- non-active instant responses after the first pass
- self-stacking by the current priority holder
- stack-free land mana production in the currently supported priority windows
- minimal colored mana support for `Forest`, `Mountain`, `Plains`, `Island`, `Swamp`, and single-color instant costs
- sorcery-speed legality in main phases
- targeted instant subset against players and creatures
- combat-relative targeted spells against attacking and blocking creatures
- actor-relative combat-targeted spells such as `blocking creature you control` and `attacking creature an opponent controls`
- minimal `Flash`-like creature casts in supported combat windows

### `features/turn-flow/`

Turn and step progression:

- upkeep, draw, main-phase, and end-step priority windows
- draw progression
- transient mana pool clearing on phase advance
- cleanup discard
- cleanup damage removal
- losing on empty draw

### `features/combat/`

Combat behavior:

- explicit combat subphases
- beginning of combat and combat-step progression
- declare attackers / declare blockers steps
- combat damage
- single-blocker simplification
- creature destruction after damage
- keyword-ability blocking legality

### `features/life/`

Life total behavior:

- explicit targeted life effects
- losing at zero life

### `features/state-based-actions/`

Currently supported SBA subset:

- zero-toughness creature death
- shared state-based-actions review

### `features/zones/`

Zone behavior outside the core battlefield/hand/library loop:

- exile zone and explicit exile action

### `features/spells/`

Spell-behavior files that predate or complement the current stack organization.

## Current executable coverage

The following features are currently executed by `tests/bdd.rs`, grouped by area.

### Stack

- `features/stack/stack_foundation.feature`
- `features/stack/respond_with_instant_spell.feature`
- `features/stack/respond_with_paid_instant_spell.feature`
- `features/stack/tap_land_for_mana_does_not_use_the_stack.feature`
- `features/stack/respond_in_upkeep_window.feature`
- `features/stack/respond_in_draw_window.feature`
- `features/stack/respond_in_first_main_window.feature`
- `features/stack/respond_in_second_main_window.feature`
- `features/stack/respond_in_end_step_window.feature`
- `features/stack/respond_in_beginning_of_combat_window.feature`
- `features/stack/respond_after_attackers.feature`
- `features/stack/respond_with_second_instant_in_declare_blockers_window.feature`
- `features/stack/respond_after_blockers.feature`
- `features/stack/respond_with_second_instant_in_combat_damage_window.feature`
- `features/stack/respond_after_combat_damage.feature`
- `features/stack/respond_with_second_instant_spell.feature`
- `features/stack/respond_with_second_instant_in_upkeep_window.feature`
- `features/stack/respond_with_second_instant_in_draw_window.feature`
- `features/stack/respond_with_second_instant_in_beginning_of_combat_window.feature`
- `features/stack/respond_with_second_instant_in_end_step_window.feature`
- `features/stack/respond_with_second_instant_in_end_of_combat_window.feature`
- `features/stack/respond_with_second_instant_in_first_main_window.feature`
- `features/stack/respond_with_second_instant_in_second_main_window.feature`
- `features/stack/cast_instant_in_upkeep_window.feature`
- `features/stack/cast_second_instant_in_upkeep_window.feature`
- `features/stack/cast_instant_in_draw_window.feature`
- `features/stack/cast_second_instant_in_draw_window.feature`
- `features/stack/cast_instant_in_first_main_window.feature`
- `features/stack/cast_green_instant_in_first_main_window.feature`
- `features/stack/cast_mixed_green_instant_in_first_main_window.feature`
- `features/stack/cast_double_green_instant_in_first_main_window.feature`
- `features/stack/reject_mixed_green_instant_without_green.feature`
- `features/stack/targeted_instant_spell.feature`
- `features/stack/target_self_player_when_rule_allows_it.feature`
- `features/stack/cast_sorcery_in_main_window.feature`
- `features/stack/cast_creature_in_second_main_window.feature`
- `features/stack/cast_artifact_in_main_window.feature`
- `features/stack/cast_enchantment_in_main_window.feature`
- `features/stack/cast_planeswalker_in_main_window.feature`
- `features/stack/cast_second_instant_in_first_main_window.feature`
- `features/stack/cast_instant_in_second_main_window.feature`
- `features/stack/reject_sorcery_response.feature`
- `features/stack/reject_planeswalker_response.feature`
- `features/stack/reject_green_instant_with_only_red_mana.feature`
- `features/stack/sorcery_speed_spells_require_active_player_priority.feature`
- `features/stack/cast_second_instant_in_second_main_window.feature`
- `features/stack/cast_instant_in_end_step_window.feature`
- `features/stack/cast_second_instant_in_end_step_window.feature`
- `features/stack/cast_instant_in_beginning_of_combat_window.feature`
- `features/stack/cast_second_instant_in_beginning_of_combat_window.feature`
- `features/stack/cast_instant_after_attackers.feature`
- `features/stack/cast_second_instant_after_attackers.feature`
- `features/stack/cast_instant_after_blockers.feature`
- `features/stack/cast_second_instant_after_blockers.feature`
- `features/stack/cast_instant_after_combat_damage.feature`
- `features/stack/cast_second_instant_after_combat_damage.feature`
- `features/stack/respond_with_flash_artifact_spell.feature`
- `features/stack/respond_with_flash_enchantment_spell.feature`
- `features/stack/cast_flash_artifact_in_beginning_of_combat_window.feature`
- `features/stack/cast_flash_artifact_after_blockers.feature`
- `features/stack/cast_flash_artifact_after_combat_damage.feature`
- `features/stack/cast_flash_enchantment_in_beginning_of_combat_window.feature`
- `features/stack/cast_flash_enchantment_after_blockers.feature`
- `features/stack/cast_flash_enchantment_after_combat_damage.feature`
- `features/stack/cast_flash_creature_after_blockers.feature`
- `features/stack/cast_flash_creature_after_combat_damage.feature`
- `features/stack/cast_own_turn_priority_artifact_in_upkeep_window.feature`
- `features/stack/cast_own_turn_priority_artifact_in_beginning_of_combat_window.feature`
- `features/stack/cast_own_turn_priority_artifact_after_attackers.feature`
- `features/stack/cast_own_turn_priority_artifact_after_blockers.feature`
- `features/stack/cast_own_turn_priority_artifact_after_combat_damage.feature`
- `features/stack/cast_own_turn_priority_enchantment_in_upkeep_window.feature`
- `features/stack/cast_own_turn_priority_enchantment_in_beginning_of_combat_window.feature`
- `features/stack/cast_own_turn_priority_enchantment_after_attackers.feature`
- `features/stack/cast_own_turn_priority_enchantment_after_blockers.feature`
- `features/stack/cast_own_turn_priority_enchantment_after_combat_damage.feature`
- `features/stack/target_blocking_creature_spell.feature`
- `features/stack/target_opponent_player_spell.feature`
- `features/stack/target_controlled_creature_spell_outside_combat.feature`
- `features/stack/target_opponents_creature_in_first_main.feature`
- `features/stack/destroy_target_creature_foundation.feature`
- `features/stack/exile_target_creature_foundation.feature`
- `features/stack/exile_target_card_from_graveyard.feature`
- `features/stack/pump_target_creature_until_end_of_turn.feature`
- `features/stack/pump_spell_changes_combat_outcome.feature`
- `features/stack/reject_controlled_creature_for_opponents_creature_spell.feature`
- `features/stack/resolve_opponents_creature_spell_in_first_main.feature`
- `features/stack/target_controlled_attacking_creature_spell.feature`
- `features/stack/target_controlled_blocking_creature_spell.feature`
- `features/stack/target_opponents_blocking_creature_spell.feature`
- `features/stack/target_opponents_attacking_creature_spell.feature`
- `features/stack/reject_own_turn_priority_artifact_response.feature`
- `features/stack/reject_own_turn_priority_enchantment_response.feature`

### Turn flow

- `features/turn-flow/upkeep_priority_window.feature`
- `features/turn-flow/draw_priority_window.feature`
- `features/turn-flow/main_phase_priority_window.feature`
- `features/turn-flow/end_step_priority_window.feature`
- `features/turn-flow/mana_pool_clears_on_phase_advance.feature`
- `features/turn-flow/turn_progression.feature`
- `features/turn-flow/draw_multiple_cards.feature`
- `features/turn-flow/cleanup_damage_removal.feature`
- `features/turn-flow/cleanup_hand_size_discard.feature`
- `features/turn-flow/lose_on_empty_draw.feature`

### Combat

- `features/combat/combat_priority_windows.feature`
- `features/combat/combat_subphases_foundation.feature`
- `features/combat/beginning_of_combat_step.feature`
- `features/combat/declare_attackers_step.feature`
- `features/combat/declare_blockers_step.feature`
- `features/combat/combat_damage_step.feature`
- `features/combat/end_of_combat_step.feature`
- `features/combat/combat_step_priority_progression.feature`
- `features/combat/beginning_of_combat_priority_window.feature`
- `features/combat/post_combat_damage_priority_window.feature`
- `features/combat/combat_damage_marking.feature`
- `features/combat/single_blocker_per_attacker.feature`
- `features/combat/keyword_abilities.feature`
- `features/combat/creature_destruction.feature`

### Spells

- `features/spells/cast_creature_spell.feature`

### Life

- `features/life/adjust_player_life_effect.feature`
- `features/life/lose_on_zero_life.feature`

### State-based actions

- `features/state-based-actions/zero_toughness_creature_dies.feature`
- `features/state-based-actions/state_based_actions_review.feature`

### Zones

- `features/zones/exile_zone.feature`

Runner:

- `tests/bdd.rs`

## Implemented reference features not currently executed

These features remain useful as repository history or semantic reference, but are not currently wired into the executable BDD suite:

- `features/stack/cast_spell_goes_on_stack.feature`
- `features/stack/pass_priority_resolves_top.feature`

## Writing guidance

Prefer:

- observable behavior
- canonical gameplay actions
- current supported semantics
- repository terminology from the domain glossary

Avoid:

- implementation detail
- speculative mechanics
- hidden assumptions about unsupported timing or targeting
- literal rulebook transcription

## Relationship to slices and rules docs

Feature files are only one part of the repository’s truth model.

- `features/` describes behavior
- `docs/slices/` explains why a capability was added and what remains out of scope
- `docs/rules/` explains the repository-owned rules interpretation behind supported behavior
- `docs/domain/current-state.md` summarizes the live support set

For the rules side of the story, see [`docs/rules/README.md`](../docs/rules/README.md).
