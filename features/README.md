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

- `features/turn-flow/turn_progression.feature`
- `features/spells/cast_creature_spell.feature`
- `features/combat/combat_damage_marking.feature`
- `features/combat/creature_destruction.feature`
- `features/turn-flow/cleanup_damage_removal.feature`
- `features/turn-flow/cleanup_hand_size_discard.feature`
- `features/turn-flow/lose_on_empty_draw.feature`
- `features/life/lose_on_zero_life.feature`
- `features/state-based-actions/zero_toughness_creature_dies.feature`
- `features/state-based-actions/state_based_actions_review.feature`
- runner: `tests/bdd.rs`

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
