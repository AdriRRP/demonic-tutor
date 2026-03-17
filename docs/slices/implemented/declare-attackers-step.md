# Slice — Declare Attackers Step

## Goal

Make `DeclareAttackers` an explicit progression step in the turn model, even before any richer attacker-declaration semantics are added.

## Supported Behavior

- `DeclareAttackers` is a real combat subphase in the aggregate
- if no attacker-declaration action is taken, turn progression may move from `DeclareAttackers` into `DeclareBlockers`
- the active player remains the active player across that step transition

## Explicit Limits

- this slice only formalizes empty-step progression
- attacker declaration legality and post-declaration priority are owned by the combat slices, not by this step-formalization slice
- no triggered abilities tied to beginning of declare-attackers are modeled

## Domain Changes

- no new public command is introduced
- turn progression now treats `DeclareAttackers` as a first-class step instead of an implicit internal moment

## Rules Support Statement

This slice makes the combat timeline easier to reason about. `DeclareAttackers` is no longer just “the thing that happens after beginning of combat”; it is an explicit step the turn model can stand in and advance through.

## Tests

- BDD coverage confirms that an empty `DeclareAttackers` step advances to `DeclareBlockers`
