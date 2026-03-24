# Slice Name

`SeparateInternalStackTargetsFromPublicTargetMaterialization`

## Status

Implemented

## Goal

Split the stack’s internal target representation from its public target DTOs so internal stack logic does not cohabit with public-id target materialization in the same model layer.

## What Changed

- `SpellTarget` no longer lives in the internal stack model module.
- the internal stack model keeps only runtime-facing target references such as `StackTargetRef`.
- public `SpellTarget` materialization now lives in a separate `game::targets` boundary module and is re-exported through the same public game API.

## Supported Behavior

- stack logic uses internal target references only
- public spell targets are materialized only when crossing outward-facing boundaries such as events or command-facing APIs
- supported target legality and resolution behavior remain unchanged

## Invariants / Legality Rules

- internal stack targets remain explicit and deterministic
- outward-facing events still expose stable public ids
- this slice does not expand supported Magic rules

## Out of Scope

- adding new target families
- changing cast-time or resolution-time legality rules
- changing public event shapes unless strictly required for boundary separation

## Domain Impact

### Aggregate Impact
- cleaner boundary between runtime stack state and boundary DTOs

### Entity / Value Object Impact
- stack target types
- stack resolution helpers
- stack event materialization helpers

## Ownership Check

This belongs to the `Game` aggregate because stack target identity is part of internal gameplay state and resolution.

## Documentation Impact

- this implemented slice document
- `docs/slices/proposals/README.md`

## Test Impact

- stack and targeting regressions stay green
- no observable gameplay behavior changes

## Rules Reference

- 114.1 — targets as spell/ability objects
- 601.2c — target choice during casting
- 608.2b — target legality on resolution

## Rules Support Statement

This slice preserves the current supported target subset and only tightens how internal and public target identities are separated.
