# ADR 0006 — Land play uses minimal turn legality

## Status
Accepted

## Context

The first player-driven gameplay action introduced in the system is playing a land.

If land play ignores all turn and phase constraints, the model would misrepresent gameplay legality and weaken the semantic meaning of the action.

However, implementing the full Magic turn structure at this stage would introduce significant complexity, including:

- complete phase sequencing
- priority handling
- stack interactions
- timing windows

This complexity is unnecessary before the basic gameplay lifecycle is stable.

A simplified legality model is therefore needed to preserve meaningful gameplay behavior while keeping the slice narrow.

## Decision

Land play is validated using a **minimal turn legality model**.

The model introduces the following concepts:

- a single active player
- a minimal playable phase (`Main`)
- a per-turn limit enforced through `lands_played_this_turn`

A player may play a land only if:

- they are the active player
- the current phase is `Main`
- they have not already played a land during the current turn

## Consequences

### Positive

- preserves meaningful gameplay legality
- keeps early slices simple and deterministic
- avoids premature implementation of full turn mechanics
- provides a clear foundation for future turn progression

### Negative

- the turn model remains intentionally incomplete
- future slices will need to expand the turn structure
- some real-game timing nuances are unsupported

## Notes

This decision introduces the first elements of the turn model but does not define the complete turn system.

Future slices will extend this model with additional phases and rules when required.
