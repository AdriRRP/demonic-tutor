# ADR 0005 — Opening hand size is fixed to seven in slice 2

## Status

Accepted

## Context

The second vertical slice needs a clear and deterministic opening hand behavior without introducing broader rules complexity such as mulligans or configurable formats.

## Decision

For slice 2, the opening hand size is fixed to 7 cards.

## Consequences

### Positive

* simpler implementation
* simpler tests
* clearer expectations
* easier progression toward the next slice

### Negative

* not yet adaptable to future formats or special rules
* will need revisiting when mulligan or alternative formats are introduced

## Notes

This decision is intentionally temporary.
