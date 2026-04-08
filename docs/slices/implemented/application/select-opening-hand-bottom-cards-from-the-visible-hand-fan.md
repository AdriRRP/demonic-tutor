# Select Opening-Hand Bottom Cards From The Visible Hand Fan

## Goal

Remove the browser-only auxiliary card picker from London mulligan bottoming so players choose bottom cards directly from the visible opening-hand fan.

## Why This Slice Existed Now

The first truthful London mulligan slice made the browser semantically correct, but it still asked players to select bottom cards from a secondary overlay grid. That was functional but not faithful to the card-first interaction model already established in the arena. The next smallest valuable step was to keep the overlay as guidance while moving the actual selection gesture onto the visible hand itself.

## Supported Behavior

- when the local player must bottom opening-hand cards, the pregame overlay now instructs them to click cards directly in the visible hand fan
- selected bottom cards are highlighted in the hand instead of being chosen from a duplicate picker
- the pregame overlay now only summarizes the current bottom selection and keep/mulligan state
- dragging from hand is temporarily disabled during opening-hand bottom selection so the gesture remains unambiguous
- keeping the hand remains blocked until the required number of bottom cards has been selected

## Out Of Scope

- new mulligan rules beyond the existing London-style redraw-plus-bottoming support
- Arena-perfect pregame motion or animation polish
- recommendations about which cards to keep or bottom

## Rules Support Statement

This slice does not add new Magic rules.

It only changes the browser interaction model for the already-supported London mulligan bottoming flow.
