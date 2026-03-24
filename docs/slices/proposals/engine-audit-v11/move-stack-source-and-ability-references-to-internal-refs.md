# Slice Name

`MoveStackSourceAndAbilityReferencesToInternalRefs`

## Status

Proposed

## Goal

Replace remaining public-id-based stack source references with internal owner/handle references so the stack runtime stops depending on `CardInstanceId` in its hot paths.

## Why This Slice Exists Now

The stack already materializes `StackObjectId` only at boundaries and now uses internal target refs, but activated abilities and some source accessors still rely directly on public card ids. That keeps a clear piece of boundary identity embedded in one of the hottest runtime corridors.

## Supported Behavior

- activated abilities continue entering and resolving through the stack
- spell and ability source lookup continues working for supported events and errors
- public ids are materialized only when leaving the runtime boundary

## Invariants / Legality Rules

- every stack object still references a valid source within the aggregate
- controller ownership remains explicit
- supported activated-ability behavior remains unchanged

## Out of Scope

- adding new ability families
- changing public event semantics
- redesigning command DTOs

## Domain Impact

### Entity / Value Object Impact
- stack object families
- activated-ability carriers
- stack-origin and source lookup helpers

## Ownership Check

This belongs to the gameplay domain because stack-source identity is aggregate-owned runtime state.

## Documentation Impact

- `docs/domain/aggregate-game.md`
- `docs/domain/current-state.md`
- `docs/architecture/runtime-abstractions.md`
- this slice document

## Test Impact

- activated-ability stack regressions
- focused source-materialization tests for events and errors

## Rules Reference

- no additional Comprehensive Rules scope; this slice only refines internal source identity

## Rules Support Statement

This slice preserves the current stack and activated-ability subset while moving source references further away from public ids.
