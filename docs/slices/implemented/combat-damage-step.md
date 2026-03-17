# Slice — Combat Damage Step

## Goal

Make `CombatDamage` an explicit progression step in the turn model, even before richer combat-damage timing is added.

## Supported Behavior

- `CombatDamage` is a real combat subphase in the aggregate
- if no combat-damage action is taken, turn progression may move from `CombatDamage` into `EndOfCombat`
- the active player remains the active player across that step transition

## Explicit Limits

- this slice only formalizes empty-step progression
- combat-damage resolution semantics live in the combat-damage slice, not here
- no first strike, double strike, or combat-damage triggers are modeled

## Domain Changes

- no new public command is introduced
- turn progression now treats `CombatDamage` as a first-class step instead of an implicit internal moment

## Rules Support Statement

This slice keeps the combat timeline explicit through the damage handoff. `CombatDamage` is now a step the turn model can stand in and advance through, even in simplified empty-step situations.

## Tests

- BDD coverage confirms that an empty `CombatDamage` step advances to `EndOfCombat`
