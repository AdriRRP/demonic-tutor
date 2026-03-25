# Slice Implemented - Support Scry On Explicit Card Profiles

## Outcome

The engine now supports bounded `scry 1` through an explicit pending library-choice decision during stack resolution.

## What Landed

- one supported `scry 1` spell profile
- a pending scry state in the `Game` aggregate with `ResolvePendingScryCommand`
- library operations to inspect the current top card and optionally move it to the bottom
- public legal actions and choice requests that surface the looked-at top card to the controlling player of the pending choice
- deterministic follow-up behavior:
  - `keep on top`
  - `move to bottom`

## Notes

- this slice intentionally stops at `scry 1`; it does not introduce generic arbitrary library reordering
- the public choice request is controller-scoped by `player_id` and carries only the currently inspected top card for that pending scry decision
