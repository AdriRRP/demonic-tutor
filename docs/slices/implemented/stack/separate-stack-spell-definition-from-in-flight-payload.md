# Implemented Slice — Separate Stack Spell Definition From In-Flight Payload

## Summary

Split canonical spell-definition metadata from the truly in-flight runtime payload so each stack object carries only the state needed while the spell is on the stack.

## Motivation

- shrink per-spell stack payload size further
- stop carrying mostly static definition data in every in-flight spell
- prepare a cleaner path toward definition handles or aggregate-owned definition lookup

## Delivered Shape

- stack spell payloads no longer carry `casting_permission` or `activated_mana_ability`
- in-flight spell definition data keeps only the metadata needed to resolve and reconstruct truthful runtime cards for the currently supported subset
- reconstruction now derives default definition behavior from `CardType` and overlays the supported spell rules, mana cost and supported non-mana activated ability when present

## Invariants

- spell rules support remains unchanged
- outward spell ids remain stable
- this slice does not expand supported Magic rules
