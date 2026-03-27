# Slice Name

`UnifyPendingStackDecisions`

## Goal

Replace the parallel pending-decision fields in `Game` with one explicit closed `PendingDecision` concept so the stack corridor and the public contract can grow new decision families without multiplying structural branches.

## Why This Slice Exists Now

The current engine already supports optional decisions, hand-card decisions, and scry decisions. Keeping one field per family now creates repeated plumbing in `Game`, `StackPriorityContext`, `passing`, and the public contract.

## Supported Behavior

- represent the current pending decision families through one closed enum
- keep the current supported decision families: optional effect, pending hand choice, and pending scry
- let stack-priority handlers restore or consume the current decision through that unified type
- let the public gameplay projection derive actions and choice requests from that unified type

## Invariants / Legality Rules

- at most one pending decision may exist at a time
- pending decisions remain mutually exclusive with an open priority window in the current subset
- the controller of the pending decision remains the only legal resolver

## Out of Scope

- adding new decision families
- changing the public command surface
- changing the supported spell or trigger effects that create these decisions

## Domain Impact

### Aggregate Impact
- `Game` stores a single optional pending-decision state

### Entity / Value Object Impact
- introduce `PendingDecision`

### Commands
- no new commands

### Events
- no new events

## Ownership Check

This belongs to the `Game` aggregate because pending decision state is part of authoritative gameplay state around stack resolution and priority.

## Documentation Impact

- this slice document
- `docs/domain/current-state.md`
- `docs/domain/aggregate-game.md`

## Test Impact

- existing optional-effect, hand-choice, and scry flows still resolve correctly
- controller mismatch still restores the pending decision
- public legal actions and choice requests still reflect the pending decision correctly

## Rules Reference

- 117.3 — decisions and priority in the supported subset
- 608.2 — resolving objects with explicit player decisions, simplified to the supported subset

## Rules Support Statement

This slice does not add new gameplay support. It makes the current pending-decision model explicit and scalable.

## Open Questions

- none
