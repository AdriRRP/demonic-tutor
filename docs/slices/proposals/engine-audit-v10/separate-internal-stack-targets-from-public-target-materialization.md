# Slice Name

`SeparateInternalStackTargetsFromPublicTargetMaterialization`

## Goal

Split the stack’s internal target representation from its public target DTOs so internal stack logic does not cohabit with public-id target materialization in the same model layer.

## Why This Slice Exists Now

`StackTargetRef` is already the right runtime representation, but `SpellTarget` still lives alongside it in the stack model, which keeps the boundary between internal and outward-facing identity blurrier than necessary.

## Supported Behavior

- stack logic uses internal target references only
- public spell targets are materialized only when crossing outward-facing boundaries such as events or user-facing errors
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

- this slice document
- `docs/slices/proposals/README.md`

## Test Impact

- stack and targeting regressions stay green
- focused regression around target materialization in outward-facing events

## Rules Reference

- 114.1 — targets as spell/ability objects
- 601.2c — target choice during casting
- 608.2b — target legality on resolution

## Rules Support Statement

This slice preserves the current supported target subset and only tightens how internal and public target identities are separated.

## Open Questions

- none
