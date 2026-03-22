# Slice — Beginning Of Combat Step

## Goal

Make the transition out of `BeginningOfCombat` explicit: once the empty priority window closes, the game advances into `DeclareAttackers`.

## Supported Behavior

- `BeginningOfCombat` is a real phase in the aggregate
- the phase opens an empty priority window for the active player
- after two consecutive passes with an empty stack, `advance_turn` moves the game into `DeclareAttackers`
- no combat action may skip directly from `BeginningOfCombat` to a later combat moment

## Explicit Limits

- this slice only formalizes progression out of `BeginningOfCombat`
- no beginning-of-combat triggered abilities are modeled
- only the currently supported minimal instant-speed stack behavior may occur in that window

## Domain Changes

- no new public command is introduced
- `advance_turn` now uses the explicit combat-subphase model to progress from `BeginningOfCombat` into `DeclareAttackers`

## Rules Support Statement

This slice closes the gap between “combat has an entry window” and “combat has a first action step”. After the active player and opponent pass through the empty beginning-of-combat window, the game now stands in `DeclareAttackers`, ready for the next combat action.

## Tests

- BDD coverage confirms that closing the empty beginning-of-combat window advances to `DeclareAttackers`
