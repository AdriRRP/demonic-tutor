# Slice Proposal — Resolve Opponents Creature Spell In First Main

## Goal

Exercise successful resolution of a supported `opponents creature` spell during `FirstMain`.

## Why This Slice Exists Now

The current targeting model should be proven not only at cast time but through full stack resolution with the same legality rule reused on revalidation.

## Supported Behavior

- a supported spell targeting an opponent-controlled creature resolves in `FirstMain`
- the effect is applied only if the target remains legal on resolution

## Invariants / Legality Rules

- resolution must reuse the shared legal-target evaluation
- illegal-on-resolution targets cause the effect not to apply

## Out of Scope

- complex replacement effects
- multiple targets

## Domain Impact

- likely no new abstractions if the previous slices already landed cleanly
- strengthens end-to-end semantics around cast and resolution

## Ownership Check

The stack, target revalidation, and effect application remain aggregate-owned.

## Documentation Impact

- current-state
- implemented slice doc

## Test Impact

- unit resolution coverage
- executable BDD for the full corridor

## Rules Reference

- 114
- 601.2c
- 608.2b

## Rules Support Statement

This slice closes the full cast-to-resolution corridor for `creature an opponent controls` in the supported non-combat targeted-spell subset.
