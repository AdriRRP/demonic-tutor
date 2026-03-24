# Proposal Slice — Derive Mana Cost Total Without Manual Color Sums

## Summary

Remove the last manual color-by-color total in `ManaCost` so mana aggregates derive from the canonical indexed color model.

## Motivation

- reduce drift potential in the mana API
- keep `ManaCost` and `ManaPool` aligned around the same representation choice
- tighten a small but visible Rust-idiomatic inconsistency

## Target Shape

- `ManaCost::total()` derives from canonical color iteration or the underlying storage directly
- public `ManaCost` behavior stays unchanged
- mana helpers remain compact and reviewable

## Invariants

- total mana value remains exactly the same for all currently supported costs
- no gameplay behavior changes
- this slice does not expand supported Magic rules
