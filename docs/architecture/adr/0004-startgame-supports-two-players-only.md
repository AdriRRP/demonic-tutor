# ADR 0004 — Early gameplay supports exactly two players

## Status
Accepted

## Context

The first playable slices of the system must remain narrow and easy to validate.

Modeling a variable number of players from the beginning would introduce additional complexity in areas such as:

- turn order
- phase progression
- player management
- command validation

Before the gameplay lifecycle is stable, this additional complexity would increase modeling effort without improving the core domain behavior.

The early system therefore benefits from a simpler and more predictable player structure.

## Decision

In the early gameplay model, a `Game` supports exactly two players.

The `StartGame` command requires two player identities and initializes the game with those participants.

This constraint simplifies the initial gameplay slices and reduces the scope of early modeling.

## Consequences

### Positive

- simpler validation logic
- easier test scenarios
- clearer expectations for early slices
- reduced modeling complexity

### Negative

- the model is temporarily more restrictive than the long-term design
- future slices may need to relax this constraint
- player management logic may require refactoring when multiplayer support is introduced

## Notes

This decision is an intentional simplification for early milestones.

Future slices may extend the model to support additional players if required by the gameplay scope.
