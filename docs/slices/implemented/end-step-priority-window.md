# Slice - EndStep Priority Window

## Goal

Open an explicit empty priority window when the active player enters `EndStep`.

## Supported Behavior

- advancing from `SecondMain` to `EndStep` opens a priority window for the active player
- two consecutive passes close an empty end-step priority window
- turn advancement out of `EndStep` is rejected while that end-step priority window remains open
- cleanup discard, when required, still happens in `EndStep` after the priority window has been closed

## Explicit Limits

- this slice models only the empty end-step priority window, not triggered end-step abilities
- the runtime still does not expose a distinct cleanup-step phase or repeated cleanup loops
- the end-step priority window is empty unless previous stack-aware actions put objects on the stack

## Domain Changes

- `advance_turn` now opens `PriorityState` when entering `EndStep`
- end-of-turn cleanup remains part of `EndStep`, but it now happens after the empty end-step window has been passed away
- helper flows that advance turn state continue to close empty windows explicitly when they need to keep moving

## Rules Support Statement

This slice makes the ending phase more semantically honest. Entering `EndStep` now opens an empty priority window for the active player before the game can finish end-of-turn cleanup and pass to the next player's `Untap`, which better matches real timing without yet introducing full end-step triggers or a separate cleanup-step model.

## Tests

- advancing from `SecondMain` to `EndStep` opens priority for the active player
- two consecutive passes close an empty end-step priority window
- cleanup discard and turn-advance helpers continue to close empty windows explicitly before moving out of `EndStep`
