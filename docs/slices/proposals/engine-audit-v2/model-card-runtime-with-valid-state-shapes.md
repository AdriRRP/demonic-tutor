# Slice Proposal — Model Card Runtime With Valid State Shapes

## Goal

Refactor `CardInstance` runtime so the supported runtime shape cannot represent impossible combinations between card type and creature state.

## Why This Slice Exists Now

The current model stores `card_type` separately from `runtime.creature: Option<_>`, which keeps invalid combinations representable and forces repeated defensive checks.

This slice exists to:

1. align runtime state with supported card kinds
2. remove impossible states by construction
3. prepare future permanent semantics on top of a cleaner Rust model

## Supported Behavior

- the current supported card subset behaves exactly as before
- creature-specific runtime state exists only on creature cards
- non-creature cards no longer carry an optional creature runtime shape

## Invariants / Legality Rules

- no non-creature card may carry creature-only runtime
- creature runtime remains explicit for power, toughness, damage, combat flags, and temporary pump
- no gameplay legality changes

## Out of Scope

- new card types
- triggered abilities
- control-changing effects

## Domain Impact

- `CardInstance` internal runtime representation becomes more semantically constrained

## Documentation Impact

- this slice document
- update canonical docs only if a stable architectural lesson needs recording

## Test Impact

- full combat, targeting, casting, and zone regression coverage remains green

## Rules Reference

- none beyond the currently supported permanent subset

## Rules Support Statement

This slice is a runtime-model refactor only. It does not expand Magic rules support.
