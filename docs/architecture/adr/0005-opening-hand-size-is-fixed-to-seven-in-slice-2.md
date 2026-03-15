# ADR 0005 — Opening hand size is fixed to seven cards

## Status
Accepted

## Context

The early gameplay model requires a deterministic and simple game initialization process.

Supporting configurable hand sizes or format-dependent rules at this stage would introduce additional complexity in areas such as:

- deck validation
- format configuration
- mulligan rules
- setup-phase orchestration

Before the core gameplay lifecycle is fully stable, this additional flexibility would increase modeling complexity without improving the core domain behavior.

A fixed opening hand size simplifies early gameplay slices and allows the system to evolve incrementally.

## Decision

The opening hand size is fixed to **seven cards** when a game begins.

During game setup, each player draws seven cards from their library to form their initial hand.

This rule defines the initial behavior of the opening hand in the gameplay model.

## Consequences

### Positive

- deterministic game initialization
- simpler implementation
- simpler test scenarios
- clearer expectations for early gameplay slices

### Negative

- the rule is temporarily rigid
- support for alternative formats may require revisiting this decision
- mulligan or format rules may introduce additional behavior around the opening hand

## Notes

Later slices introduce mulligan behavior that allows players to redraw their hand during the setup phase.

The opening hand size itself remains seven cards in the current model.
