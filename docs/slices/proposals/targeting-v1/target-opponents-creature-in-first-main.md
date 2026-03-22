# Slice Proposal — Target Opponents Creature In First Main

## Goal

Support a non-combat targeted spell that requires `creature an opponent controls` in `FirstMain`.

## Why This Slice Exists Now

The current targeting model already supports `creature you control` and combat-relative actor rules. The next coherent non-combat expansion is the symmetric `opponents creature` restriction.

## Supported Behavior

- the acting player may cast a supported targeted spell at an opponent-controlled creature in `FirstMain`
- the spell resolves through the shared targeted-spell corridor

## Invariants / Legality Rules

- the target must be a creature on the battlefield
- the target must be controlled by an opponent of the acting player
- the legal-target check is shared between cast and resolution

## Out of Scope

- multiplayer targeting
- creatures changing controller mid-resolution beyond the currently modeled subset

## Domain Impact

- extend target-legality rules with `opponents creature`
- reuse the current single-target contextual evaluation path

## Ownership Check

Target legality and spell resolution remain owned by the `Game` aggregate.

## Documentation Impact

- `docs/domain/current-state.md`
- `docs/domain/DOMAIN_GLOSSARY.md`
- the implemented slice doc for this capability

## Test Impact

- unit coverage for acceptance and resolution
- executable BDD in `FirstMain`

## Rules Reference

- 114
- 601.2c
- 608.2b

## Rules Support Statement

This slice extends the current non-combat targeting subset with `creature an opponent controls`. It does not yet imply broader controller-changing behavior.
