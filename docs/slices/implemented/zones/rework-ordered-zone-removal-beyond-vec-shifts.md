# Implemented Slice — Rework Ordered Zone Removal Beyond Vec Shifts

## Summary

Replace the `Vec::remove` plus suffix reindexing strategy in ordered player zones with a slot-backed linked ordered structure that preserves visible order while avoiding full suffix rewrites on each removal.

## Supported Behavior

- hand, graveyard, and exile still preserve visible insertion order
- preserved-order removals no longer shift the whole visible suffix
- ordered iteration and positional reads remain truthful for gameplay code and tests

## Invariants

- battlefield remains intentionally separate with its swap-remove semantics
- membership and visible order stay explicit
- this slice does not expand supported Magic rules

## Implementation Notes

- ordered player zones now store handles in reusable slots linked by `prev` and `next`
- removals update only neighboring links and the handle-to-slot index
- `handle_at` and ordered iteration traverse the visible sequence instead of depending on `Vec::remove`

## Tests

- full repository validation remains green
- dedicated ordered-zone unit coverage verifies preserved order and slot reuse after removals
