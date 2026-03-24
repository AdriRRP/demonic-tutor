# Slice Proposal — Close Player Zone Storage Escape Hatches

## Goal

Stop exposing raw mutable zone storage from `Player`, especially `library_mut`, and route all zone mutation through semantic domain operations.

## Why This Slice Exists Now

`Player` is already the aggregate façade for owned-card transitions, but a few raw storage entrypoints still let rules bypass those semantics.

This slice exists to:

1. tighten ownership and zone-transition invariants
2. reduce future refactor surface for storage changes
3. keep gameplay rules phrased as domain actions instead of collection edits

## Supported Behavior

- gameplay rules mutate player-owned zones only through semantic operations
- `Player` no longer exposes the current mutable storage escape hatches that bypass those operations
- observable gameplay behavior remains unchanged

## Invariants / Legality Rules

- zone transitions remain explicit and reviewable
- no gameplay legality changes
- no broader Magic support is implied

## Out of Scope

- changing supported zones
- multiplayer ownership
- new gameplay rules

## Domain Impact

- aggregate-internal API tightening on `Player`

## Documentation Impact

- this slice document

## Test Impact

- regression coverage for draw, recycle, casting, discard, and other zone transitions remains green

## Rules Reference

- none beyond current supported zone semantics

## Rules Support Statement

This slice is an architectural tightening only. It does not expand Magic rules support.
