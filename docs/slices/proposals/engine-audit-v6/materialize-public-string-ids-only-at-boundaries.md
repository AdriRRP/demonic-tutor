# Proposal Slice — Materialize Public String Ids Only At Boundaries

## Summary

Finish the broader identity shift by making compact internal ids canonical in the core and materializing public string ids only when crossing API, event, or serialization boundaries.

## Motivation

- reduce pointer, allocation, and clone cost across the runtime
- align identity representation with embedded-class constraints
- complete the direction already proven with internal stack numbers and player-local handles

## Target Shape

- compact internal ids are canonical inside the aggregate
- outward-facing string ids are derived only when needed at boundaries
- tests and events can still assert stable public identifiers

## Invariants

- outward ids remain deterministic and reviewable
- internal identity stays explicit and deterministic
- this slice does not expand supported Magic rules
