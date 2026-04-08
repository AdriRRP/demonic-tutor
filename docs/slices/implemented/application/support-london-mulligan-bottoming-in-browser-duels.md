# Support London Mulligan Bottoming In Browser Duels

## Goal

Make browser duels honor the currently intended London mulligan semantics by allowing repeated seven-card redraws during `Setup` and requiring the chooser to put the correct number of opening-hand cards on the bottom before keeping.

## Why This Slice Existed Now

The browser setup flow already kept remote duels in `Setup` and exposed keep-or-mulligan decisions, but the underlying behavior still matched an older simplified subset: one redraw and no explicit bottoming choice. The next smallest truthful step was to let the aggregate track repeated mulligans, expose the required bottom count through the wasm adapter, and make the browser pregame overlay collect an explicit bottom selection before `Keep`.

## Supported Behavior

- players may now mulligan repeatedly during `Setup`
- each mulligan redraws a fresh seven-card opening hand
- when a player chooses to keep after one or more mulligans, the aggregate requires exactly that many cards from the opening hand to be put on the bottom of the library
- the wasm/browser contract exposes both the player's mulligan count and the current required bottom count for the active chooser
- the pregame overlay now blocks `Keep` until the chooser has selected exactly the required number of cards to bottom
- the resulting kept hand size becomes `7 - mulligan_count`

## Out Of Scope

- simultaneous Arena-style reveal and choice choreography between both devices
- integrating bottom selection directly into the hand fan itself
- free mulligan variants or format-specific mulligan exceptions
- a domain-level canonical `keep opening hand` command

## Rules Support Statement

This slice widens the supported opening-hand rules from the earlier simplified redraw to London-style mulligan bottoming for the current two-player browser duel flow.

It does not yet claim full production parity with Arena's presentation choreography.
