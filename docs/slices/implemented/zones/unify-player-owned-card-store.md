# Slice Name

UnifyPlayerOwnedCardStore

---

## Goal

Replace the five per-zone `HashMap<CardInstanceId, CardInstance>` stores on `Player` with one player-owned card store shared by library, hand, battlefield, graveyard, and exile.

---

## Why This Slice Exists Now

The repository had already moved player zones to id-backed carriers, but `Player` still duplicated ownership across one store per zone.

This slice exists to:

1. collapse duplicated runtime ownership into one player-owned store
2. let zone-to-zone moves within one player move ids instead of whole cards
3. tighten ownership invariants before any future global registry work

---

## Supported Behavior

- `Player` owns cards through one internal `CardInstanceId -> CardInstance` store
- library, hand, battlefield, graveyard, and exile keep their current ordered zone semantics
- moving a card between player-owned zones no longer moves it between separate stores
- removing a card from a player-owned zone entirely still extracts the `CardInstance`

---

## Invariants / Legality Rules

- a card owned by a player exists in exactly one player-owned zone at a time
- zone ordering semantics stay unchanged
- this slice does not change gameplay legality or supported Magic rules

---

## Out of Scope

- multiplayer ownership
- control-changing effects across players
- a global card registry
- stack storage changes

---

## Domain Impact

### Aggregate Impact

- `Player` now owns one shared runtime card store behind semantic zone views

### Entity / Value Object Impact

- no new public entity or value object is introduced

### Commands

- no new public command required

### Events

- no event payload changes

### Errors

- no new public error required

---

## Documentation Impact

- this slice document

---

## Test Impact

- full unit and BDD regression coverage remains green
- zone transitions, draw, discard, exile, and casting continue to work through the shared store

---

## Rules Reference

- none beyond the current supported zone semantics

---

## Rules Support Statement

This slice is a runtime-storage refactor only. It does not expand Magic rules support.
