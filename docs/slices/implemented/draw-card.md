# Slice — DrawCard

## Goal

Provide a simplified explicit draw effect entrypoint that lets the active player draw one or more cards from their library into their hand.

## Supported behavior

* accept `DrawCardsEffectCommand`
* verify that the referenced player exists
* verify that the referenced player is the active player
* verify that the current phase is an explicit action window
* verify that the requested draw count is at least one
* draw the requested number of cards one by one from library
* move each drawn card into the player's hand
* emit one `CardDrawn` per drawn card with explicit `DrawKind`
* if the library runs out mid-effect, emit `GameEnded` after any already completed draws

## Invariants currently enforced

* only the active player may draw through this command
* explicit draw effects are only allowed during `Phase::FirstMain` or `Phase::SecondMain`
* explicit draw effects must request at least one card
* if the library runs out during the effect, the game ends through the `LoseOnEmptyDraw` slice after completed draws are kept

## Out of scope

* replacing the automatic draw step
* replacement effects
* priority
* stack
* spell abilities

## Rules Reference

- 121.1
- 121.2

## Rules Support Statement

This slice implements a minimal explicit draw effect per rules 121.1 and 121.2. The current model also includes automatic turn-step draw, and this command remains as a simplified effect-oriented entrypoint distinct from the automatic draw flow. If the effect tries to draw past an empty library, the game ends through the separate `LoseOnEmptyDraw` slice after any already completed draws.

## Notes

This slice intentionally models a minimal explicit draw effect, not the full Magic draw step. The event distinguishes explicit effects from automatic turn-step draws, and multi-card effects are resolved one draw at a time.
