# Slice — Support Swamp Black Mana

## Goal

Extend the current minimal colored mana model so `Swamp` produces `Black`.

## Supported behavior

- a `Swamp` card face may now exist in setup
- tapping a `Swamp` adds one `Black` mana to the acting player's mana pool
- the corresponding mana-added event now exposes `Black` in the exercised corridor

## Current scope

This slice extends the currently exercised land-production subset to:

- `Forest -> Green`
- `Mountain -> Red`
- `Plains -> White`
- `Island -> Blue`
- `Swamp -> Black`

It does not add black-specific spell behavior beyond mana production.

## Rules reference

- 106
- 107.4
- 305
- 605

## Rules support statement

DemonicTutor now extends the current minimal colored mana subset so `Swamp` produces `Black` through the same stack-free land-mana corridor already used by the supported colored lands.
