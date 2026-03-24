# Implemented Slice — Model Card Runtime With Valid State Shapes

## Summary

Refactor `CardInstance` runtime so the currently supported runtime shapes cannot represent impossible combinations between card kind and creature-only state.

## Supported Behavior

- the supported card subset behaves exactly as before
- creature-only runtime state now exists only behind the creature runtime variant
- non-creature cards no longer carry an optional creature runtime slot

## Invariants

- non-creature cards cannot carry creature-only runtime by construction
- creature runtime still owns stats, damage, combat flags, blocking target, temporary pump, and keywords
- gameplay legality remains unchanged

## Implementation Notes

- this slice keeps the public `CardInstance` API stable while tightening the internal runtime model
- the refactor reduces internal defensive matching against impossible states and prepares future permanent semantics on a more honest shape

## Tests

- full combat, targeting, casting, and zone regression coverage remains green
