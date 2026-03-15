# ADR 0008 — Card draw is modeled as an explicit player action

## Status
Accepted

## Context

The gameplay model requires a mechanism for moving cards from the library to the hand.

Card draw is necessary to:

- evolve gameplay state
- enable future card interactions
- validate zone transitions between library and hand

In the full Magic rules, drawing a card normally occurs automatically during the draw step of a turn.

However, implementing the full turn structure at this stage would introduce additional complexity, including:

- multiple turn steps
- automatic phase-driven effects
- additional timing rules

To keep the gameplay model small and deterministic, card draw must initially be modeled in a simpler way.

## Decision

Card draw is modeled as an **explicit command-driven action**.

The action has the following constraints:

- the active player performs the draw
- the action is valid only during `Phase::Main`
- exactly one card is drawn from the player's library

This provides a minimal mechanism for evolving the player's hand while maintaining domain legality.

## Consequences

### Positive

- enables gameplay progression through card acquisition
- reuses existing zone and player structures
- keeps the slice small and deterministic
- avoids premature implementation of full turn-step mechanics

### Negative

- does not model the automatic draw step of real gameplay
- future slices will likely introduce a proper draw phase
- command-driven draw may later become a derived or automatic behavior

## Notes

This decision is an intentional simplification of the draw mechanism.

Future slices may introduce additional phases or automatic behaviors that replace this explicit action.
