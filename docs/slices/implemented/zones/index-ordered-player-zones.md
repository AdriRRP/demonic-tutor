# Slice Name

IndexOrderedPlayerZones

---

## Goal

Add explicit membership/index support to ordered player-owned zones so common `contains` and `remove` operations stop depending on repeated linear scans.

---

## Why This Slice Exists Now

After moving hand, battlefield, graveyard, and exile to id-backed carriers, the next honest performance step was to make ordered zones answer membership and removal directly.

This slice exists to:

1. give ordered zones explicit index-aware membership
2. stop repeating scan-plus-store checks in `Player`
3. keep current ordering semantics while reducing repeated lookup cost

---

## Supported Behavior

- hand, battlefield, graveyard, and exile keep their visible order semantics
- those zones now maintain explicit `CardInstanceId -> index` membership support
- `Player` trusts zone membership directly instead of combining the same check with extra scans
- battlefield still preserves its current `swap_remove` semantics for unordered removal

---

## Invariants / Legality Rules

- zone ordering semantics remain unchanged
- zone membership stays unique per player-owned zone
- gameplay behavior does not change

---

## Out of Scope

- global card indexing
- stack indexing
- cross-player control changes

---

## Domain Impact

### Aggregate Impact

- ordered zone carriers now own explicit membership/index responsibility

### Entity / Value Object Impact

- no new public entity type is introduced

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
- draw, discard, zone movement, and combat removal continue to behave the same

---

## Rules Reference

- none beyond the current supported zone semantics

---

## Rules Support Statement

This slice optimizes ordered zone lookup and removal only. It does not expand Magic rules support.
