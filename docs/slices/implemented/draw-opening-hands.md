# Slice — DrawOpeningHands

## Goal

Assign opening hands to all players in a started game.

## Supported behavior

* accept a `DealOpeningHandsCommand`
* receive `PlayerLibrary` data for each player
* translate `LibraryCard` values into runtime `CardInstance`s
* initialize player libraries
* draw exactly 7 cards per player
* move those cards from library to hand
* emit one `OpeningHandDealt` event per player
* apply the operation atomically
* represent creature and non-creature library inputs as distinct variants

## Invariants currently enforced

* all referenced players must exist in the game
* all players must have enough cards to receive an opening hand
* no partial state changes are allowed on failure
* opening hand size is exactly 7 for this slice
* creature `LibraryCard` values always carry explicit power and toughness by type

## Out of scope

* mulligan
* shuffle configuration
* real deck validation
* phase-aware card draw
* priority
* stack
* card text execution
* analytics projections
* persistence
* event publication

## Rules Reference

- 103.3

## Rules Support Statement

This slice implements the initial hand creation per rule 103.3, drawing exactly seven cards. Opening hand size is fixed to seven cards as a development simplification.

## Notes

This slice is intentionally limited.
Opening hand size is currently hardcoded to 7 as a temporary rule for early development.
The `Play` context now receives play-owned library initialization data rather than raw deck-context terminology.
Creature and non-creature library initialization are distinct variants in the input model.
