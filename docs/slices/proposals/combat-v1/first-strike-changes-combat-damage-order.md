# Slice Proposal — First Strike Changes Combat Damage Order

## Goal

Support `First Strike` so creatures with that keyword deal combat damage in an earlier combat-damage step.

## Why This Slice Exists Now

The current combat model has a single combat-damage step. `First Strike` is the first keyword that forces a richer damage-order model and therefore marks an important stability milestone.

## Supported Behavior

- creatures with `First Strike` deal combat damage before creatures without it
- creatures destroyed by first-strike damage do not deal normal combat damage later that combat

## Invariants / Legality Rules

- the model remains explicit about the supported first-strike subset
- state-based actions still review lethal damage between the supported damage steps

## Out of Scope

- double strike
- multiple blockers
- deathtouch interactions unless directly required

## Domain Impact

- extend the closed keyword set with `First Strike`
- refine combat-damage progression into an earlier and later supported pass

## Ownership Check

Combat-step progression and damage legality remain aggregate-owned.

## Documentation Impact

- current-state
- glossary
- implemented slice doc

## Test Impact

- unit and BDD coverage for reordered damage and skipped retaliation

## Rules Reference

- 510
- 702.7
- 704

## Rules Support Statement

This slice adds a minimal first-strike combat-damage model only. It does not yet imply double strike or broader multi-step combat complexity.
