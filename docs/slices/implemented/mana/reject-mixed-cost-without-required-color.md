# Slice — Reject Mixed Cost Without Required Color

## Goal

Reject a spell cast when the mana pool can cover the total amount but not the specific required color symbols.

## Supported behavior

- a spell with a mixed cost is rejected when the pool lacks the required colored symbol
- failed payment leaves the mana pool untouched
- the spell remains in hand after rejection

## Current scope

This slice exercises the negative case for a minimal mixed cost such as `1G`.

It does not yet add broader multicolor combinations or replacement-cost behavior.

## Rules reference

- 106
- 202
- 601.2f

## Rules support statement

DemonicTutor now proves the negative legality corridor for the current mixed-cost mana model: total mana alone is insufficient when the required color symbol is missing.
