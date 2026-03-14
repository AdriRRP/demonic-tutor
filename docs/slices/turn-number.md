# Slice 9 — Turn Number

## Goal

Track the turn count in the game.

## Supported behavior

### Turn Number
- Games start with `turn_number = 1`
- Turn number increments by 1 each time `advance_turn` is called
- `TurnNumberChanged` event is emitted when turn advances

### Events

#### TurnNumberChanged
```rust
pub struct TurnNumberChanged {
    pub game_id: GameId,
    pub from_turn: u32,
    pub to_turn: u32,
}
```

Emitted whenever the turn number changes.

## Domain Changes

- `Game` struct gains `turn_number: u32` field
- Default turn number: 1
- `Game::advance_turn()` now returns `(TurnAdvanced, TurnNumberChanged)`
- GameService publishes both events to EventStore and EventBus

## Tests

- Game starts with turn number 1
- advance_turn increments turn number to 2
- TurnNumberChanged event is emitted with correct values
