# Slice: Use Explicit Location Lookup For Aura Detach

Status: implemented

## Summary

Aura detach cleanup now uses explicit location lookup when the aggregate already has the shared card-location index available, while preserving a safe fallback for callers that do not.

## What changed

- battlefield-to-graveyard and battlefield-to-hand zone moves accept optional shared location context
- supported detach corridors now resolve the enchanted permanent through the aggregate location index instead of a full battlefield scan
- callers without index context still fall back safely to the older scan path

## Why it matters

- removes avoidable global battlefield scans from common zone-exit paths
- keeps attachment cleanup aligned with the rest of the aggregate's location model
- improves performance without forcing unrelated callers to widen their local state
