# Mark Opening-Hand Bottom Picks Directly On Cards

## Goal

Make bottom-card selection feel tactile by annotating the chosen opening-hand cards directly in the visible hand fan with ordered seals.

## Why This Slice Existed Now

The browser already supported London-style bottom selection from the visible hand fan, but the overlay still carried too much of the meaning. The next smallest improvement was to let the hand itself communicate the chosen bottom set and its selection order, so players can read the pregame state from the cards instead of a side summary.

## Supported Behavior

- opening-hand cards selected for bottoming now show numbered markers directly on the cards
- the markers reflect selection order inside the hand fan
- the overlay summary becomes lighter and treats the hand as the primary source of truth
- markers only appear on the local visible hand during active bottom selection

## Out Of Scope

- true bottom-order rules beyond the repository's current selected-set semantics
- drag reordering during pregame
- changes to mulligan legality

## Rules Support Statement

This slice does not add new Magic rules.

It only improves how the browser presents already-supported bottom-card selection during `Setup`.

