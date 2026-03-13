# ADR 0004 — StartGame supports exactly two players in slice 1

## Status
Accepted

## Context

The first vertical slice must remain narrow and easy to validate.

Supporting arbitrary player counts from the beginning would introduce unnecessary modeling and validation complexity before the core game lifecycle is stable.

## Decision

For slice 1, `StartGame` supports exactly two players.

This is a temporary product and modeling constraint for the first executable slice.

## Consequences

### Positive
- simpler validation
- simpler tests
- clearer domain expectations
- easier progression toward the next slice

### Negative
- the model is temporarily more restrictive than the long-term vision
- future slices may need to revisit this rule

## Notes

This decision is intentionally temporary and may be superseded later.
