# Implemented Slice — Introduce Handle-First Runtime Identity

## Summary

Invert the internal ownership flow so player-owned runtime transitions operate from compact internal handles first and use `CardInstanceId` mainly as an edge-facing lookup.

## Supported Behavior

- internal player-owned removals and zone transitions now route through handle-first helpers
- zone membership checks resolve a handle once and keep the rest of the transition in handle space
- public card ids remain stable for commands, tests, and events

## Invariants

- aggregate ownership remains deterministic
- public `CardInstanceId` values remain stable at the boundaries
- this slice does not expand supported Magic rules

## Implementation Notes

- `Player` now centralizes handle resolution and handle-scoped movement helpers instead of repeating id-to-handle lookups across each transition
- the runtime keeps `CardInstanceId` as the public entrypoint while internal ownership corridors stay aligned with dense arena semantics

## Tests

- full casting, combat, targeting, draw, and BDD regression coverage remains green
