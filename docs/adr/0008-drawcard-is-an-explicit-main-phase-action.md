# ADR 0008 — DrawCard is modeled as an explicit main-phase action in slice 5

## Status

Accepted

## Context

The system needs a minimal card-draw behavior to evolve gameplay state and test interactions between library, hand and later actions.

Introducing a full turn structure with automatic draw step would add complexity too early.

## Decision

For slice 5, card draw is modeled as an explicit command-driven action that:

* targets the active player
* requires `Phase::Main`
* draws exactly one card

## Consequences

### Positive

* keeps the slice narrow
* reuses existing structures
* avoids premature turn-step complexity

### Negative

* does not model the real automatic draw step yet
* may need revision when turn phases expand

## Notes

This is an intentionally temporary simplification.
