# Implemented Slice — Make Player Arena Handle-First At The Core

## Summary

The player-owned arena now treats `PlayerCardHandle` as its canonical runtime identity.
`CardInstanceId` remains available at the player boundary for commands, events, and tests, but it no longer lives inside the arena storage itself.

## What Changed

- `PlayerCardArena` now stores only dense handle-indexed card slots plus free-slot reuse.
- `Player` owns the outward-facing `CardInstanceId -> PlayerCardHandle` projection.
- handle-based removals and receives now update that outward projection at the player boundary.

## Outcome

- arena storage is more aligned with compact runtime ownership
- public card ids are pushed one layer closer to the true boundary
- supported gameplay behavior remains unchanged
