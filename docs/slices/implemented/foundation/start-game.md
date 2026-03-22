# Slice — StartGame

## Goal

Initialize a valid game with exactly two players.

## Supported behavior

- accept a `StartGameCommand`
- validate player count
- reject duplicate players
- create a `Game`
- emit `GameStarted`
- expose the created aggregate and event through the application service

## Invariants currently enforced

- a game must start with exactly two players
- duplicate players are not allowed

## Out of scope

- deck validation
- card instances
- zones with cards
- hand drawing
- turn progression
- phase progression
- priority
- stack
- analytics projections
- event persistence
- event publication

## Rules Reference

- 103.1
- 103.2

## Rules Support Statement

This slice initializes the game state as defined in rule 103.1 and establishes the two-player structure per rule 103.2. It establishes the two-player game start but does not implement the full pre-game procedure (ante, sideboards, etc.).

## Notes

This slice intentionally prioritizes correctness and narrow scope over completeness.
