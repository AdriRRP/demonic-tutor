# Proposal Slice — Thin Stack Spell Payloads Further

## Summary

Reduce stack-borne spell payload size again by separating truly resolution-relevant data from heavyweight shared definition references.

## Motivation

- shrink per-object stack footprint
- reduce repeated static metadata carried by every spell on the stack
- prepare a cleaner separation between runtime card ownership and in-flight spell semantics

## Target Shape

- stack spell payloads carry only the data needed for resolution and destination routing
- immutable card-definition facts are referenced through a cheaper internal path
- supported permanent and non-permanent spell families remain explicit

## Invariants

- spell resolution behavior remains unchanged
- outward spell events keep the same observable semantics
- this slice does not expand supported Magic rules
