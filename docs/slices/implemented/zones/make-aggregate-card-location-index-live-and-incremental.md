# Implemented Slice — Make Aggregate Card Location Index Live And Incremental

## Summary

The aggregate card-location index no longer clears and rebuilds its whole map as the steady-state maintenance path.
It now keeps owner-scoped card sets and refreshes location entries per player.

## What Changed

- `AggregateCardLocationIndex` now tracks card ids by owner
- owner refresh removes and re-inserts only that owner's entries
- several hot aggregate entrypoints now refresh only the affected player locations where the supported behavior allows it

## Outcome

- the index stops behaving like a full regenerated snapshot on every update
- location maintenance is closer to localized runtime upkeep
- the current supported gameplay behavior remains unchanged
