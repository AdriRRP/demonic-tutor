# Slice Name

`MoveStackSourceAndAbilityReferencesToInternalRefs`

## Status

Implemented

## Goal

Replace remaining public-id-based activated-ability stack source references with internal owner/handle references so the stack runtime stops depending on `CardInstanceId` in its hot paths.

## What Changed

- activated abilities now enter the stack with a canonical `StackCardRef`
- activation resolves the public `source_card_id` once at the boundary and then works through `owner_index + handle`
- resolution materializes the readable `CardInstanceId` from the internal stack reference only when building outward-facing events
- stack inspection compatibility remains available through the public source-id accessor

## Supported Behavior

- activated abilities continue entering and resolving through the stack
- spell and ability source lookup continues working for supported events, tests, and errors
- public ids are materialized at runtime boundaries instead of acting as the canonical stack source reference

## Invariants / Legality Rules

- every activated-ability stack object still references a valid source within the aggregate
- controller ownership remains explicit
- supported activated-ability behavior remains unchanged

## Out of Scope

- adding new ability families
- changing public event semantics
- redesigning command DTOs

## Domain Impact

### Entity / Value Object Impact
- activated-ability stack carriers
- stack-source materialization helpers
- activation and resolution corridors

## Ownership Check

This belongs to the gameplay domain because stack-source identity is aggregate-owned runtime state.

## Documentation Impact

- `docs/architecture/runtime-abstractions.md`
- `docs/slices/proposals/README.md`
- this implemented slice document

## Test Impact

- activated-ability stack regressions remain green
- stack inspection regressions still prove readable public source ids at the boundary

## Rules Reference

- no additional Comprehensive Rules scope; this slice only refines internal source identity

## Rules Support Statement

This slice preserves the current stack and activated-ability subset while moving source references further away from public ids.
