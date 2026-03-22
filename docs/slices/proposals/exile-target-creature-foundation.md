# Slice Proposal — Exile Target Creature Foundation

## Goal

Support a targeted spell that exiles a creature from the battlefield.

## Why This Slice Exists Now

The repo already models exile as a player-owned zone and explicit non-stack exile actions. The next coherent step is to connect exile to the supported targeted spell corridor.

## Supported Behavior

- a supported targeted spell may choose a legal creature on the battlefield
- on resolution, the target creature is moved to its owner's exile zone

## Invariants / Legality Rules

- the spell requires one legal creature target
- the effect applies only if the target remains legal on resolution
- ownership of the exile destination remains explicit

## Out of Scope

- blink effects
- temporary exile-and-return behavior
- exile from zones other than battlefield

## Domain Impact

- extend supported resolution profiles with exile-to-owner-exile behavior
- reuse current exile zone movement semantics

## Ownership Check

This belongs to aggregate-owned targeting, resolution, and zone movement.

## Documentation Impact

- current-state
- implemented slice doc

## Test Impact

- unit tests for positive, negative, and resolution-loss paths
- executable BDD for one positive corridor

## Rules Reference

- 114
- 406
- 608.2b

## Rules Support Statement

This slice adds a targeted exile effect for battlefield creatures only. It does not imply temporary exile or broader exile interactions.
