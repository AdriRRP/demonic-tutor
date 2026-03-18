# Slice — DrawCard

## Goal

Provide a simplified explicit draw effect entrypoint that lets the active player make a chosen player draw one or more cards from that player's library into that player's hand.

## Supported behavior

* accept `DrawCardsEffectCommand`
* verify that the caster exists
* verify that the target player exists
* verify that the caster is the active player
* verify that the current phase is an explicit action window
* verify that the requested draw count is at least one
* draw the requested number of cards one by one from the target player's library
* move each drawn card into the target player's hand
* emit one `CardDrawn` per drawn card with explicit `DrawKind`
* if the target library runs out mid-effect, emit `GameEnded` after any already completed draws

## Invariants currently enforced

* only the active player may cast this command
* explicit draw effects are only allowed during `Phase::FirstMain` or `Phase::SecondMain`
* explicit draw effects must request at least one card
* if the target library runs out during the effect, the target player loses through the `LoseOnEmptyDraw` slice after completed draws are kept

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

This slice implements a minimal explicit draw effect per rules 121.1 and 121.2. The current model also includes automatic turn-step draw, and this command remains as a simplified effect-oriented entrypoint distinct from the automatic draw flow. The active player currently chooses the target player directly in the command. If the effect tries to draw past that target player's empty library, the game ends through the separate `LoseOnEmptyDraw` slice after any already completed draws.

## Notes

This slice intentionally models a minimal explicit draw effect, not the full Magic draw step. The event distinguishes explicit effects from automatic turn-step draws, and multi-card effects are resolved one draw at a time on the chosen target player.
