# Proposal Slice — Materialize Public String Ids Only At True Boundaries

## Summary

Push `GameId`, `PlayerId`, `CardInstanceId`, and related string-backed ids out of the core so they are created only when crossing API, event, serialization, or test-facing boundaries.

## Motivation

- remove one of the last major fixed memory taxes from the runtime core
- align identity representation with embedded-class constraints
- complete the long-running shift toward compact canonical internal identity

## Target Shape

- compact internal ids are canonical inside the core
- outward-facing string ids are derived only when needed
- events and tests still observe stable deterministic public identifiers

## Invariants

- public ids remain reviewable and deterministic
- internal identity remains explicit and deterministic
- this slice does not expand supported Magic rules
