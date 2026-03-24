# Proposal Slice — Thin Stack Spell Payloads Beyond Shared Definitions

## Summary

Reduce the in-flight spell carrier again so stack objects stop transporting full shared card-definition references when a cheaper runtime payload would do.

## Motivation

- cut per-object stack footprint further
- reduce static metadata duplicated across many stack objects
- separate in-flight spell semantics more cleanly from owned card storage

## Target Shape

- stack spell payloads carry only the runtime data needed for resolution and destination routing
- immutable spell-definition facts are reachable through a cheaper internal path than `Arc<CardDefinition>` per object
- supported spell families remain explicit and reviewable

## Invariants

- spell outcomes and outward events remain unchanged
- supported permanent spells still resolve to the same destinations
- this slice does not expand supported Magic rules
