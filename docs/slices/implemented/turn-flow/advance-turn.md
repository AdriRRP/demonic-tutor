# Slice — AdvanceTurn

> **Note**: This is a historical baseline slice. Its original minimal turn model has been superseded by `full-turn-phases.md` and ADR 0013.

## Goal

Advance the game to the next player's turn using a minimal turn model.

## Supported behavior

* accept `AdvanceTurnCommand`
* change the active player
* progress turn and phase according to the current turn model
* reset land-play counters according to the slice's original minimal turn model
* emit `TurnProgressed`

## Invariants currently enforced

* turn progression emits a single composite fact
* the turn transition keeps the model compatible with phase-based legality checks

## Out of scope

* full turn structure
* automatic draw-step semantics
* upkeep
* full combat phase behavior
* priority
* stack
* automatic triggers
* multiplayer turn-order generalization

## Rules Reference

- 500–514

## Rules Support Statement

This slice implements the basic turn progression per the turn structure rules 500-514, using a minimal model. This implements basic player alternation and minimal turn progression. Full turn steps, priority, combat, and other detailed turn structure rules are not implemented.

## Notes

This slice records the repository's original minimal turn model and assumes exactly two players.

The original technical turn delta events have been superseded by the composite `TurnProgressed` event.
