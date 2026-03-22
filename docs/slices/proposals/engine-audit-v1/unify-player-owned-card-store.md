# Slice Proposal — Unify Player Owned Card Store

## Goal

Replace the five per-zone `HashMap<CardInstanceId, CardInstance>` stores on `Player` with one player-owned card store shared by library, hand, battlefield, graveyard, and exile.

## Why This Slice Exists Now

The current id-backed zones already proved the storage direction, but `Player` still duplicates ownership across one map per zone plus one ordered carrier per zone. A single owned card store is the next coherent step for memory, invariants, and future runtime indexing.

## Supported Behavior

- `Player` owns cards through one internal `CardInstanceId -> CardInstance` store
- zones continue to expose the same semantic ordered views by `CardInstanceId`
- moving a card between player-owned zones no longer moves it between different maps

## Invariants / Legality Rules

- a card owned by a player exists in exactly one player-owned zone at a time
- zones keep their current ordering semantics
- no gameplay rule support changes

## Out of Scope

- multiplayer ownership
- cross-player control changes
- stack or global card registry

## Domain Impact

- collapse duplicated player-owned storage into one store plus zone location semantics
- keep `Game` aggregate ownership unchanged

## Ownership Check

This belongs to the `Game` aggregate through `Player`, because card ownership and zone membership remain aggregate-owned runtime state.

## Documentation Impact

- `docs/domain/aggregate-game.md`
- `docs/domain/current-state.md`
- ADRs only if the storage direction materially changes

## Test Impact

- focused unit tests for hand, battlefield, graveyard, exile, and library transitions
- regression tests for ownership and lookup after moves

## Rules Reference

- none beyond current supported zone semantics

## Rules Support Statement

This slice is a runtime-storage refactor only. It does not expand Magic rules support.
