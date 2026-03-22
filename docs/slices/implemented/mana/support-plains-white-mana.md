# Slice — Support Plains White Mana

## Goal

Extend the current minimal colored mana model so `Plains` produces `White`.

## Supported behavior

- a `Plains` card face may now exist in setup
- tapping a `Plains` adds one `White` mana to the acting player's mana pool
- the corresponding mana-added event now exposes `White` in the exercised corridor

## Current scope

This slice only extends the existing land-production subset:

- `Forest -> Green`
- `Mountain -> Red`
- `Plains -> White`

It does not add white-specific spell behavior beyond mana production.

## Rules reference

- 106
- 107.4
- 305
- 605

## Rules support statement

DemonicTutor now extends the current minimal colored mana subset so `Plains` produces `White` through the same stack-free land-mana corridor already used by the supported colored lands.
