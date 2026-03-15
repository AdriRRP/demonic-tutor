# Slice — AdvanceTurn

## Goal

Advance the game to the next player's turn using a minimal turn model.

## Supported behavior

* accept `AdvanceTurnCommand`
* change the active player
* reset the phase to `Phase::Main`
* reset land-play counters according to the current simplified model
* emit `TurnAdvanced`

## Invariants currently enforced

* the active player changes when the turn advances
* the game returns to `Phase::Main`
* the turn transition keeps the model compatible with `PlayLand`

## Out of scope

* full turn structure
* draw step
* upkeep
* combat
* priority
* stack
* automatic triggers
* multiplayer turn-order generalization

## Rules Reference

- 500–514

## Rules Support Statement

This slice implements the basic turn progression per the turn structure rules 500-514, using a minimal model. This implements basic player alternation and minimal turn progression. Full turn steps, priority, combat, and other detailed turn structure rules are not implemented.

## Notes

This slice uses a minimal turn model and currently assumes exactly two players.
