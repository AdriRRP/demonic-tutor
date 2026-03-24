# Proposal Slice — Thin Stack Spell Payloads Beyond Definition Records

## Summary

Reduce `SpellPayload` further so a spell in flight carries only the minimal resolution-relevant state instead of a generalized definition record.

## Motivation

- shrink per-object stack footprint
- reduce repeated static metadata carried by each spell in flight
- move the stack closer to an embedded-grade carrier model

## Target Shape

- stack spell payloads are specialized by the supported resolution families
- canonical card-definition metadata is no longer copied wholesale into every spell payload
- resolution still has enough information to produce the same observable outcomes and events

## Invariants

- stack resolution remains deterministic and truthful to the current rules subset
- public events still expose stable card identity and outcome data
- this slice does not expand supported Magic rules
