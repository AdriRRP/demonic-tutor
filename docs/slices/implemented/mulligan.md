# Slice 6 — Mulligan

## Goal

Allow a player to return their hand to their library, shuffle it, and draw a new 7-card hand before the game begins.

## Why it matters

Mulligan is a fundamental Magic mechanic. Implementing it now establishes the `Phase::Setup` pattern and validates that the library shuffle capability works correctly.

## Expected Behavior

The system must be able to:
- receive a `MulliganCommand`
- verify the player is in the game
- verify the player has not previously used mulligan
- verify the phase is `Phase::Setup`
- verify the library contains at least 7 cards
- move all cards from hand back to library
- shuffle the library
- draw 7 new cards
- emit a `MulliganTaken` event

## Flow

| Phase | Allowed Actions |
|-------|----------------|
| `Phase::Setup` | `MulliganCommand`, `DealOpeningHandsCommand` |
| `Phase::Main` | `PlayLandCommand`, `DrawCardCommand`, `AdvanceTurnCommand` |

Mulligan is only valid during `Phase::Setup`.

This slice does not finalize the setup flow. The exact transition from `Setup` to `Main` is out of scope for this slice.

## Simplification

This slice models a simplified one-time mulligan to 7 cards.

## Rules Reference

- 103.4

## Rules Support Statement

This slice implements the basic mulligan mechanic per rule 103.4, allowing a player to shuffle their hand back into the library and draw a new seven-card hand. This implements a simplified one-time mulligan to exactly seven cards.

## Out of Scope

- Scry
- Partial mulligan
- Multiple mulligans
- Opponent responses
- Post-game mulligans

## Tests

| Test | Description |
|------|-------------|
| `mulligan_succeeds` | Valid player performs mulligan in Setup phase |
| `mulligan_fails_already_used` | Cannot mulligan twice |
| `mulligan_fails_not_enough_cards` | Fails if library has fewer than 7 cards |
| `mulligan_fails_not_setup_phase` | Fails if game is not in Setup phase |
