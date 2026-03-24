# Slice Name

`FinishDualLayerIdentityWithNumericCoreIds`

## Status

Implemented

## Goal

Complete the dual-layer identity model so the runtime core uses numeric/internal identities canonically while public string-backed ids are materialized only at true outward-facing boundaries.

## What Changed

- public gameplay ids now carry a numeric core identity plus stable public text
- `GameId`, `PlayerId`, `CardInstanceId`, `CardDefinitionId`, `DeckId`, and `StackObjectId` intern their public string into a shared numeric core id
- equality and hashing for those ids now rely on the numeric core while `Display` and `as_str()` keep the same outward-facing text
- focused regressions prove that repeated construction of the same public string reuses the same numeric core without changing observable ids

## Supported Behavior

- core gameplay logic can rely on numeric/internal identity carried by the id value objects
- public string-backed ids remain available at commands, events, serialization, and tests
- outward-facing behavior remains deterministic and stable

## Invariants / Legality Rules

- public ids remain reviewable and deterministic at boundaries
- internal identities remain explicit and stable within aggregate state
- this slice does not expand supported Magic rules

## Out of Scope

- changing user-facing event semantics
- broad serialization redesign beyond identity materialization
- unrelated gameplay rules work

## Domain Impact

### Aggregate Impact
- identity handling across `Game`, `Player`, stack, and boundary-facing helpers

### Entity / Value Object Impact
- `ids.rs`
- boundary id materialization across commands, events, and tests

## Ownership Check

This belongs to the `Game` aggregate and gameplay domain because runtime identity is a core modeling concern of authoritative game state.

## Documentation Impact

- this implemented slice document
- `docs/slices/proposals/README.md`

## Test Impact

- all gameplay regressions remain green
- focused regressions prove that public ids stay stable while numeric core ids are reused internally

## Rules Reference

- no additional Comprehensive Rules scope; this is internal runtime identity work

## Rules Support Statement

This slice does not broaden Magic rules support. It completes the identity architecture behind the existing supported subset.
