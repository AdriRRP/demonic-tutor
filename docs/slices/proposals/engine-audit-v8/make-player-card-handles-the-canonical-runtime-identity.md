# Proposal Slice — Make Player Card Handles The Canonical Runtime Identity

## Summary

Push `PlayerCardHandle` to become the canonical internal identity for player-owned cards so the runtime stops entering the arena primarily through public `CardInstanceId`.

## Motivation

- remove hashing and public-id churn from core ownership paths
- align the runtime with dense arena storage and handle-first lookup
- continue the path toward edge-only materialization of public card ids

## Target Shape

- internal zone and ownership operations use handles as their primary identity
- public `CardInstanceId` remains available as an outward-facing projection
- player runtime helpers no longer depend on public ids as their main entry path

## Invariants

- live cards still expose stable outward-facing ids
- aggregate ownership remains explicit and deterministic
- this slice does not expand supported Magic rules
