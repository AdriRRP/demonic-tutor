# Slice Proposal — Reject Mixed Cost Without Required Color

## Goal

Reject a spell cast when the mana pool can cover the total amount but not the specific required color symbols.

## Why This Slice Exists Now

Once mixed costs exist, the negative case is just as important as the positive one. This slice prevents the payment corridor from overstating support through total-only affordability.

## Supported Behavior

- a spell with a mixed cost is rejected when the pool lacks the required colored symbol
- the same spell remains castable when the exact color is present

## Invariants / Legality Rules

- total mana alone is insufficient when a required color is missing
- failed payment leaves the mana pool untouched
- the spell remains in hand after rejection

## Out of Scope

- multiple different required colors
- cost reductions or replacements

## Domain Impact

- strengthen payment validation and rejection semantics
- ensure error reporting stays tied to insufficient mana legality

## Ownership Check

This is aggregate-owned casting legality.

## Documentation Impact

- `docs/domain/current-state.md`
- the implemented slice doc for this capability

## Test Impact

- unit and BDD coverage for a mixed-cost rejection path

## Rules Reference

- 106
- 202
- 601.2f

## Rules Support Statement

This slice adds the required negative case for the current mixed-cost mana model. It does not imply broad symbol parsing beyond the supported subset.
