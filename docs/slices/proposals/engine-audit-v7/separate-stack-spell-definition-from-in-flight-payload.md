# Proposal Slice — Separate Stack Spell Definition From In-Flight Payload

## Summary

Split canonical spell-definition metadata from the truly in-flight runtime payload so each stack object carries only the state needed while the spell is on the stack.

## Motivation

- shrink per-spell stack payload size further
- stop carrying mostly static definition data in every in-flight spell
- prepare a cleaner path toward definition handles or aggregate-owned definition lookup

## Target Shape

- stack spell payloads carry only minimal in-flight state
- canonical definition data is referenced or rebuilt from a cheaper boundary than a copied mini-definition record
- resolution still reconstructs truthful runtime cards when needed

## Invariants

- spell rules support remains unchanged
- outward spell ids remain stable
- this slice does not expand supported Magic rules
