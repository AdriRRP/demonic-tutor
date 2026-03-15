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
* emit `CardDrawn`

## Invariants currently enforced

* only the active player may draw through this command
* drawing is only allowed during `Phase::Main`
* drawing fails if the library has no available cards

## Out of scope

* automatic draw step
* drawing multiple cards
* decking / losing from empty library
* replacement effects
* priority
* stack
* spell abilities

## Notes

This slice intentionally models a minimal explicit draw action, not the full Magic draw step.
