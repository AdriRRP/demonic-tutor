# Slice Proposal — Target Self Player When Rule Allows It

## Goal

Support self-targeting when a spell's explicit target rule permits `any player`.

## Why This Slice Exists Now

Once `any player` exists as an explicit rule, the positive self-target case should be exercised separately from the opponent-target case so that self-targeting remains intentional, not accidental.

## Supported Behavior

- the acting player may cast a supported `any player` spell targeting themselves
- the spell resolves through the shared player-target corridor

## Invariants / Legality Rules

- self-targeting is legal only under the unrestricted `any player` rule
- the target must remain the same legal player on resolution

## Out of Scope

- player protection or hexproof-like effects
- multiplayer self/teammate distinctions

## Domain Impact

- no new targeting abstractions beyond `any player`
- strengthens the positive self-target path

## Ownership Check

This remains aggregate-owned spell targeting and resolution behavior.

## Documentation Impact

- current-state
- implemented slice doc

## Test Impact

- unit tests for successful self-target cast and resolution
- optional executable BDD

## Rules Reference

- 114
- 601.2c
- 608.2b

## Rules Support Statement

This slice exercises the positive self-target path for explicit `any player` spells. It does not imply support for broader player-target restrictions.
