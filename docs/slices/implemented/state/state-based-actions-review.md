# Slice Name

StateBasedActionsReview

---

## Goal

Review the currently supported state-based actions in a single shared step after relevant gameplay actions.

---

## Why This Slice Exists Now

Several automatic consequences were already implemented, but they were still checked from isolated points in different rules modules.

This slice exists to:

1. keep supported state-based actions semantically centralized
2. avoid re-encoding the same automatic consequences in multiple action handlers
3. make future SBA growth less fragile before introducing stack or priority
4. preserve the current minimal scope without building a generic rules engine

---

## Supported Behavior

- relevant gameplay actions now trigger a shared review of currently supported state-based actions
- the shared review currently covers:
  - players at `0` life
  - creatures with `0` toughness
  - creatures with lethal marked damage
- the review may emit `CreatureDied` and `GameEnded` in addition to the initiating action's own events
- the review remains internal to the aggregate and is not modeled as a player command

---

## Invariants / Legality Rules

- state-based actions are still modeled only for the repository's currently supported rule subset
- the shared review is deterministic and aggregate-owned
- the review does not imply a full priority system or a complete Magic SBA engine
- terminal game state still blocks future gameplay actions as before

---

## Out of Scope

- a complete state-based action system
- repeated SBA loops caused by continuous effects, triggers, or replacement effects
- stack-based timing
- simultaneous multi-player elimination edge cases beyond the current two-player model

---

## Domain Impact

### Aggregate Impact

- the `Game` aggregate now routes relevant actions through a shared internal SBA review step

### Entity / Value Object Impact

- no new entity type is required

### Commands

- no new public command required

### Events

- no new event type required
- existing actions may now publish existing automatic-consequence events through the shared review

### Errors

- no new public error required

---

## Documentation Impact

- `docs/domain/current-state.md`
- `docs/rules/rules-map.md`
- `docs/rules/notes/combat.md`
- `features/state-based-actions/state_based_actions_review.feature`
- this slice document

---

## Test Impact

- relevant actions still emit their primary events
- existing zero-toughness and lethal-damage creature deaths still occur
- pending supported SBAs are reviewed after another relevant action completes
- terminal game-end behavior still works through the shared review

---

## Rules Reference

- 704 — State-based actions
- 704.5a — A player with 0 life loses the game
- 704.5f — A creature with toughness 0 or less is put into its owner's graveyard
- 704.5g — A creature with lethal damage is destroyed

---

## Rules Support Statement

This slice does not introduce a general state-based action engine. It centralizes the currently supported SBA subset into a shared aggregate-owned review step that runs after relevant gameplay actions.
