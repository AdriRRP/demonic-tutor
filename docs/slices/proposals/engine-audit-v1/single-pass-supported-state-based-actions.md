# Slice Proposal — Single Pass Supported State Based Actions

## Goal

Reduce the current supported SBA review to one battlefield sweep per iteration instead of separate zero-toughness and lethal-damage scans.

## Why This Slice Exists Now

The supported SBA subset is still small and explicit, but the current implementation scans battlefields twice and allocates intermediate vectors for each pass. The next clean refactor is to keep the same explicit semantics with a cheaper review loop.

## Supported Behavior

- zero-toughness and lethal-damage creature checks are gathered in one battlefield sweep per iteration
- zero-life game end remains explicit and ordered after creature checks
- the supported SBA loop stays deterministic

## Invariants / Legality Rules

- no unsupported SBA are implied
- the order of the currently supported checks stays explicit
- repeated SBA iterations still continue until no supported change remains

## Out of Scope

- broader SBA coverage
- replacement effects
- trigger handling

## Domain Impact

- compress the current explicit SBA corridor into a cheaper per-iteration battlefield review
- preserve the current aggregate-owned SBA model

## Ownership Check

Supported SBA review already belongs inside the `Game` aggregate.

## Documentation Impact

- `docs/domain/current-state.md` only if the supported SBA wording changes
- this proposal file

## Test Impact

- existing SBA unit tests should continue to pass
- add focused regression tests only if the internal ordering changes

## Rules Reference

- 704

## Rules Support Statement

This slice optimizes the current explicit SBA subset only. It does not claim broader SBA support.
