# Implemented Slice — Materialize Public String Ids Only At Boundaries

## Summary

Push another internal corridor away from public string ids by using player indexes as the canonical runtime locator for zone transitions and materializing `PlayerId` only when emitting outward-facing errors or events.

## Supported Behavior

- battlefield and graveyard exile effects now surface the same public `CardMovedZone(origin -> Exile)` events at the application boundary
- id-based rule helpers remain available at the outer API surface
- internal resolution paths can now stay in index space longer before crossing outward boundaries

## Invariants

- outward ids remain deterministic and reviewable
- internal ownership stays explicit and deterministic
- this slice does not expand supported Magic rules

## Implementation Notes

- zone rules now expose index-based exile helpers for internal callers
- stack-resolution effects use `player_index` directly and only materialize `PlayerId` when building the resulting event or error

## Tests

- full exile, targeting, casting, combat, and BDD regression coverage remains green
