# Proposal Slice — Thin Stack Spell Payloads

## Summary

Replace generalized spell snapshots on stack objects with smaller spell payloads that only carry resolution-relevant data.

## Motivation

- reduce stack object size
- avoid carrying generalized mini-card state when most supported spells only need definition-driven data plus a tiny payload
- prepare the stack for cheaper embedded execution

## Target Shape

- stack stores a compact spell payload per supported spell family
- immutable definition data remains shared
- resolution reconstructs only what the destination corridor actually needs

## Invariants

- stack semantics stay unchanged
- supported spell destinations and effects remain truthful to the current model
- this slice does not expand supported Magic rules
