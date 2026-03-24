# Proposal Slice — Make Player Arena Handle-First At The Core

## Summary

Shift the player-owned runtime so `PlayerCardHandle` becomes the canonical internal identity and `CardInstanceId` stops being the primary entry path into the arena.

## Motivation

- remove hashing and public-id churn from the core ownership path
- align the runtime with dense arena storage and embedded-class constraints
- prepare edge-only materialization of public card ids

## Target Shape

- internal zone and ownership operations enter through handles first
- public `CardInstanceId` remains available as a projection for commands, events, and tests
- player arena lookups no longer rely on public ids as the dominant runtime key

## Invariants

- live cards still expose stable outward-facing ids
- aggregate ownership remains explicit and deterministic
- this slice does not expand supported Magic rules
