# Implemented Slice — Make Player Card Location A Primary Runtime Index

## Summary

Promote player-owned card location to primary runtime data so ownership and zone queries read one maintained index instead of probing every zone container.

## Supported Behavior

- each live player-owned card now stores its current player-owned zone alongside the arena entry
- semantic transitions keep the zone container and primary location index synchronized
- `card_zone()` and ownership-style queries read the arena location directly

## Invariants

- a live card still belongs to at most one player-owned zone at a time
- visible zone behavior remains unchanged
- this slice does not expand supported Magic rules

## Implementation Notes

- `PlayerCardArena` now stores `PlayerOwnedCard`, coupling each live card with its current `PlayerCardZone`
- draw, recycle, and zone-to-zone transitions update the primary location as part of the same semantic movement

## Tests

- full casting, draw, combat, targeting, and BDD regression coverage remains green
