# Slice — Beginning Of Combat Priority Window

## Goal

Open an explicit empty priority window when the active player enters `BeginningOfCombat` from `FirstMain`.

## Supported Behavior

- advancing from `FirstMain` to `BeginningOfCombat` opens a priority window for the active player
- that window starts empty
- two consecutive passes may close the empty combat-entry window
- declaring attackers cannot happen until the window is closed

## Explicit Limits

- this slice now sits on top of the explicit combat-subphase foundation
- this slice models only the empty beginning-of-combat window, not richer triggered timing
- only the currently supported minimal stack semantics are available inside the window

## Domain Changes

- `advance_turn` now opens `PriorityState` when entering `BeginningOfCombat`
- combat setup helpers must close the empty combat-entry window explicitly before declaring attackers

## Rules Support Statement

This slice adds the first pre-attack combat timing window to the runtime model. Entering `BeginningOfCombat` now opens an empty priority window for the active player, making the transition from `FirstMain` into combat more semantically honest.

## Tests

- advancing from `FirstMain` to `BeginningOfCombat` opens priority for the active player
- BDD coverage confirms the game remains in `BeginningOfCombat` with an empty stack and an open priority window
