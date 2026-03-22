# Slice Proposal — Target Any Player Spell Foundation

## Goal

Introduce an explicit non-contextual `any player` target rule for supported spells.

## Why This Slice Exists Now

The current model already distinguishes `opponent player`. To stay semantically honest, unrestricted player targeting should be represented as its own rule rather than inferred from looser behavior.

## Supported Behavior

- a supported spell may target any player explicitly
- the acting player may choose self or opponent when the rule allows it

## Invariants / Legality Rules

- the target must be an existing player in the game
- unrestricted player targeting is still an explicit rule, not a fallback

## Out of Scope

- multiplayer targeting semantics
- multiple player targets

## Domain Impact

- extend target-legality rules with explicit `any player`
- reuse the current single-target shared legality corridor

## Ownership Check

This belongs to aggregate-owned targeting and spell-casting legality.

## Documentation Impact

- glossary if the term becomes canonical
- current-state
- implemented slice doc

## Test Impact

- unit coverage for self and opponent selection
- executable BDD for one positive cast

## Rules Reference

- 114
- 601.2c
- 608.2b

## Rules Support Statement

This slice adds an explicit unrestricted player-target rule. It does not imply multiple players or multiplayer targeting semantics.
