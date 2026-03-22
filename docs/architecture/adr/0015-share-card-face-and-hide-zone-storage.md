# ADR 0015 — Share immutable card face and hide zone storage

## Status
Accepted

## Context

As the runtime gained richer casting, targeting, mana, and combat behavior, `CardInstance` risked carrying too much duplicated immutable metadata across library, hand, battlefield, graveyard, exile, and stack.

At the same time, direct dependence on raw zone storage would make later performance-oriented refactors expensive, because rules and tests would depend on `Vec` and `VecDeque` details instead of gameplay semantics.

The repository needed a stable runtime direction that improves memory behavior now without forcing an immediate full registry-by-id redesign.

## Decision

The runtime shares immutable card-face metadata across card instances and consumes zones through semantic queries and transitions rather than through raw storage internals.

For the current model this means:

- `CardInstance` shares immutable card definition data instead of copying full definitions by value
- stack objects may carry explicit spell metadata needed by resolution instead of rediscovering it from the moved card value
- `Player` and zone types expose semantic reads and transitions that shield rules and tests from storage details
- player-owned zones may migrate to id-backed storage behind those same semantic APIs before hotter carriers such as stack do
- future performance refactors should prefer changing storage behind those semantic APIs rather than widening direct mutable access again

## Consequences

### Positive

- lower duplication of immutable card metadata across zones
- clearer separation between immutable card face and mutable runtime state
- easier future migration toward cheaper zone carriers or id-based storage
- stronger protection against storage-shaped logic leaking into rules and tests
- safer phased storage refactors because low-interaction zones can prove the pattern before hotter runtime carriers

### Negative

- transitional APIs on `Player` can grow while the storage migration is still incomplete
- the runtime still stores full `CardInstance` values on the stack for now
- a later registry-by-id design will still require a deliberate follow-up refactor

## Notes

This ADR records a runtime direction, not a claim that the repository already uses a full central card registry.
