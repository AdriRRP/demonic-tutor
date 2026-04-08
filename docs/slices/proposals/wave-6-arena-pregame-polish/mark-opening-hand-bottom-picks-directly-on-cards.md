# Mark Opening-Hand Bottom Picks Directly On Cards

## Goal

Make bottom-card selection feel more tactile by giving selected opening-hand cards explicit ordered markers directly in the fan.

## Why This Slice Exists Now

The current hand-fan selection flow is already much better than the old auxiliary picker, but the user still has to infer some of the state from the overlay summary. The next smallest improvement is to annotate the chosen cards directly so the hand itself carries the full bottom-selection meaning.

## Supported Behavior

- opening-hand cards selected for bottoming show a direct visual marker on the card
- the marker reflects ordered selection, so players can read their chosen bottom set from the hand alone
- the overlay summary remains lightweight and secondary

## Invariants / Legality Rules

- markers only appear during opening-hand bottom selection
- the number of marked cards never exceeds the required bottom count
- keeping remains blocked until the required number has been selected

## Out Of Scope

- bottom-order rules beyond the repository's current simplified “selected set” semantics
- drag reordering during pregame
- any change to mulligan legality

## Domain Impact

### Aggregate Impact

- none

## Ownership Check

This behavior belongs to the browser hand presentation and pregame overlay.

It is a UX improvement on top of already-supported bottom selection.

## Documentation Impact

- `docs/architecture/web-client.md`
- `apps/web/README.md`
- this slice document

## Test Impact

- selected bottom cards show markers in the hand fan
- markers clear when setup state changes or selection changes
- waiting players do not receive or render those markers for the opposing hand

## Rules Reference

- 103.5 — Opening hand decisions

## Rules Support Statement

This slice does not add new Magic rules support.

It only improves how the browser presents already-supported opening-hand bottom selection.
