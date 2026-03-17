# Slice - Draw Priority Window

## Goal

Open an explicit empty priority window when the active player enters `Draw`, after the automatic turn draw has already happened.

## Supported Behavior

- advancing from `Upkeep` to `Draw` performs the automatic turn draw
- after that automatic draw, entering `Draw` opens a priority window for the active player
- two consecutive passes close an empty draw-step priority window
- turn advancement is rejected while the draw-step priority window remains open

## Explicit Limits

- this slice models only the post-draw empty priority window, not triggered abilities tied to drawing
- the runtime still uses a simplified phase model, not full beginning-phase substep timing
- end-step priority remains out of scope for this slice

## Domain Changes

- `advance_turn` now opens `PriorityState` when entering `Draw`
- the automatic turn draw still happens before the empty draw-step priority window is opened
- helper flows that advance turn state continue to close empty windows explicitly when they need to keep moving

## Rules Support Statement

This slice extends beginning-phase timing so the draw step is no longer modeled as only an automatic draw followed by an immediate jump to `FirstMain`. The active player now draws first, then receives an empty priority window in `Draw`, which keeps the current minimal stack model closer to real gameplay without yet modeling draw-step triggers.

## Tests

- advancing from `Upkeep` to `Draw` performs the automatic draw and opens priority for the active player
- two consecutive passes close an empty draw-step priority window
- unit and BDD setup helpers continue to close empty priority windows explicitly when advancing through setup states
