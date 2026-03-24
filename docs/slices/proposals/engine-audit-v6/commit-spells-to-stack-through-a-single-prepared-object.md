# Proposal Slice — Commit Spells To Stack Through A Single Prepared Object

## Summary

Collapse the remaining casting corridor into one prepared internal object that already represents the final spell-to-stack commit.

## Motivation

- reduce conceptual split between preparation and final stack insertion
- make future payload and identity changes easier to localize
- shrink recomposition of ids, payload, and target in the casting hot path

## Target Shape

- the casting corridor returns one prepared internal stack-bound spell object
- `cast_spell` performs legality checks and then inserts that object directly
- stack-object construction is no longer rebuilt from separate fragments in the outer rule layer

## Invariants

- spell timing and target legality remain unchanged
- failed casts still leave state untouched
- this slice does not expand supported Magic rules
