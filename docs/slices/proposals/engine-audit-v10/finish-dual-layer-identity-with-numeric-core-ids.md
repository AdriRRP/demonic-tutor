# Slice Name

`FinishDualLayerIdentityWithNumericCoreIds`

## Goal

Complete the dual-layer identity model so the runtime core uses numeric/internal identities canonically while public string-backed ids are materialized only at true outward-facing boundaries.

## Why This Slice Exists Now

The engine already leans on indices, handles, and stack object numbers, but the canonical public id types are still string-first value objects. The remaining excellence move is to make numeric core identity the real source of truth.

## Supported Behavior

- core gameplay logic relies on numeric/internal identities as canonical state
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
- identity handling across `Game`, `Player`, stack, and location helpers

### Entity / Value Object Impact
- `ids.rs`
- stack/event materialization
- any helper still carrying string ids internally without boundary need

## Ownership Check

This belongs to the `Game` aggregate and gameplay domain because runtime identity is a core modeling concern of authoritative game state.

## Documentation Impact

- this slice document
- `docs/slices/proposals/README.md`
- possibly ADR documentation if the refactor becomes the canonical identity policy

## Test Impact

- all gameplay regressions remain green
- focused regressions proving public ids stay stable even if the internal canonical identity changes

## Rules Reference

- no additional Comprehensive Rules scope; this is internal runtime identity work

## Rules Support Statement

This slice does not broaden Magic rules support. It completes the identity architecture behind the existing supported subset.

## Open Questions

- whether the final identity policy deserves an ADR once implemented
