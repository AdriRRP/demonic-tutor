# Slice 8 — Player Life

## Goal

Add player life tracking to support future damage and win conditions.

## Supported behavior

### Player Life
- Players start with 20 life
- Life can be modified by `SetLifeCommand`
- Life change uses `i32`: positive values gain life, negative values lose life
- Life cannot go below 0 (saturating arithmetic)

### Commands

#### SetLifeCommand
```rust
pub struct SetLifeCommand {
    pub player_id: PlayerId,
    pub life_change: i32,
}
```

- `life_change > 0`: gain life
- `life_change < 0`: lose life
- Valid for any player in the game

### Events

#### LifeChanged
```rust
pub struct LifeChanged {
    pub game_id: GameId,
    pub player_id: PlayerId,
    pub from_life: u32,
    pub to_life: u32,
}
```

Emitted whenever a player's life total changes.

## Domain Changes

- `Player` struct gains `life: u32` field
- Default starting life: 20
- `Game::set_life()` method handles life modification
- Life uses `saturating_add_signed` to prevent underflow

## Rules Reference

- 118.1
- 118.2

## Rules Support Statement

This slice implements player life tracking per rules 118.1 and 118.2. This supports basic life total management. Damage, life gain, and life loss effects are not implemented.

## Tests

- Players start with 20 life
- SetLifeCommand with negative value decreases life
- SetLifeCommand with positive value increases life
- Life cannot go below 0
- SetLifeCommand fails for unknown player
- LifeChanged event is emitted correctly
