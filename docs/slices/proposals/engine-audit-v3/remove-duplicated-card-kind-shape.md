# Proposal Slice — Remove Duplicated Card Kind Shape

## Summary

Model card kind once by type shape instead of carrying correlated `card_type` and runtime-kind information in parallel.

## Motivation

- reduce duplicated static information in `CardInstance` and `SpellCardSnapshot`
- make invalid or drifting internal combinations harder to represent
- shrink runtime and snapshot payloads

## Target Shape

- face/runtime/snapshot variants carry their own supported payload
- creature-specific data lives only in creature-shaped variants
- non-creature paths avoid creature-oriented baggage

## Invariants

- supported card kinds remain truthful to the current subset
- creature-specific operations stay impossible on non-creature variants by construction
- this slice does not expand supported Magic rules
