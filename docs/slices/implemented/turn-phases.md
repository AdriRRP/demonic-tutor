# Slice 10 — Turn Phases

## Status

Superseded by [`full-turn-phases.md`](./full-turn-phases.md).

## Note

This document is kept only as historical context for the earlier reduced phase model.

The current implementation uses the full turn-phase slice documented in [`full-turn-phases.md`](./full-turn-phases.md), including:

- `Setup`, `Untap`, `Upkeep`, `Draw`, `FirstMain`, `Combat`, `SecondMain`, `EndStep`
- composite `TurnProgressed`
- optional automatic `CardDrawn` during turn progression
