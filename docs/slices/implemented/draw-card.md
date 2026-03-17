# Slice — DrawCard

## Goal

Allow the active player to draw exactly one card from their library into their hand.

## Supported behavior

* accept `DrawCardCommand`
* verify that the referenced player exists
* verify that the referenced player is the active player
* verify that the current phase allows drawing
* draw exactly one card from library
* move that card into the player's hand
* emit `CardDrawn` with explicit `DrawKind`

## Invariants currently enforced

* only the active player may draw through this command
* drawing is only allowed during `Phase::Untap`, `Phase::Draw`, `Phase::FirstMain`, or `Phase::SecondMain`
* drawing fails if the library has no available cards

## Out of scope

* replacing the automatic draw step
* drawing multiple cards
* decking / losing from empty library
* replacement effects
* priority
* stack
* spell abilities

## Rules Reference

- 121.1
- 121.2

## Rules Support Statement

This slice implements a minimal explicit draw action per rules 121.1 and 121.2. The current model also includes automatic turn-step draw, and this command remains as an explicit draw entrypoint distinct from the automatic draw flow.

## Notes

This slice intentionally models a minimal explicit draw action, not the full Magic draw step. The event now distinguishes explicit draws from automatic turn-step draws.
