# Proposal Slice — Introduce Handle-First Runtime Identity

## Summary

Invert the current identity priority so the runtime core works primarily from compact internal handles and only projects public `CardInstanceId` when crossing outward-facing boundaries.

## Motivation

- remove repeated hashing and public-id dependency from hot card movement paths
- align the runtime with the dense arena already in place
- push the core closer to embedded-class memory and locality goals

## Target Shape

- player-owned card operations accept and propagate internal handles first
- public `CardInstanceId` becomes an edge-facing lookup or projection, not the main runtime key
- internal transitions stay semantically explicit through player-owned APIs

## Invariants

- public card ids remain stable for tests, commands, and events
- aggregate ownership stays deterministic
- this slice does not expand supported Magic rules
