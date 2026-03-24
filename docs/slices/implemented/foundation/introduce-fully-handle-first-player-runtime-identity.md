# Implemented Slice — Introduce Fully Handle-First Player Runtime Identity

## Summary

Finish the identity inversion inside `Player` so the runtime core treats `PlayerCardHandle` as canonical and uses public `CardInstanceId` only when crossing outward-facing boundaries.

## Motivation

- remove hashing and public-id dependence from player hot paths
- align the runtime with the dense arena already in place
- move the core closer to embedded-class locality and memory goals

## Delivered Shape

- internal player-owned lookups now resolve a handle once at the boundary and operate through handle-based arena access
- repeated internal `CardInstanceId -> handle -> arena` lookups were collapsed behind zone-aware helpers
- the public API still accepts stable outward ids where commands, events and tests expect them

## Invariants

- public card ids remain stable and deterministic
- aggregate ownership remains explicit
- this slice does not expand supported Magic rules
