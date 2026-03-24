# Proposal Slice — Move Public String Ids To Edge Materialization

## Summary

Finish the broader shift toward compact internal identity by materializing string/public ids only at API, event, and serialization edges.

## Motivation

- reduce allocation and clone cost across the core runtime
- align ids with embedded-class constraints and denser in-memory structures
- complete the direction already proven for stack object numbers and arena handles

## Target Shape

- compact numeric/internal ids are canonical inside the aggregate
- public string ids are derived only when crossing outward-facing boundaries
- tests and events can still assert stable public identifiers

## Invariants

- outward ids remain deterministic and reviewable
- internal identity stays explicit and deterministic
- this slice does not expand supported Magic rules
