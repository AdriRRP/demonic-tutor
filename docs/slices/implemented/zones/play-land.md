# Slice — PlayLand

## Goal

Allow the active player to play a land from hand under a minimal but meaningful legality model.

## Supported behavior

* accept a `PlayLandCommand`
* verify that the referenced player exists
* verify that the referenced player is the active player
* verify that the current phase allows playing a land
* verify that the referenced card exists in the player's hand
* verify that the referenced card is a land
* verify that the player has not already played a land this turn
* move the card from hand to battlefield
* increment the player's land-play counter
* emit `LandPlayed`

## Invariants currently enforced

* only the active player may play a land
* lands may only be played during `Phase::FirstMain` or `Phase::SecondMain`
* only cards in hand may be played as lands
* only cards with `CardType::Land` may be played as lands
* a player may play at most one land per turn in the current model

## Out of scope

* stack
* priority
* spell casting
* abilities
* combat
* effects that allow additional land plays
* card text execution

## Rules Reference

- 305.1
- 305.2
- 305.3

## Rules Support Statement

This slice implements land playing per rules 305.1, 305.2, and 305.3, with a simplified one-land-per-turn model.

## Notes

This slice introduces the first player-driven gameplay action with legality checks.

The current legality model is intentionally minimal and temporary.
