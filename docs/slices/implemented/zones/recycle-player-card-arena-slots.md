# Implemented Slice — Recycle Player Card Arena Slots

## Summary

Teach the dense player card arena to reuse released slots instead of growing monotonically after every remove-and-reinsert cycle.

## Supported Behavior

- player-local handles remain the internal zone carrier
- inserting a card now reuses a released arena slot before extending the backing vector
- outward card lookup and zone semantics remain unchanged

## Invariants

- live cards still map to exactly one player-local handle
- removed cards stop being addressable through the arena
- this slice does not expand supported Magic rules

## Implementation Notes

- `PlayerCardArena` now tracks a local free-slot stack
- `remove_by_handle` releases the slot for future reuse
- `insert` prefers recycled slots before growing the arena backing vector

## Tests

- full repository validation remains green after the arena reuse refactor
