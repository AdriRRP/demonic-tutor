# Slice — DrawOpeningHands

## Goal

Assign opening hands to all players in a started game.

## Supported behavior

* accept a `DealOpeningHandsCommand`
* receive initial card definitions for each player
* build card instances
* initialize player libraries
* draw exactly 7 cards per player
* move those cards from library to hand
* emit one `OpeningHandDealt` event per player
* apply the operation atomically

## Invariants currently enforced

* all referenced players must exist in the game
* all players must have enough cards to receive an opening hand
* no partial state changes are allowed on failure
* opening hand size is exactly 7 for this slice

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
