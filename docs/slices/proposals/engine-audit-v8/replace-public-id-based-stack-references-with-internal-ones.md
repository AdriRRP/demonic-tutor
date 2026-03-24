# Proposal Slice — Replace Public-Id-Based Stack References With Internal Ones

## Summary

Move `StackObject`, `SpellTarget`, and related in-flight references away from public `PlayerId` and `CardInstanceId` toward internal indices and handles.

## Motivation

- remove more public-id traffic from the hottest runtime structures
- reduce cloning and lookup overhead during stack casting and resolution
- align stack representation with the rest of the handle-first runtime

## Target Shape

- stack objects reference controller and targets through internal runtime references
- public ids are materialized only when building events, errors, or test-facing outputs
- stack operations stay semantically identical at the observable surface

## Invariants

- stack ordering and ownership remain explicit and deterministic
- target legality and resolution still use truthful aggregate state
- this slice does not expand supported Magic rules
