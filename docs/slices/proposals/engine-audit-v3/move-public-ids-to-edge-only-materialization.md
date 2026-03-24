# Proposal Slice — Move Public Ids To Edge-Only Materialization

## Summary

Adopt compact numeric internal ids across the engine and materialize string/public ids only at API, event, or serialization boundaries.

## Motivation

- reduce allocation, pointer, and clone cost for core identity paths
- align identity representation with embedded and high-throughput goals
- keep public ergonomics while shrinking the internal model

## Target Shape

- compact internal ids are canonical inside the aggregate
- public ids are derived views for outward-facing boundaries
- tests and events can still assert stable public identifiers where needed

## Invariants

- domain identity remains stable and explicit
- public-facing ids stay deterministic
- this slice does not expand supported Magic rules
