# Proposal Slice — Introduce Dense Player Card Arena

## Summary

Replace the per-player `HashMap<CardInstanceId, CardInstance>` ownership store with a dense arena keyed by compact internal handles.

## Motivation

- reduce heap overhead and hashing on every zone lookup and movement
- make zone storage cheaper by storing compact handles instead of cloned public ids
- prepare the engine for tighter memory targets such as ESP32-class devices

## Target Shape

- each player owns one dense card arena
- zones store internal handles
- public ids are projected only at aggregate boundaries, events, and tests

## Invariants

- gameplay semantics stay unchanged
- zone ownership remains explicit and deterministic
- this slice does not expand supported Magic rules

## Notes

- this is the highest-return storage refactor still open in the engine
- follow-up slices should build on this instead of widening hash-based stores further
