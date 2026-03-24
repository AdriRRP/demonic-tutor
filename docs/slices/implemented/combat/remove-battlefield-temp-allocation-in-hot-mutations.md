# Implemented Slice — Remove Battlefield Temp Allocation In Hot Mutations

## Summary

Replace battlefield-wide mutation that collected a temporary handle buffer with direct indexed handle access over the stable battlefield order.

## Supported Behavior

- untap, combat cleanup, and end-step cleanup still mutate every battlefield permanent as before
- the hot path no longer allocates a temporary `Vec` of handles for these passes
- battlefield storage remains encapsulated behind the existing player-owned semantic API

## Invariants

- callers still do not gain raw mutable access to battlefield internals
- battlefield mutation still respects aggregate ownership
- this slice does not expand supported Magic rules

## Implementation Notes

- `Player::for_each_battlefield_card_mut` now walks stable battlefield indices directly
- each visible handle is resolved lazily and applied immediately to the arena-backed card store
- the battlefield visit path no longer performs a per-pass temporary allocation

## Tests

- full repository validation remains green after the battlefield hot-path cleanup
