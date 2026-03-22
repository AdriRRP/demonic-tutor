# Slice — Declare Blockers Step

## Goal

Make `DeclareBlockers` an explicit progression step in the turn model, even before richer blocker-declaration semantics are added.

## Supported Behavior

- `DeclareBlockers` is a real combat subphase in the aggregate
- if no blocker-declaration action is taken, turn progression may move from `DeclareBlockers` into `CombatDamage`
- the active player remains the active player across that step transition

## Explicit Limits

- this slice only formalizes empty-step progression
- blocker declaration legality and post-declaration priority are owned by the combat slices, not by this step-formalization slice
- no triggered abilities tied to declare blockers are modeled

## Domain Changes

- no new public command is introduced
- turn progression now treats `DeclareBlockers` as a first-class step instead of an implicit internal moment

## Rules Support Statement

This slice keeps combat timing explicit through the next handoff point. `DeclareBlockers` is now a step the turn model can stand in and advance through even when no blocking action is taken.

## Tests

- BDD coverage confirms that an empty `DeclareBlockers` step advances to `CombatDamage`
