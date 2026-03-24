# Proposal Slice — Thin Stack Payloads To Minimal In-Flight State

## Summary

Reduce `SpellPayload` so a spell in flight carries only the resolution-relevant state and not a generalized definition record.

## Motivation

- shrink per-object stack footprint
- remove repeated static metadata from the runtime hot path
- align the stack model with elite memory and locality goals

## Target Shape

- in-flight spell payloads are specialized by supported resolution family
- static definition metadata is no longer copied wholesale into each stack object
- the observable casting and resolution surface remains unchanged

## Invariants

- stack resolution stays deterministic and truthful to the current subset
- public events still expose stable identity and outcome data
- this slice does not expand supported Magic rules
