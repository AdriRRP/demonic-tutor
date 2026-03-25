# Slice Name

Multiple Blockers Per Attacker

## Goal

Allow one attacking creature to be blocked by more than one blocker.

## Why This Slice Exists Now

The current one-blocker simplification is one of the biggest remaining combat gaps. Lifting it unlocks many real board states and is a strong step toward a usable combat engine.

## Supported Behavior

- allow several blockers to be declared against one attacker
- preserve existing blocker legality checks for each blocker
- represent the attacking creature as blocked by an ordered set of blockers

## Invariants / Legality Rules

- each blocker still blocks at most one attacker in the supported subset unless another slice says otherwise
- blocking legality still respects current flying and reach constraints
- declaring blockers remains impossible outside the supported step and priority state

## Out of Scope

- one blocker blocking multiple attackers
- banding
- menace, skulk, or “must be blocked by two or more” rules

## Domain Impact

### Aggregate Impact

- widen combat assignment storage from one-blocker-per-attacker to ordered blocker groups

## Ownership Check

This belongs to the `Game` aggregate because combat assignment is aggregate-owned legality state.

## Documentation Impact

- `docs/domain/current-state.md`
- `docs/domain/aggregate-game.md`
- this slice doc

## Test Impact

- two blockers can block one attacker
- flying/reach legality still applies per blocker
- existing one-blocker scenarios still work unchanged

## Rules Reference

- 509

## Rules Support Statement

This slice removes the current one-blocker simplification. It does not yet imply full blocking exceptions or unusual combat keywords.

