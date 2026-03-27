# Slice Name

`MoveCombatParticipantsToInternalCardReferences`

## Status

Implemented

## Goal

Keep combat damage participants on internal card references until the last outward-facing boundary instead of cloning public `CardInstanceId` values during participant collection.

## What Changed

- combat damage participants now carry internal `player_index + handle` references instead of public card ids
- blocker-to-attacker linkage is preserved through those internal references during participant collection
- public `CardInstanceId` values are materialized only when building `DamageEvent` records or applying marked damage

## Supported Behavior

- combat damage participant collection uses internal card references
- public card ids are materialized only where events or damage application require them
- current combat semantics remain unchanged

## Invariants / Legality Rules

- each combat participant still refers to one concrete attacker or blocker
- supported trample and first-strike semantics stay unchanged
- this slice does not expand supported Magic rules

## Out of Scope

- multi-blocker combat
- combat damage assignment redesign
- new keyword ability support

## Domain Impact

### Aggregate Impact
- internal combat damage corridor becomes more compact

### Entity / Value Object Impact
- combat participant carriers
- combat damage helpers
- event materialization at the damage boundary

## Ownership Check

This belongs to the `Game` aggregate because combat participant identity is internal aggregate state.

## Documentation Impact

- this implemented slice document
- `docs/slices/proposals/README.md`

## Test Impact

- combat regressions for normal damage, trample, and first strike remain green
- focused regression now proves blocker collection keeps the attacker linkage through internal handles

## Rules Reference

- 508 — declare attackers
- 509 — declare blockers
- 510 — combat damage step

## Rules Support Statement

This slice does not broaden combat rules support. It only keeps the supported combat corridor on internal references for longer.
