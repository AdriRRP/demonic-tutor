# Slice Name

Dies Trigger Foundation

## Goal

Support the first triggered abilities that fire when a creature dies from battlefield to graveyard.

## Why This Slice Exists Now

The engine already supports destruction and graveyard movement. Dies triggers reuse those semantics and make combat and removal much more realistic.

## Supported Behavior

- detect when a supported creature dies from battlefield to graveyard
- enqueue one supported dies trigger under the correct controller
- place that trigger on the stack
- resolve it through the existing stack corridor

## Invariants / Legality Rules

- “dies” means battlefield to graveyard only
- exile or bounce does not count as dying
- the controller of the trigger is determined from the permanent just before it left the battlefield

## Out of Scope

- leaves-the-battlefield triggers broader than “dies”
- multiple simultaneous dies-trigger ordering across many permanents
- last-known-information complexities beyond the supported dies subset

## Domain Impact

### Aggregate Impact

- extend trigger production from death events already modeled by combat and removal

### Entity / Value Object Impact

- add supported dies-trigger profiles on card face metadata

### Events

- trigger visibility and ordinary resolution events

## Ownership Check

This belongs to the `Game` aggregate because death detection and triggered stack insertion derive from aggregate-owned zone transitions and SBA outcomes.

## Documentation Impact

- `docs/domain/current-state.md`
- `docs/rules/rules-map.md`
- this slice doc

## Test Impact

- supported creature death creates a trigger
- exile does not create a dies trigger
- trigger resolves through stack order

## Rules Reference

- 603
- 603.6
- 700.4

## Rules Support Statement

This slice introduces a minimal explicit “dies” trigger family only. It does not imply broad leaves-the-battlefield support or full LKI trigger behavior.

