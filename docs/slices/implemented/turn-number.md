# Slice 9 — Turn Number

## Goal

Track the turn count in the game.

## Supported behavior

### Turn Number
- Games start with `turn_number = 1`
- Turn number increments by 1 each time `advance_turn` is called
- `TurnProgressed` event includes the old and new turn number when turn advances

### Events

#### TurnProgressed
```rust
pub struct TurnProgressed {
    pub game_id: GameId,
    pub active_player: PlayerId,
    pub from_turn: u32,
    pub to_turn: u32,
    pub from_phase: Phase,
    pub to_phase: Phase,
}
```

Emitted whenever `advance_turn` progresses the game state.

## Domain Changes

- `Game` struct gains `turn_number: u32` field
- Default turn number: 1
- `Game::advance_turn()` now returns `(TurnProgressed, Option<CardDrawn>)`
- GameService publishes the composite turn event and optional draw event

## Rules Reference

This slice provides turn counting as a game state attribute. It does not correspond directly to a specific Magic Comprehensive Rules section.

## Rules Support Statement

This slice implements turn number tracking as a simple counter. The full turn structure rules are handled by the AdvanceTurn and Turn Phases slices.

## Tests

- Game starts with turn number 1
- advance_turn increments turn number to 2
- TurnProgressed event is emitted with correct values
