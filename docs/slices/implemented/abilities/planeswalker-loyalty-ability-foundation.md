# Slice Name

Planeswalker Loyalty Ability Foundation

## Goal

Support the first explicit planeswalker loyalty abilities in the current subset.

## Why This Slice Exists Now

Planeswalkers are already castable permanents in the current engine but not yet meaningfully usable. A narrow loyalty-ability slice gives them real gameplay value without committing to the full planeswalker ruleset at once.

## Supported Behavior

- allow a supported planeswalker to activate one explicit loyalty ability during the controller's main phase
- add or remove loyalty as part of activation cost
- put the loyalty ability on the stack and resolve it through the normal corridor
- enforce the current one-loyalty-activation-per-turn limit for that supported planeswalker

## Invariants / Legality Rules

- loyalty abilities follow the current active-player main-phase timing restriction for the supported subset
- changing loyalty is part of activation, not resolution
- activation fails if the planeswalker lacks enough loyalty for a minus ability
- a supported planeswalker cannot activate a second loyalty ability in the same turn

## Out of Scope

- planeswalker combat damage redirection history
- static or triggered planeswalker text

## Domain Impact

### Aggregate Impact

- extend planeswalker runtime with loyalty counters and activation restrictions

## Ownership Check

This belongs to the `Game` aggregate because loyalty state, activation timing, and stack insertion are gameplay-domain responsibilities.

## Documentation Impact

- `docs/domain/current-state.md`
- `docs/domain/aggregate-game.md`
- `docs/rules/rules-map.md`
- this slice doc

## Test Impact

- activate a plus loyalty ability
- activate a minus loyalty ability when enough loyalty exists
- reject minus activation when loyalty is insufficient
- ability uses the stack and resolves normally

## Rules Reference

- 306
- 606

## Rules Support Statement

This slice introduces a minimal explicit loyalty-ability corridor for supported planeswalkers only. It does not imply full planeswalker rules coverage.
