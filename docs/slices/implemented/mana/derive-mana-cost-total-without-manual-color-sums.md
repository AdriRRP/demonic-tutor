# Implemented Slice — Derive Mana Cost Total Without Manual Color Sums

## Summary

Remove the last manual color-by-color total in `ManaCost` so mana aggregates derive from the canonical indexed color model.

## Supported Behavior

- public `ManaCost` behavior remains unchanged
- total mana now derives from canonical color iteration instead of manual color summation
- mana helpers stay aligned with the current indexed representation

## Invariants

- total mana value remains unchanged for all supported costs
- no gameplay behavior changes
- this slice does not expand supported Magic rules

## Implementation Notes

- `ManaCost::total()` now derives from `ManaColor::ALL`
- the mana model now computes totals consistently across both `ManaPool` and `ManaCost`

## Tests

- full repository validation remains green after the mana helper cleanup
