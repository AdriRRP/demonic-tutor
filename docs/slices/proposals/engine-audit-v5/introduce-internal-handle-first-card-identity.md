# Proposal Slice — Introduce Internal Handle-First Card Identity

## Summary

Shift the core runtime toward handle-first card identity so player-owned operations stop depending on public `CardInstanceId` as their primary lookup path.

## Motivation

- reduce hashing and public-id churn in hot zone and ownership paths
- align the runtime with dense arena storage and embedded-class memory goals
- prepare future edge-only materialization of public card ids

## Target Shape

- internal card ownership and movement use player-local handles first
- public `CardInstanceId` remains available as an outward-facing projection
- the player arena no longer relies on public ids as its main runtime key

## Invariants

- live cards still have stable outward-facing ids
- aggregate ownership stays explicit and deterministic
- this slice does not expand supported Magic rules
