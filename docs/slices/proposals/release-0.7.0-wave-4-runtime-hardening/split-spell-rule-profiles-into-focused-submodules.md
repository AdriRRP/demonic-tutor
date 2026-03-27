# Slice Name

`SplitSpellRuleProfilesIntoFocusedSubmodules`

## Goal

Reduce the cognitive hotspot around spell targeting and resolution profiles by moving them into focused submodules without changing the supported spell subset.

## Why This Slice Exists Now

`SupportedSpellRules` is now the meeting point for most new spell slices. Keeping all targeting and resolution variants in one file makes each new capability more expensive to review and easier to degrade.

## Supported Behavior

- keep the current `SupportedSpellRules` semantics intact
- move targeting and spell-resolution profile definitions into focused internal submodules
- keep the public card-rules API stable for the rest of the engine

## Invariants / Legality Rules

- no spell legality or resolution behavior changes
- no rules support is widened or narrowed

## Out of Scope

- replacing explicit enums with a generic effect engine
- changing the card-definition authoring surface
- adding new spell effects

## Domain Impact

### Entity / Value Object Impact
- internal module split of card rule profiles

## Ownership Check

This belongs inside the gameplay domain model because it restructures how supported spell semantics are represented, while preserving the same domain boundary.

## Documentation Impact

- this slice document
- `docs/architecture/runtime-abstractions.md`

## Test Impact

- current spell tests remain green with no behavior change

## Rules Reference

- no new rules support; references remain those already owned by current spell slices

## Rules Support Statement

This slice is structural only. It preserves the current supported spell subset.

## Open Questions

- none
