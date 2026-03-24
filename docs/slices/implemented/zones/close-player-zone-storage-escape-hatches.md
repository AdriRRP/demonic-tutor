# Implemented Slice — Close Player Zone Storage Escape Hatches

## Summary

Remove the last live raw mutable zone-storage escape hatch from `Player` and keep player-owned zone mutation behind semantic domain operations.

## Supported Behavior

- gameplay rules continue to use semantic player operations for draw, recycle, receive, remove, and zone-to-zone movement
- `Player` no longer exposes the remaining raw mutable library storage entrypoint
- observable gameplay behavior remains unchanged

## Invariants

- player-owned zone transitions stay explicit and reviewable
- supported zone semantics remain unchanged
- this slice does not expand Magic rules support

## Implementation Notes

- the last live mutable storage escape hatch was unused by runtime and tests
- closing it keeps future storage refactors narrower and more honest to the aggregate API

## Tests

- existing full repository regression coverage remains green
