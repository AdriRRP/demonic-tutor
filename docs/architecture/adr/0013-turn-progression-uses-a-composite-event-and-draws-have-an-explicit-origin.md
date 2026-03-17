# ADR 0013 — Turn progression uses a composite event and draws have an explicit origin

## Status
Accepted

## Context

The play context had modeled a single `advance_turn` operation with three separate events:

- `TurnAdvanced`
- `TurnNumberChanged`
- `PhaseChanged`

In practice those three events were always emitted together from one domain operation, which made replay and projection logic reconstruct a single semantic fact from multiple technical pieces.

The context also emitted `CardDrawn` both for the automatic draw step and for the explicit draw-effect command, but without indicating the origin of the draw.

## Decision

Turn progression is represented as a single composite event:

- `TurnProgressed`

This event carries:

- active player after progression
- previous and new turn number
- previous and new phase

`CardDrawn` remains a distinct event, but now includes `DrawKind` so projections and replay models can distinguish:

- `TurnStep`
- `ExplicitEffect`

## Consequences

### Positive

- turn progression is represented as one semantic fact
- projections no longer need to correlate three inseparable events
- draw events are more expressive without multiplying event types
- the event catalog is better aligned with DDD language than with technical state deltas

### Negative

- consumers of the old turn events must be updated together
- projections lose the smaller-granularity technical event stream unless it is reintroduced later

## Notes

This decision does not require splitting `events.rs` into multiple modules yet. It only refines the semantics of the event catalog.
