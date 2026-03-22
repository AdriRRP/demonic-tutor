# Slice Proposal — Pump Target Creature Until End Of Turn

## Goal

Support a spell that gives a target creature `+N/+N until end of turn`.

## Why This Slice Exists Now

Pump effects are one of the smallest useful ways to open temporary power/toughness changes without yet introducing a full continuous-effects engine.

## Supported Behavior

- a supported spell may target a legal creature
- on resolution, the creature gets the explicit temporary stat increase until end of turn

## Invariants / Legality Rules

- the effect applies only if the target remains legal on resolution
- the temporary increase expires during the owned end-of-turn cleanup corridor

## Out of Scope

- layers
- permanent counters
- multiple overlapping temporary buffs from different sources unless directly required

## Domain Impact

- introduce minimal temporary creature stat modification
- extend end-of-turn cleanup to remove the current supported temporary buff subset

## Ownership Check

Temporary stat changes that affect legality and combat remain aggregate-owned.

## Documentation Impact

- current-state
- aggregate-game if creature runtime responsibilities materially grow
- implemented slice doc

## Test Impact

- unit tests for apply, expire, and combat interaction
- executable BDD for at least one positive cast

## Rules Reference

- 114
- 608.2b
- 611
- 613
- 514

## Rules Support Statement

This slice introduces a minimal temporary `+N/+N until end of turn` effect only. It does not imply a general continuous-effects or layer engine.
