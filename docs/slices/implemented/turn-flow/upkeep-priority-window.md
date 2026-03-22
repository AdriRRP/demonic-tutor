# Slice - Upkeep Priority Window

## Goal

Open an explicit empty priority window when the active player enters `Upkeep`.

## Supported Behavior

- advancing from `Untap` to `Upkeep` opens a priority window for the active player
- two consecutive passes close an empty upkeep priority window
- turn advancement is rejected while the upkeep priority window remains open
- if the stack is non-empty, unrelated gameplay actions are still rejected until the stack resolves or the window closes

## Explicit Limits

- this slice models only the empty upkeep window, not triggered abilities or upkeep-specific effects
- the runtime still uses the simplified phase model without separate upkeep-driven stack triggers
- draw-step and end-step priority windows remain out of scope for this slice

## Domain Changes

- `advance_turn` now opens `PriorityState` when entering `Upkeep`
- `PassPriority` can close an empty upkeep window without resolving a stack object
- helper flows that advance turn state continue to close empty windows explicitly when they need to keep moving

## Rules Support Statement

This slice extends turn-flow timing so the beginning phase no longer jumps straight from `Untap` to later interactive windows. Entering `Upkeep` now opens an empty priority window for the active player, which keeps the current minimal stack model more faithful to Magic timing without yet modeling upkeep triggers.

## Tests

- advancing from `Untap` to `Upkeep` opens priority for the active player
- two consecutive passes close an empty upkeep priority window
- unit and BDD setup helpers continue to close empty priority windows explicitly when advancing through setup states
