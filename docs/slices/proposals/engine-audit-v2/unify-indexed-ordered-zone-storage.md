# Slice Proposal — Unify Indexed Ordered Zone Storage

## Goal

Extract a shared internal representation for ordered id-backed zones that currently duplicate `Vec<CardInstanceId> + HashMap<CardInstanceId, usize>`.

## Why This Slice Exists Now

`Hand`, `Graveyard`, and `Exile` share nearly the same structure and removal costs, which increases drift risk and makes future storage tuning more expensive than necessary.

This slice exists to:

1. centralize ordered-zone storage semantics
2. reduce duplicated maintenance and indexing logic
3. create one place to improve ordered-zone performance later

## Supported Behavior

- `Hand`, `Graveyard`, and `Exile` keep their current ordering semantics
- membership and positional lookup remain available
- removal semantics stay explicit and deterministic

## Invariants / Legality Rules

- zone ordering remains truthful to the current model
- no gameplay legality changes
- no broader rules support is implied

## Out of Scope

- battlefield storage changes
- library storage changes
- multiplayer ownership

## Domain Impact

- ordered player-owned zones share a common internal storage strategy

## Documentation Impact

- this slice document

## Test Impact

- regression coverage for draw, discard, exile, graveyard targeting, and casting remains green

## Rules Reference

- none beyond current zone semantics

## Rules Support Statement

This slice is a storage-structure refactor only. It does not expand Magic rules support.
