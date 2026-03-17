# Gameplay Features

This directory contains Gherkin-style behavior specifications for DemonicTutor.

These files are not a literal copy of the Magic Comprehensive Rules.

They describe **repository-supported gameplay behavior** using the ubiquitous language of the `play` bounded context.

## Purpose

Features exist to make behavior:

- readable
- traceable to rules references
- mappable to slices
- easier to preserve across refactors

Some features may also be executable through `cucumber-rs`.

Current executable feature coverage:

- `features/stack/stack_foundation.feature`
- `features/stack/respond_with_instant_spell.feature`
- `features/stack/respond_in_upkeep_window.feature`
- `features/stack/respond_in_draw_window.feature`
- `features/stack/respond_in_first_main_window.feature`
- `features/stack/respond_in_second_main_window.feature`
- `features/stack/respond_in_end_step_window.feature`
- `features/stack/respond_in_beginning_of_combat_window.feature`
- `features/stack/respond_after_attackers.feature`
- `features/stack/respond_after_blockers.feature`
- `features/stack/respond_after_combat_damage.feature`
- `features/stack/cast_instant_in_upkeep_window.feature`
- `features/stack/cast_second_instant_in_upkeep_window.feature`
- `features/stack/cast_instant_in_draw_window.feature`
- `features/stack/cast_second_instant_in_draw_window.feature`
- `features/stack/cast_instant_in_second_main_window.feature`
- `features/stack/cast_second_instant_in_second_main_window.feature`
- `features/stack/cast_instant_in_end_step_window.feature`
- `features/stack/cast_instant_in_beginning_of_combat_window.feature`
- `features/stack/cast_instant_after_attackers.feature`
- `features/stack/cast_instant_after_blockers.feature`
- `features/stack/cast_instant_after_combat_damage.feature`
- `features/turn-flow/upkeep_priority_window.feature`
- `features/turn-flow/draw_priority_window.feature`
- `features/turn-flow/main_phase_priority_window.feature`
- `features/turn-flow/end_step_priority_window.feature`
- `features/turn-flow/turn_progression.feature`
- `features/turn-flow/draw_multiple_cards.feature`
- `features/spells/cast_creature_spell.feature`
- `features/combat/combat_priority_windows.feature`
- `features/combat/beginning_of_combat_priority_window.feature`
- `features/combat/post_combat_damage_priority_window.feature`
- `features/combat/combat_damage_marking.feature`
- `features/combat/single_blocker_per_attacker.feature`
- `features/combat/creature_destruction.feature`
- `features/turn-flow/cleanup_damage_removal.feature`
- `features/turn-flow/cleanup_hand_size_discard.feature`
- `features/turn-flow/lose_on_empty_draw.feature`
- `features/life/lose_on_zero_life.feature`
- `features/state-based-actions/zero_toughness_creature_dies.feature`
- `features/state-based-actions/state_based_actions_review.feature`
- runner: `tests/bdd.rs`

Implemented reference features that are not currently executed:

- `features/stack/cast_spell_goes_on_stack.feature`
- `features/stack/pass_priority_resolves_top.feature`

## Required Header Convention

Each feature should start with metadata comments containing:

- `status`
- `rules`
- `slices`

Example:

```gherkin
# status: implemented
# rules: 601.1, 601.2
# slices: cast-spell.md, pay-mana-cost.md
```

## Writing Rules

Prefer:

- observable behavior
- canonical gameplay actions
- current supported semantics

Avoid:

- implementation detail
- speculative mechanics
- hidden assumptions about stack or priority
- literal rulebook transcription

## Status Values

- `implemented`
- `proposed`
- `historical`

## Execution

Executable feature pilots live alongside normal Rust tests.

Current command:

```bash
cargo test --test bdd
```

Conventional non-BDD behavior tests are aggregated under:

```bash
cargo test --test unit
```
