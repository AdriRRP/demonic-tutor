# ADR 0007 — AdvanceTurn uses a minimal turn model

## Status

Accepted

## Context

After introducing `PlayLand`, the model required a way to reset turn-scoped state and make repeated land play possible across turns.

A full implementation of turn structure would introduce unnecessary complexity too early.

## Decision

For slice 4, the system introduces a minimal turn model based on:

* one active player
* one legal phase after turn advance: `Phase::Main`
* simple alternation between the two players
* reset of land-play tracking during turn transition

## Consequences

### Positive

* closes the most urgent consistency gap in the model
* keeps the design small
* prepares the system for future turn-aware actions

### Negative

* turn structure remains intentionally incomplete
* current logic still assumes exactly two players
* future slices may refine turn-scoped state handling

## Notes

This decision is intentionally incremental.
