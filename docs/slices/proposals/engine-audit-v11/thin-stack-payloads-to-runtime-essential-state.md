# Slice Name

`ThinStackPayloadsToRuntimeEssentialState`

## Status

Proposed

## Goal

Reduce stack-borne spell payloads to the minimum runtime state needed for resolution so each in-flight spell stops carrying a mini-definition record.

## Why This Slice Exists Now

The stack already stopped reusing full card runtimes, but `SpellPayload` still carries metadata such as public ids, card type, and supported rule profiles that are partly canonical definition data rather than true in-flight state. That keeps each spell wider than necessary and slows the final convergence toward a compact runtime model.

## Supported Behavior

- stack objects continue resolving correctly for the currently supported spell subset
- runtime payloads keep only the state required for resolution, destination, and temporary combat/runtime effects
- definition-oriented metadata is derived from a cheaper shared lookup or a thinner canonical reference

## Invariants / Legality Rules

- spell resolution semantics remain unchanged for the current subset
- the stack continues to be self-sufficient for supported resolution
- unsupported spell families remain explicitly unsupported

## Out of Scope

- adding new spell families
- changing public event payloads
- introducing generic rules-text evaluation

## Domain Impact

### Entity / Value Object Impact
- stack spell payloads
- spell-resolution carriers
- card-definition lookup shape if needed

## Ownership Check

This belongs to the gameplay domain because the shape of a spell while it exists on the aggregate-owned stack is part of aggregate runtime state.

## Documentation Impact

- `docs/domain/aggregate-game.md`
- `docs/domain/current-state.md`
- `docs/architecture/runtime-abstractions.md`
- this slice document

## Test Impact

- payload round-trip regressions for the supported creature, permanent, and effect spell corridors
- focused resolution tests proving no supported spell behavior changes

## Rules Reference

- no additional Comprehensive Rules scope; this is a runtime representation refinement

## Rules Support Statement

This slice preserves the current supported stack subset while making in-flight spell state more compact and explicit.
