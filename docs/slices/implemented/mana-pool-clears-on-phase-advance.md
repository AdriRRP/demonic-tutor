# Slice — Mana Pool Clears On Phase Advance

## Goal

Make the current transient mana model executable when the game moves from one phase to the next.

## Supported behavior

- players may accumulate generic mana in their transient mana pool during the current phase
- when the game advances to the next phase, all mana pools are cleared
- this behavior is now exercised explicitly from `Upkeep` into `Draw`

## Current scope

This slice does not add colored mana, mana burn, or richer floating-mana timing. It only proves the current generic transient mana model already implemented in the aggregate.

## Rules reference

- 106.4
- 500.4

## Rules support statement

DemonicTutor currently models mana as a transient pool of generic mana that clears when the game advances to the next phase or turn.
