# Slice — Exile Zone

## Goal

Introduce the exile zone as a first-class player-owned zone in the `Game` aggregate.

## Status

`implemented`

---

## Why This Slice

This slice follows the existing zone model foundation because:

1. the aggregate already owns `Library`, `Hand`, `Battlefield`, and `Graveyard` zones
2. exile is a fundamental Magic zone referenced by CR 406
3. many future gameplay mechanics depend on exile (exile-to-play effects, exile-linked abilities, suspend, blink)
4. the behavior is observable, narrow, and deterministic
5. no triggered abilities or complex replacement effects are required yet

---

## Supported Behavior

- each player owns an `Exile` zone
- cards can be moved to exile from:
  - the stack (spells or abilities that exile as part of their resolution)
  - the battlefield (creatures exiled by effects)
  - the graveyard (cards exiled from graveyard)
- exiled cards can be examined by any player (CR 406.3)
- exiled cards remain face up by default
- the aggregate preserves insertion order within each player's exile zone
- exile is intentionally modeled as an ordered zone, unlike battlefield storage
- no "return from exile" behavior is introduced yet
- no exile-linked abilities are modeled yet

---

## Invariants / Legality Rules

- cards in exile belong to exactly one player
- zone changes maintain consistency (a card cannot be in two zones simultaneously)
- exile preserves insertion order to preserve card history
- battlefield order remains intentionally unstable in the current runtime and is not reused as a precedent for exile
- the aggregate exposes cards in exile for gameplay inspection
- exile does not yet support face-down cards

---

## Out of Scope

- return from exile to battlefield or other zones
- exile-linked abilities (CR 406.6)
- face-down exile (CR 406.3a, 406.4)
- exile pile organization (CR 406.5)
- triggered abilities that fire when cards are exiled
- suspend or other mechanics that depend on exile timing
- exile caused by replacement effects
- Auras or Equipment being attached from exile

---

## Domain Impact

### Aggregate Impact

- extend `Player` with `exile: Exile`
- extend `Game` with `exile_card` behavior that moves a card to exile
- expose `exile()` accessor on `Player`

### Entity / Value Object Impact

- add `Exile` struct in `zones.rs` following the existing `Graveyard` pattern

### Commands

- add `ExileCardCommand`
- the current minimal runtime exposes exile as a direct public effect entrypoint for battlefield/graveyard moves
- future spell or ability slices should reuse that zone-transition path rather than duplicating exile semantics

### Events

- add visible exile-zone movement events
```rust
pub struct CardMovedZone {
    pub game_id: GameId,
    pub player_id: PlayerId,
    pub card_id: CardInstanceId,
    pub origin_zone: ZoneType,
}
```

### Errors

- no new public error type required beyond existing zone/card legality failures

---

## Ownership Check

This behavior belongs to the `Game` aggregate because:

- exile is a zone that players own within the authoritative game state
- zone transitions must maintain consistency across the aggregate
- card movements from exile affect future game state and legality

---

## Documentation Impact

- `docs/domain/current-state.md`
- `docs/domain/aggregate-game.md`
- `docs/domain/DOMAIN_GLOSSARY.md`
- `docs/rules/rules-map.md`
- `docs/rules/notes/exile.md`
- `features/zones/exile_zone.feature`
- this slice document

---

## Test Impact

- a card can be moved to exile from the battlefield
- a card can be moved to exile from the graveyard
- exiled cards are visible through the player's exile zone
- cards cannot be in both exile and another zone simultaneously
- a visible zone-move event to `Exile` is emitted when exile occurs

---

## Rules Reference

- 406 — Exile
- 406.1 — The exile zone is essentially a holding area for objects
- 406.2 — To exile an object is to put it into the exile zone from whatever zone it is currently in
- 406.3 — Exiled cards are kept face up and may be examined by any player

---

## Implementation Notes

### Zone Model

The `Exile` zone should follow the same pattern as `Graveyard`, preserving insertion order:

```rust
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Exile(Vec<CardInstance>);

impl Exile {
    pub fn add(&mut self, card: CardInstance) { ... }
    pub fn cards(&self) -> &[CardInstance] { ... }
    pub fn remove(&mut self, card_id: &CardInstanceId) -> Option<CardInstance> { ... }
}
```

### Player Extension

```rust
pub struct Player {
    // ... existing fields ...
    exile: Exile,
}

impl Player {
    pub fn exile_mut(&mut self) -> &mut Exile { ... }
    pub const fn exile(&self) -> &Exile { ... }
}
```

### Aggregate Helper

```rust
impl Game {
    pub(crate) fn exile_card(
        &mut self,
        player_id: &PlayerId,
        card: CardInstance,
        origin_zone: ZoneType,
    ) -> CardExiled { ... }
}
```

---

## Rules Support Statement

This slice introduces the exile zone as a foundational zone model and also exposes a minimal public exile action for moving cards from battlefield or graveyard into exile. Exile-linked abilities, face-down exile, and return-from-exile behavior remain future work.
