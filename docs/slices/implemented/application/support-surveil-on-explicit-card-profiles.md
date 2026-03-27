# Slice Implemented - Support Surveil On Explicit Card Profiles

## Outcome

The engine now supports bounded `surveil 1` through an explicit pending library-choice decision during stack resolution.

## What Landed

- one supported `surveil 1` spell profile
- a pending surveil state in the `Game` aggregate with `ResolvePendingSurveilCommand`
- reuse of the current mill corridor so the looked-at card can move from library to graveyard deterministically
- public legal actions and choice requests that surface the currently inspected top card to the controlling player of the pending choice
- deterministic follow-up behavior:
  - `keep on top`
  - `move to graveyard`

## Notes

- this slice intentionally stops at `surveil 1`; it does not introduce generic arbitrary graveyard-or-library partitioning
- the public choice request is controller-scoped by `player_id` and carries only the currently inspected top card for that pending surveil decision
