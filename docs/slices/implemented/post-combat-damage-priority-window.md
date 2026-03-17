# Slice — Post Combat Damage Priority Window

## Goal

Reopen an explicit priority window for the active player after combat damage resolves.

## Supported Behavior

- resolving combat damage reopens a priority window for the active player when the game remains active
- that post-damage priority window starts with an empty stack
- two consecutive passes may close the empty post-damage window
- `advance_turn` cannot move from `Combat` to `SecondMain` until the window is closed

## Explicit Limits

- the combat model still uses a single `Combat` phase rather than distinct damage and end-of-combat steps
- this slice does not yet model a separate end-of-combat step
- only the currently supported minimal stack semantics are available inside the post-damage window

## Domain Changes

- `Game::resolve_combat_damage()` now reopens `PriorityState` for the active player when combat damage finishes and the game is still active
- combat-oriented helpers must close the empty post-damage window explicitly before continuing out of `Combat`

## Rules Support Statement

This slice extends the current combat timing model with a post-damage priority window. After combat damage resolves, the game now reopens priority for the active player while remaining in `Combat`, which makes the transition toward `SecondMain` more semantically honest without yet introducing full combat-step modeling.

## Tests

- resolving combat damage reopens priority for the active player
- BDD coverage confirms the game remains in `Combat` with an empty stack and an open priority window after damage resolution
