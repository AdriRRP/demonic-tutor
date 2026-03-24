# Implemented Slice — Derive Mana Pool Queries From Color Iteration

## Summary

Make `ManaPool` derive aggregate queries from the closed supported color set instead of hardcoding each supported color manually.

## Supported Behavior

- mana totals remain unchanged
- current color-specific queries remain available
- observable payment behavior remains unchanged

## Invariants

- the closed supported color set remains explicit
- gameplay legality remains unchanged
- this slice does not expand Magic rules support

## Implementation Notes

- `ManaPool::total()` now derives its colored contribution from `ManaColor::ALL`
- this reduces drift risk as the mana model evolves while keeping the current API stable

## Tests

- full mana, casting, and regression coverage remains green
