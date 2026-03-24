# Slice Proposal — Derive Mana Pool Queries From Color Iteration

## Goal

Make `ManaPool` derive aggregate queries such as `total()` from the closed color set instead of enumerating colors manually.

## Why This Slice Exists Now

The array-backed mana representation is already the right base, but some API methods still hardcode the current color subset one field at a time.

This slice exists to:

1. remove leftover shape from the pre-array model
2. reduce drift risk as mana support grows
3. make the mana API smaller and more idiomatic

## Supported Behavior

- mana totals remain unchanged
- current color-specific queries remain unchanged where still useful
- observable payment behavior remains unchanged

## Invariants / Legality Rules

- no gameplay legality changes
- no broader mana support is implied
- the closed supported color set remains explicit

## Out of Scope

- new mana colors
- hybrid mana
- restricted mana

## Domain Impact

- `ManaPool` uses the closed supported color set more consistently

## Documentation Impact

- this slice document

## Test Impact

- mana and casting regression coverage remains green

## Rules Reference

- none beyond the current mana model

## Rules Support Statement

This slice is a local mana-model refactor only. It does not expand Magic rules support.
