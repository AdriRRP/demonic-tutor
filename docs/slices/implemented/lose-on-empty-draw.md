# Slice Name

LoseOnEmptyDraw

---

## Goal

End the game immediately when a player is required to draw a card but their library is empty.

---

## Why This Slice Exists Now

This slice follows explicit draw effects and automatic draw-step support because:

1. the runtime already models libraries, hand movement, and draw legality
2. draw behavior is now central to turn progression and effect resolution
3. empty-library draw is a fundamental game-loss condition with clear observability
4. the behavior is narrow and valuable without requiring stack or priority support

---

## Supported Behavior

- end the game if the active player reaches the draw step and cannot draw from an empty library
- end the game if an explicit draw effect requires a draw from an empty library
- record the terminal game state with winner, loser, and `GameEndReason::EmptyLibraryDraw`
- emit `GameEnded`
- reject subsequent gameplay actions once the game has ended

---

## Invariants / Legality Rules

- a required draw from an empty library never silently succeeds
- empty-library draw loss is automatic game behavior, not a separate player command
- terminal game state is recorded exactly once for the current model
- once the game has ended, normal gameplay commands are no longer legal
- this slice assumes exactly two players when deriving the winner from the loser

---

## Out of Scope

- replacement effects that change how draws work
- multiplayer winner derivation
- conceding
- poison, life-based loss, or other loss conditions
- draws that are prevented or replaced
- restart effects

---

## Domain Impact

### Aggregate Impact

- extend `Game` with terminal game state (`winner`, `loser`, `end_reason`)
- prevent normal gameplay actions after the game has ended

### Entity / Value Object Impact

- no new entity required

### Commands

- no new public command required

### Events

- add `GameEnded`
- add `GameEndReason::EmptyLibraryDraw`

### Errors

- add `GameAlreadyEnded`

---

## Ownership Check

This behavior belongs to the `Game` aggregate because it:

- enforces a core game-loss rule inside the `play` domain
- derives the terminal outcome from authoritative aggregate state
- affects legality of subsequent gameplay actions
- emits the resulting domain fact for projections and replay

---

## Documentation Impact

- `docs/domain/current-state.md`
- `docs/domain/aggregate-game.md`
- `docs/domain/DOMAIN_GLOSSARY.md`
- `docs/rules/notes/turn-flow.md`
- `docs/rules/rules-map.md`
- `docs/slices/implemented/draw-card.md`
- `features/turn-flow/lose_on_empty_draw.feature`
- this slice document

---

## Test Impact

- game ends when the active player cannot perform the automatic draw step
- game ends when an explicit draw effect tries to draw from an empty library
- terminal winner, loser, and reason are recorded correctly
- later gameplay actions fail once the game has ended
- projections log `GameEnded`

---

## Rules Reference

- 121.4 — If a player attempts to draw from an empty library, that player loses the game
- 704.5b — A player with an empty library loses if required to draw

---

## Rules Support Statement

This slice implements a narrow game-loss condition for required draws from an empty library. It currently applies to the automatic draw step and to the simplified explicit draw-effect entrypoint. It does not yet implement the broader set of game-loss conditions or replacement effects that can modify draw behavior.
