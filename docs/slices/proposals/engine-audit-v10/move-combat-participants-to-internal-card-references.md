# Slice Name

`MoveCombatParticipantsToInternalCardReferences`

## Goal

Keep combat damage participants on internal card references until the last outward-facing boundary instead of cloning public `CardInstanceId` values during participant collection.

## Why This Slice Exists Now

Combat runtime links already use handles for blocking relationships, but the damage corridor still materializes public ids early when building attacker and blocker participants.

## Supported Behavior

- combat damage participant collection uses internal card references
- public card ids are materialized only where events, errors, or other outward-facing boundaries require them
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
- any event or error adapter that still needs public ids

## Ownership Check

This belongs to the `Game` aggregate because combat participant identity is internal aggregate state.

## Documentation Impact

- this slice document
- `docs/slices/proposals/README.md`

## Test Impact

- combat regressions for normal damage, trample, and first strike remain green
- focused regression proving blocker-attacker linkage still resolves correctly after the internal refactor

## Rules Reference

- 508 — declare attackers
- 509 — declare blockers
- 510 — combat damage step

## Rules Support Statement

This slice does not broaden combat rules support. It only keeps the supported combat corridor on internal references for longer.

## Open Questions

- none
