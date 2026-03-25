# Slice Name

`ThinStackPayloadsToRuntimeEssentialState`

## Status

Implemented

## Goal

Reduce stack-borne spell payloads to the minimum runtime state needed for resolution so each in-flight spell stops carrying repeated static metadata.

## What Changed

- `SpellPayload` now encodes more spell family semantics in the enum variant itself
- effect payloads no longer store `card_type`; `Instant` and `Sorcery` are represented as distinct payload families
- permanent payloads no longer store `card_type`; `Artifact`, `Enchantment`, and `Planeswalker` are represented as distinct payload families
- creature payloads keep only runtime-essential creature data such as stats and keywords
- round-trip reconstruction still preserves the currently supported subset

## Supported Behavior

- stack objects continue resolving correctly for the supported creature, instant, sorcery, artifact, enchantment, and planeswalker subset
- runtime payloads keep only the state required for resolution, destination, and supported permanent reconstruction
- definition-oriented metadata is carried in a thinner shape than before

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
- spell-resolution reconstruction

## Ownership Check

This belongs to the gameplay domain because the shape of a spell while it exists on the aggregate-owned stack is part of aggregate runtime state.

## Documentation Impact

- `docs/architecture/runtime-abstractions.md`
- `docs/slices/proposals/README.md`
- this implemented slice document

## Test Impact

- payload round-trip regressions for supported permanent, effect, and creature spells remain green
- focused runtime tests prove the thinner payload still preserves relevant supported semantics

## Rules Reference

- no additional Comprehensive Rules scope; this is a runtime representation refinement

## Rules Support Statement

This slice preserves the current supported stack subset while making in-flight spell state more compact and explicit.
