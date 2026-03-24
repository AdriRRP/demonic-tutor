# Implemented Slice — Unify Indexed Ordered Zone Storage

## Summary

Extract a shared internal representation for the ordered id-backed player zones that preserve insertion order.

## Supported Behavior

- `Hand`, `Graveyard`, and `Exile` keep their current ordering semantics
- membership and positional lookup remain unchanged
- order-preserving removals remain unchanged

## Invariants

- observable zone order remains truthful to the current model
- gameplay legality remains unchanged
- this slice does not expand Magic rules support

## Implementation Notes

- the repository now uses one shared internal storage shape for the ordered indexed-zone pattern
- `Battlefield` stays separate because its current removal semantics intentionally use a different strategy

## Tests

- full draw, discard, exile, graveyard-targeting, casting, and BDD regression coverage remains green
