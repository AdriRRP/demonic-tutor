# Slice Name

LoseOnZeroLife

---

## Goal

End the game immediately when a player's life total reaches 0.

---

## Why This Slice Exists Now

This slice follows `PlayerLife` and `LoseOnEmptyDraw` because:

1. the runtime already tracks life totals explicitly
2. the aggregate already owns terminal game state and `GameEnded`
3. zero-life loss is one of the most fundamental game-loss conditions
4. the behavior is narrow and valuable without requiring a full state-based action engine

---

## Supported Behavior

- end the game if `AdjustLifeCommand` reduces a player's life total to 0
- emit `LifeChanged`
- emit `GameEnded` with `GameEndReason::ZeroLife`
- record the terminal game state with winner, loser, and reason
- reject subsequent gameplay actions once the game has ended

---

## Invariants / Legality Rules

- life totals still saturate at 0
- zero-life loss is automatic game behavior, not a separate player command
- terminal game state is recorded exactly once for the current model
- once the game has ended, normal gameplay commands are no longer legal
- this slice assumes exactly two players when deriving the winner from the loser

---

## Out of Scope

- a general state-based action engine
- poison counters
- commander damage
- losing for attempting to draw from an empty library outside the existing slice
- alternate win conditions
- replacement or prevention effects that would modify life change handling

---

## Domain Impact

### Aggregate Impact

- extend `AdjustLife` so life changes can also produce terminal game state

### Entity / Value Object Impact

- no new entity required

### Commands

- no new public command required

### Events

- extend `GameEndReason` with `ZeroLife`
- reuse `GameEnded`

### Errors

- no new public error required

---

## Ownership Check

This behavior belongs to the `Game` aggregate because it:

- enforces a core game-loss rule inside the `play` domain
- derives winner and loser from authoritative aggregate state
- affects legality of all later gameplay actions
- emits the resulting domain facts for projections and replay

---

## Documentation Impact

- `docs/domain/current-state.md`
- `docs/domain/aggregate-game.md`
- `docs/domain/DOMAIN_GLOSSARY.md`
- `docs/rules/rules-map.md`
- `docs/slices/implemented/player-life.md`
- `features/life/lose_on_zero_life.feature`
- this slice document

---

## Test Impact

- reducing a player to 0 life ends the game
- `AdjustLife` still emits `LifeChanged`
- `GameEnded` carries `ZeroLife`
- later gameplay actions fail once the game has ended
- BDD covers the zero-life loss scenario

---

## Rules Reference

- 104.3b — A player loses the game if their life total is 0 or less
- 704.5a — A player with 0 or less life loses the game

---

## Rules Support Statement

This slice implements a narrow game-loss condition for zero life. The repository still does not model a full state-based action engine; instead, this condition is applied immediately after the explicit life change supported by the current runtime.
