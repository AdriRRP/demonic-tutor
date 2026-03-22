# Slice — Support Island Blue Mana

## Goal

Extend the current minimal colored mana model so `Island` produces `Blue`.

## Supported behavior

- an `Island` card face may now exist in setup
- tapping an `Island` adds one `Blue` mana to the acting player's mana pool
- the corresponding mana-added event now exposes `Blue` in the exercised corridor

## Current scope

This slice extends the currently exercised land-production subset to:

- `Forest -> Green`
- `Mountain -> Red`
- `Plains -> White`
- `Island -> Blue`

It does not add blue-specific spell behavior beyond mana production.

## Rules reference

- 106
- 107.4
- 305
- 605

## Rules support statement

DemonicTutor now extends the current minimal colored mana subset so `Island` produces `Blue` through the same stack-free land-mana corridor already used by the supported colored lands.
