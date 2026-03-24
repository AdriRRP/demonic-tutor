# Proposal Slice — Move Stack Runtime References Off Public Ids

## Summary

Replace public-id-based stack references such as controller and target links with internal indices and handles inside the stack runtime.

## Motivation

- remove public ids from one of the hottest runtime structures
- reduce clones and lookup churn during casting and resolution
- complete the shift toward an internal-reference-driven stack model

## Target Shape

- stack objects use internal runtime references for controller and target identity
- public ids are materialized only when building events, errors, or external projections
- stack ordering and target legality remain semantically identical

## Invariants

- stack ordering remains explicit and deterministic
- target legality still reflects truthful aggregate state
- this slice does not expand supported Magic rules
