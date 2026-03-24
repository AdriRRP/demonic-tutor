# Slice Name

`ThinStackPayloadsBeyondStaticSpellMetadata`

## Status

Implemented

## Goal

Reduce `SpellPayload` further so spells on the stack carry only in-flight resolution state, not broad copies of static card metadata.

## What Changed

- `SpellPayload` no longer stores `mana_cost` in effect, permanent, or creature carriers.
- stack payload reconstruction now derives the rebuilt `CardDefinition` from the narrower canonical subset that still matters for the supported runtime:
  - `definition_id`
  - `card_type` when still needed
  - `supported_spell_rules` for effect spells
  - `activated_ability` for supported non-mana activated permanents
  - creature stats and keywords for creature permanents
- regression tests now pin that effect, permanent, and creature payloads still round-trip the supported semantics after this thinning.

## Supported Behavior

- stack-borne spells carry a smaller runtime payload
- static spell definition data is derived from a narrower canonical subset during reconstruction
- resolution behavior remains unchanged for the supported spell subset

## Invariants / Legality Rules

- a spell on the stack still carries enough information to resolve honestly
- public behavior and outward events remain unchanged
- this slice does not expand supported Magic rules

## Out of Scope

- adding new spell families
- changing supported targeting or timing rules
- redesigning the whole card-definition pipeline in one step

## Domain Impact

### Aggregate Impact
- slimmer stack object representation

### Entity / Value Object Impact
- `SpellPayload`
- stack resolution carriers
- spell-payload reconstruction into `CardInstance`

## Ownership Check

This belongs to the `Game` aggregate because stack object representation is internal aggregate state.

## Documentation Impact

- this implemented slice document
- `docs/slices/proposals/README.md`

## Test Impact

- existing stack, targeting, and spell-resolution regressions remain green
- focused runtime regressions prove payload thinning does not change destination or effect semantics for the supported subset

## Rules Reference

- 601.2 — casting process context for spells becoming stack objects
- 608.2 — resolving spells and abilities

## Rules Support Statement

This slice keeps the same supported stack semantics while reducing redundant per-spell metadata in the runtime carrier.
