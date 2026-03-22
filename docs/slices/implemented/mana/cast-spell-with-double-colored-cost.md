# Slice — Cast Spell With Double Colored Cost

## Goal

Support spell costs that require two mana of the same color.

## Supported behavior

- a spell face may now declare a repeated same-color cost such as `GG`
- casting succeeds only when the mana pool contains at least two mana of that color
- payment consumes the exact colored amount and leaves off-color mana untouched on failed casts

## Current scope

This slice exercises a minimal repeated-colored corridor for `GG`.

It does not yet add multicolor symbol combinations, hybrid costs, or alternative costs.

## Rules reference

- 106
- 107.4
- 202
- 601.2f

## Rules support statement

DemonicTutor now supports repeated same-color requirements in the minimal mana-cost model, exercised through a real `FirstMain` casting corridor and rejection tests for missing second-color symbols.
