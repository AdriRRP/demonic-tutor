# Slice Proposal — Pump Spell Changes Combat Outcome

## Goal

Prove that the current supported temporary pump effect can change a combat result in the same turn.

## Why This Slice Exists Now

The pump foundation is not very meaningful unless it can actually alter combat. This slice exercises the real gameplay payoff of the temporary buff model.

## Supported Behavior

- a supported pump spell may be cast in a combat window
- the temporary stat change affects later supported combat damage and survival outcomes that turn

## Invariants / Legality Rules

- the buff lasts through the current turn's supported combat flow
- lethal and nonlethal outcomes continue to use shared combat and SBA corridors

## Out of Scope

- trample interactions
- multiple combat-damage steps

## Domain Impact

- likely no new abstractions beyond the temporary buff introduced earlier
- validates interaction between stack resolution and combat damage

## Ownership Check

This is aggregate-owned because it couples spell resolution to combat-state legality and damage.

## Documentation Impact

- current-state
- implemented slice doc

## Test Impact

- executable BDD showing changed combat outcome
- unit coverage for the underlying combat interaction

## Rules Reference

- 510
- 611
- 613
- 704

## Rules Support Statement

This slice proves the currently supported temporary pump effect can influence the same turn's combat. It does not add broader combat mechanics by itself.
