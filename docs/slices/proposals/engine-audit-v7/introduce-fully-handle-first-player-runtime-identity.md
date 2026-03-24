# Proposal Slice — Introduce Fully Handle-First Player Runtime Identity

## Summary

Finish the identity inversion inside `Player` so the runtime core treats `PlayerCardHandle` as canonical and uses public `CardInstanceId` only when crossing outward-facing boundaries.

## Motivation

- remove hashing and public-id dependence from player hot paths
- align the runtime with the dense arena already in place
- move the core closer to embedded-class locality and memory goals

## Target Shape

- internal player-owned operations accept and propagate handles first
- outward card ids are resolved only at command, event, or test-facing edges
- the player runtime keeps one explicit internal ownership identity

## Invariants

- public card ids remain stable and deterministic
- aggregate ownership remains explicit
- this slice does not expand supported Magic rules
