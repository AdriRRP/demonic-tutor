# Slice — Beginning Of Combat Priority Window

## Goal

Open an explicit empty priority window when the active player enters `Combat` from `FirstMain`.

## Supported Behavior

- advancing from `FirstMain` to `Combat` opens a priority window for the active player
- that window starts empty
- two consecutive passes may close the empty combat-entry window
- declaring attackers cannot happen until the window is closed

## Explicit Limits

- the combat model still uses a single `Combat` phase rather than distinct begin-combat and declare-attackers steps
- this slice models only the empty priority window at combat entry, not a full combat-step engine
- only the currently supported minimal stack semantics are available inside the window

## Domain Changes

- `advance_turn` now opens `PriorityState` when entering `Combat`
- combat setup helpers must close the empty combat-entry window explicitly before declaring attackers

## Rules Support Statement

This slice adds the first pre-attack combat timing window to the runtime model. Entering `Combat` now opens an empty priority window for the active player, making the transition from `FirstMain` into combat more semantically honest without yet splitting the combat phase into full Magic substeps.

## Tests

- advancing from `FirstMain` to `Combat` opens priority for the active player
- BDD coverage confirms the game remains in `Combat` with an empty stack and an open priority window
