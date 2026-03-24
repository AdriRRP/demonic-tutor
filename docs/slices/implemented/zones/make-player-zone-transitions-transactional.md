# Implemented Slice — Make Player Zone Transitions Transactional

## Summary

Refactor player-owned zone-to-zone movement so internal transitions apply as one transactional semantic change instead of a sequence of partial mutations.

## Motivation

- remove partial-failure windows from central movement helpers
- keep visible zones and primary location index synchronized by construction
- prepare safer ownership and location indexing at the aggregate level

## Delivered Shape

- `Player::move_handle_between_zones` now validates the arena handle first, removes from source, applies the destination move, and rolls the visible zone move back if the arena update fails
- source and destination zone mutations now flow through explicit helper methods inside `Player`
- regression coverage proves the card is restored to its original visible zone when the arena update path is desynchronized

## Invariants

- a live player-owned card still belongs to at most one player-owned zone
- visible zone behavior remains unchanged
- this slice does not expand supported Magic rules
