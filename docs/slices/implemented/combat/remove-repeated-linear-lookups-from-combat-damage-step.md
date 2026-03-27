# Slice: Remove Repeated Linear Lookups From Combat Damage Step

Status: implemented

## Summary

Combat damage resolution now uses temporary keyed lookups for damage accumulation and blocker lookup instead of repeated linear scans over already materialized participants.

## What changed

- accumulated creature damage is merged through a temporary target index
- blocker reconstruction for an attacker now uses a keyed lookup table over blocker refs
- combat semantics remain unchanged

## Why it matters

- reduces avoidable hot-path cost in combat damage
- keeps the current explicit combat algorithm while improving locality
- prepares the combat corridor for broader limited-set density
