# Slice Name

DrawMultipleCards

---

## Goal

Extend explicit draw effects so they can draw more than one card through a single canonical command onto a chosen target player.

---

## Why This Slice Exists Now

The project already supports:

- automatic turn-step draw
- explicit one-card draw effects
- game loss when a player must draw from an empty library

Allowing an explicit effect to draw multiple cards onto a chosen player is a small but useful extension that reuses those semantics without introducing stack or priority.

---

## Supported Behavior

- replace the single-card explicit draw command with `DrawCardsEffectCommand`
- let the active player choose the target player for the draw effect
- require an explicit draw count of at least one
- draw cards one by one
- emit one `CardDrawn` event per completed draw
- if the effect tries to draw from the target player's empty library mid-resolution, emit `GameEnded(EmptyLibraryDraw)` after completed draws remain applied

---

## Invariants / Legality Rules

- only the active player may use the explicit draw-effect command
- explicit draw effects are only allowed during `FirstMain` and `SecondMain`
- draw count must be at least one
- completed draws are not rolled back if the effect later ends the game on the target player's empty library

---

## Out of Scope

- automatic draw-step replacement
- replacement effects
- priority
- stack
- spell or ability objects that grant the draw effect

---

## Domain Impact

### Commands

- replace `DrawCardEffectCommand` with `DrawCardsEffectCommand`

### Aggregate / Rules

- explicit draw effects now return a batch of `CardDrawn` events plus optional `GameEnded`

### Events

- no new event types
- reuse `CardDrawn` and `GameEnded`

---

## Documentation Impact

- `docs/slices/implemented/draw-card.md`
- `docs/slices/implemented/lose-on-empty-draw.md`
- `docs/domain/current-state.md`
- `docs/rules/notes/turn-flow.md`

---

## Test Impact

- unit coverage for drawing multiple cards
- unit coverage for zero draw count rejection
- BDD coverage for successful multi-draw and empty-library loss mid-effect

---

## Rules Reference

- 121.1
- 121.2
- 121.4

---

## Rules Support Statement

DemonicTutor supports explicit multi-card draw effects as a simplified non-stack action window. The active player chooses the target player, the effect resolves one draw at a time on that target, and it reuses the existing empty-library loss semantics when that target library is overrun.
