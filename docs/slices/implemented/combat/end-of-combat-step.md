# Slice — End Of Combat Step

## Goal

Make the transition out of `EndOfCombat` explicit: once the empty post-combat priority window closes, the game advances into `SecondMain`.

## Supported Behavior

- `EndOfCombat` is a real phase in the aggregate
- the phase opens an empty priority window for the active player
- after two consecutive passes with an empty stack, `advance_turn` moves the game into `SecondMain`
- no turn progression may skip directly from `EndOfCombat` to a later turn moment while the window remains open

## Explicit Limits

- this slice only formalizes progression out of `EndOfCombat`
- no end-of-combat triggered abilities are modeled
- only the currently supported minimal instant-speed stack behavior may occur in that window

## Domain Changes

- no new public command is introduced
- `advance_turn` now uses the explicit combat-subphase model to progress from `EndOfCombat` into `SecondMain`

## Rules Support Statement

This slice completes the explicit combat corridor. After both players pass through the empty end-of-combat window, the game now stands in `SecondMain`, ready for post-combat main-phase actions.

## Tests

- BDD coverage confirms that closing the empty `EndOfCombat` window advances to `SecondMain`
