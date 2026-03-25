# Slice Name

Enter The Battlefield Trigger Foundation

## Goal

Introduce the first supported triggered-ability family for permanents entering the battlefield.

## Why This Slice Exists Now

Triggered abilities are one of the largest missing families blocking the engine from feeling like usable Magic. Enter-the-battlefield triggers are common, intuitive, and high-value while still local to one clear event.

## Supported Behavior

- detect when a supported permanent enters the battlefield
- enqueue one supported ETB trigger under that permanent's controller
- put the trigger onto the stack using the existing priority corridor
- resolve the trigger through the stack like another stack object family

## Invariants / Legality Rules

- supported ETB triggers only trigger from actual battlefield entry
- the trigger is controlled by the controller of the permanent at the moment it entered
- the trigger uses the stack and priority rather than resolving immediately

## Out of Scope

- multiple simultaneous ETB ordering across many triggers
- intervening-if clauses
- optional “you may” choices
- replacement effects that change how the permanent entered

## Domain Impact

### Aggregate Impact

- add a first triggered-ability object family and enqueue corridor

### Entity / Value Object Impact

- extend card-face supported behavior with ETB trigger profiles

### Events

- likely add trigger-put-on-stack visibility

### Errors

- no large new public error family required in the first slice

## Ownership Check

This belongs to the `Game` aggregate because trigger creation, stack insertion, and resolution are gameplay-legal state transitions owned by the aggregate.

## Documentation Impact

- `docs/domain/current-state.md`
- `docs/domain/aggregate-game.md`
- `docs/rules/rules-map.md`
- this slice doc

## Test Impact

- supported ETB trigger goes to the stack after the permanent enters
- trigger resolves through the existing stack corridor
- the trigger is controlled by the permanent's controller

## Rules Reference

- 603
- 603.2
- 603.3
- 405
- 117

## Rules Support Statement

This slice introduces a minimal ETB trigger engine for explicitly modeled triggers only. It does not imply general triggered-ability coverage or full APNAP trigger ordering.

