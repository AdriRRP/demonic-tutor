# Slice Proposal — Vigilance Creatures Do Not Tap To Attack

## Goal

Support `Vigilance` so a creature does not tap when declared as an attacker.

## Why This Slice Exists Now

The repo already has attack declaration and combat damage. `Vigilance` is a straightforward keyword that affects visible combat state without requiring a new rules engine.

## Supported Behavior

- a creature with `Vigilance` may attack without becoming tapped

## Invariants / Legality Rules

- attack legality remains unchanged apart from the tapping outcome
- non-vigilance attackers still tap when declared

## Out of Scope

- vigilance interactions with activated abilities
- temporary granting of vigilance

## Domain Impact

- extend the closed keyword set with `Vigilance`
- refine attack-declaration state transitions

## Ownership Check

This belongs to aggregate-owned combat state and keyword legality.

## Documentation Impact

- current-state
- glossary
- implemented slice doc

## Test Impact

- unit and BDD coverage for tapping outcome on attack

## Rules Reference

- 508
- 702.20

## Rules Support Statement

This slice adds `Vigilance` only for the current attack-declaration model.
