# Slice Name

End Step Trigger Foundation

## Goal

Support the first triggered abilities that trigger at the beginning of the end step.

## Why This Slice Exists Now

The engine already models `EndStep`, cleanup discard, and end-step priority. Beginning-of-end-step triggers unlock many common “until next end step” and delayed value patterns.

## Supported Behavior

- detect the beginning of `EndStep`
- enqueue supported end-step triggers from supported battlefield permanents across all controllers before ordinary end-step priority actions
- resolve those triggers through the stack

## Invariants / Legality Rules

- supported end-step triggers happen once on entering `EndStep`
- they must be placed onto the stack before ordinary end-step responses
- they do not repeat during cleanup handling in the same turn

## Out of Scope

- delayed “at the beginning of the next end step” objects created from arbitrary prior effects
- cleanup-step triggers
- full “until end of turn” state rollback beyond the currently supported subset

## Domain Impact

### Aggregate Impact

- extend turn-flow entry semantics for `EndStep`

### Entity / Value Object Impact

- supported end-step trigger profiles on card face metadata

## Ownership Check

This belongs to the `Game` aggregate because phase entry timing and triggered stack insertion are gameplay-domain responsibilities.

## Documentation Impact

- `docs/domain/current-state.md`
- `docs/rules/rules-map.md`
- this slice doc

## Test Impact

- supported end-step trigger is put on the stack when `EndStep` begins
- trigger resolves before cleanup-only follow-up actions
- trigger does not duplicate within the same end step

## Rules Reference

- 513
- 603
- 117

## Rules Support Statement

This slice adds a minimal beginning-of-end-step trigger family only. It does not yet imply generic delayed trigger support or full cleanup-step trigger handling.
