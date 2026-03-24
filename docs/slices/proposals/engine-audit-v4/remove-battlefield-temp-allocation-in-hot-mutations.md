# Proposal Slice — Remove Battlefield Temp Allocation In Hot Mutations

## Summary

Replace the current battlefield-wide mutable visit pattern with one that does not allocate a temporary handle buffer on every hot turn-flow and combat cleanup pass.

## Motivation

- reduce repeated per-phase allocations in untap, end-step cleanup, and combat-state clearing
- keep battlefield-wide mutation aligned with the engine's locality and embedded goals
- simplify the hot mutation path without widening storage escape hatches again

## Target Shape

- battlefield-wide mutation can visit cards safely without collecting handles into a temporary `Vec`
- turn-flow and combat cleanup callers keep a semantic API instead of open-coding storage details
- the optimized path remains compatible with player-owned handle storage

## Invariants

- battlefield mutation still respects aggregate ownership
- callers do not regain raw mutable access to battlefield storage internals
- this slice does not expand supported Magic rules
