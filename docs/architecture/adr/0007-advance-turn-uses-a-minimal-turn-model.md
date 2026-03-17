# ADR 0007 — Turn progression uses a minimal turn model

## Status
Superseded

## Context

Once player-driven actions such as `PlayLand` exist, the model requires a mechanism to:

- progress the game between turns
- reset turn-scoped state
- allow repeated actions across turns

Without a turn transition mechanism, actions limited to once per turn (such as land play) cannot be meaningfully repeated.

At the same time, implementing the full Magic turn structure at this stage would introduce unnecessary complexity, including:

- complete phase sequencing
- priority handling
- stack interactions
- complex timing rules

A minimal turn progression model is therefore needed to support early gameplay behavior while keeping the system simple.

## Decision

Turn progression is modeled using a **minimal turn system**.

The model introduces the following behavior:

- one active player
- simple alternation between the two players
- a minimal playable phase after a turn advance (`Phase::Main`)
- reset of turn-scoped state (such as `lands_played_this_turn`) when a turn changes

This model provides a deterministic mechanism for repeating turn-limited actions.

## Consequences

### Positive

- enables repeated turn-scoped actions
- keeps the gameplay model simple
- avoids premature implementation of full turn mechanics
- provides a foundation for future turn and phase modeling

### Negative

- the turn system remains intentionally incomplete
- the model still assumes exactly two players
- future slices will expand the turn structure and state tracking

## Notes

This decision introduces the minimal infrastructure required for turn progression.

Future slices will refine the turn model by introducing additional phases and rules when required.

This ADR has been superseded by the later full phase model and composite turn progression decisions.
