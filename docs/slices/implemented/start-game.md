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

## Notes

This slice intentionally prioritizes correctness and narrow scope over completeness.
