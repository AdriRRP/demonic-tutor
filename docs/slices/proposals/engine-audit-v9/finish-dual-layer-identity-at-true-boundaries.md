# Proposal Slice — Finish Dual-Layer Identity At True Boundaries

## Summary

Complete the dual-layer identity model so public string-backed ids are created only at true boundaries while the runtime core operates on compact internal identities.

## Motivation

- remove one of the last fixed memory taxes still present in the core
- finish the long-running shift from string-first identity to compact internal identity
- align the engine with ambitious embedded-class execution goals

## Target Shape

- internal numeric or handle-based identities are canonical inside the core
- string-backed ids are materialized only for API, events, serialization, and tests
- internal rules and aggregate helpers no longer require public string ids to do their work

## Invariants

- outward-facing ids remain deterministic and reviewable
- internal identity remains explicit and stable within the aggregate
- this slice does not expand supported Magic rules
