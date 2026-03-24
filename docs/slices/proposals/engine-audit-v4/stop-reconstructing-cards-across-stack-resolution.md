# Proposal Slice — Stop Reconstructing Cards Across Stack Resolution

## Summary

Reduce stack-resolution churn by avoiding full card reconstruction when a spell leaves the stack and re-enters player-owned storage or a destination zone.

## Motivation

- cut unnecessary payload rebuilding on every spell resolution
- align stack and player storage around a cheaper carrier/lifetime model
- prepare the engine for tighter memory targets without changing outward gameplay semantics

## Target Shape

- stack resolution consumes a thinner spell carrier than a generalized mini-card
- when a resolving spell must return to player-owned storage, the corridor reuses or transfers a carrier instead of rebuilding a fresh `CardInstance`
- instant and sorcery destinations remain semantically correct

## Invariants

- observable spell outcomes remain unchanged
- supported permanent spells still land with the same runtime semantics they currently expose
- this slice does not expand supported Magic rules
