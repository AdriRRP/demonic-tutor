# Slice Name

`ThinStackPayloadsBeyondStaticSpellMetadata`

## Goal

Reduce `SpellPayload` further so spells on the stack carry only in-flight resolution state, not broad copies of static card metadata.

## Why This Slice Exists Now

`SpellPayload` is already lighter than earlier versions, but each spell still stores card kind and rule metadata that are mostly canonical definition data rather than runtime state.

## Supported Behavior

- stack-borne spells carry a smaller runtime payload
- static spell definition data is derived or referenced from a narrower canonical source
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
- any helper rebuilding or reading spell metadata at resolution time

## Ownership Check

This belongs to the `Game` aggregate because stack object representation is internal aggregate state.

## Documentation Impact

- this slice document
- `docs/slices/proposals/README.md`

## Test Impact

- existing stack, targeting, and spell-resolution regressions remain green
- focused regression proving payload thinning does not change destination or effect semantics

## Rules Reference

- 601.2 — casting process context for spells becoming stack objects
- 608.2 — resolving spells and abilities

## Rules Support Statement

This slice keeps the same supported stack semantics while reducing redundant per-spell metadata in the runtime carrier.

## Open Questions

- none
