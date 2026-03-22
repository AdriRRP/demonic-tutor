# Slice Proposal — Expand State Based Actions For New Effects

## Goal

Extend the current supported SBA subset so it remains semantically correct after the planned new spell effects and combat keywords land.

## Why This Slice Exists Now

As the engine grows into destruction, pump, and richer combat, the current SBA subset may stop being enough to keep the model honest. This slice closes those gaps explicitly instead of scattering fixes.

## Supported Behavior

- the supported SBA review grows only as required by the newly implemented effect and combat slices
- SBA review remains explicit, ordered, and deterministic

## Invariants / Legality Rules

- no unsupported SBA should be implied
- newly added SBA checks must correspond to actual supported gameplay behavior

## Out of Scope

- full Magic SBA coverage
- token-specific or multiplayer SBA unless directly required

## Domain Impact

- extend the current explicit SBA check sequence
- keep the implementation closed and aggregate-owned

## Ownership Check

State-based action review is already aggregate-owned gameplay legality.

## Documentation Impact

- current-state
- rules-map if a new supported SBA rule is added
- implemented slice doc

## Test Impact

- focused unit tests around each newly supported SBA case

## Rules Reference

- 704

## Rules Support Statement

This slice expands the supported SBA subset only as far as newly implemented behavior requires. It does not claim full rule coverage.
