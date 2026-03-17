# Slice Name

CleanupHandSizeDiscard

---

## Goal

Require the active player to discard down to the maximum hand size before the turn can advance out of `EndStep`.

---

## Why This Slice Exists Now

This slice is the next coherent cleanup increment because:

1. end-of-turn damage cleanup is already modeled
2. the runtime already exposes an `EndStep` transition point
3. Magic cleanup semantics require a hand-size discard choice before the turn fully ends
4. automatic arbitrary discard would be semantically misleading

---

## Supported Behavior

- block `advance_turn` from `EndStep` while the active player's hand size is above the maximum
- accept `DiscardCardCommand` for the active player in `EndStep`
- move the chosen card from hand to graveyard
- allow the turn to advance once the hand size is at or below the maximum
- emit `CardDiscarded`

---

## Invariants / Legality Rules

- only the active player may discard for cleanup
- cleanup discard is only legal during `EndStep`
- cleanup discard is only legal while the active player has more than the maximum hand size
- `advance_turn` from `EndStep` fails while cleanup discard is still required
- discarding for cleanup preserves zone consistency by moving the card from hand to graveyard

---

## Out of Scope

- a distinct `Cleanup` phase in the phase model
- repeated cleanup loops caused by state-based actions or triggered abilities
- discard effects from spells or abilities outside cleanup
- player loss from empty library
- priority during cleanup

---

## Domain Impact

### Aggregate Impact

- `Game` now enforces a cleanup hand-size gate before leaving `EndStep`

### Commands

- add `DiscardCardCommand`

### Events

- add `CardDiscarded`

### Errors

- add explicit errors for invalid discard timing and for pending cleanup discard requirements

---

## Ownership Check

This behavior belongs to the `Game` aggregate because it:

- enforces end-of-turn legality
- owns zone movement between hand and graveyard
- decides whether the turn is allowed to advance

---

## Documentation Impact

- `docs/domain/current-state.md`
- `docs/domain/aggregate-game.md`
- `docs/domain/DOMAIN_GLOSSARY.md`
- `docs/rules/notes/turn-flow.md`
- `docs/rules/rules-map.md`
- `features/turn-flow/cleanup_hand_size_discard.feature`
- this slice document

---

## Test Impact

- discarding during cleanup moves the chosen card from hand to graveyard
- discarding fails outside `EndStep`
- discarding fails when it is not required
- the turn cannot advance from `EndStep` while hand-size cleanup is still pending
- the turn advances normally once the player has discarded down to the maximum

---

## Rules Reference

- 514.1
- 514.1a

---

## Rules Support Statement

This slice introduces a minimal hand-size cleanup behavior. The active player explicitly discards chosen cards during `EndStep` until their hand is at or below the maximum, after which the turn may advance. It does not model a distinct cleanup step or repeated cleanup loops.
