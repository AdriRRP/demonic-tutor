# Slice — DrawCard

## Goal

Provide a simplified explicit draw effect entrypoint that lets the active player draw exactly one card from their library into their hand.

## Supported behavior

* accept `DrawCardEffectCommand`
* verify that the referenced player exists
* verify that the referenced player is the active player
* verify that the current phase is an explicit action window
* draw exactly one card from library
* move that card into the player's hand
* emit `CardDrawn` with explicit `DrawKind`

## Invariants currently enforced

* only the active player may draw through this command
* explicit draw effects are only allowed during `Phase::FirstMain` or `Phase::SecondMain`
* if the library is empty, the explicit draw effect ends the game through the `LoseOnEmptyDraw` slice

## Out of scope

* replacing the automatic draw step
* drawing multiple cards
* replacement effects
* priority
* stack
* spell abilities

## Rules Reference

- 121.1
- 121.2

## Rules Support Statement

This slice implements a minimal explicit draw effect per rules 121.1 and 121.2. The current model also includes automatic turn-step draw, and this command remains as a simplified effect-oriented entrypoint distinct from the automatic draw flow. If the draw cannot happen because the library is empty, the game ends through the separate `LoseOnEmptyDraw` slice.

## Notes

This slice intentionally models a minimal explicit draw effect, not the full Magic draw step. The event now distinguishes explicit effects from automatic turn-step draws.
