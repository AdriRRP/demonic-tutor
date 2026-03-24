# Proposal Slice — Commit Spells To Stack Through One Internal Object

## Summary

Collapse the remaining casting corridor into a single internal spell-commit object that already encapsulates the final stack insertion payload.

## Motivation

- reduce the conceptual split between validation, payment, extraction, and stack assembly
- make future payload and identity refactors easier to localize
- keep the casting corridor semantically explicit while shrinking branching and clones

## Target Shape

- the player/casting corridor returns one internal object ready for stack insertion
- `cast_spell` orchestrates timing/target legality and then inserts the prepared object directly
- stack object construction is no longer reassembled from separate prepared fragments

## Invariants

- spell timing and target legality remain unchanged
- failed casts still leave game state unchanged
- this slice does not expand supported Magic rules
