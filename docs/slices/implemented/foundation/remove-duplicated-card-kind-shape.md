# Implemented Slice — Remove Duplicated Card Kind Shape

## Summary

Make `CardDefinition` the single canonical owner of static card type, while runtime and spell snapshots only carry the dynamic data they actually need.

## Supported Behavior

- `CardInstance` derives its `CardType` from shared immutable definition data
- `SpellCardSnapshot` no longer duplicates static card type state
- creature-specific runtime data remains expressed through creature-shaped runtime variants

## Invariants

- static card kind remains truthful to the current supported subset
- creature-only runtime data stays impossible on non-creature runtime variants
- this slice does not expand supported Magic rules

## Implementation Notes

- `CardDefinition` now stores canonical `CardType`
- `CardFace` only keeps shared immutable definition data
- `SpellCardSnapshot` only keeps definition plus optional creature payload

## Tests

- full repository validation remains green after the shape simplification
