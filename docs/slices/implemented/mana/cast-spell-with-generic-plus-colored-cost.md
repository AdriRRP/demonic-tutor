# Slice — Cast Spell With Generic Plus Colored Cost

## Goal

Support spell costs that require both generic mana and one colored mana symbol.

## Supported behavior

- a spell face may now declare a mixed cost with one colored requirement plus generic mana
- casting succeeds when the mana pool satisfies both the colored requirement and the remaining generic amount
- colored mana may be consumed during that payment

## Current scope

This slice exercises a minimal mixed-cost corridor such as `1G`.

It does not yet add hybrid, alternative, or broader multicolor cost support.

## Rules reference

- 106
- 107.4
- 202
- 601.2f

## Rules support statement

DemonicTutor now supports a minimal mixed-cost mana model with one colored requirement plus generic mana, exercised through a real casting corridor in `FirstMain`.
