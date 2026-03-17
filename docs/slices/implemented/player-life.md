# Slice 8 — Player Life

## Goal

Add player life tracking to support gameplay semantics that depend on life totals.

## Supported behavior

### Player Life
- Players start with 20 life
- Life can be modified by `AdjustLifeCommand`
- Life change uses `i32`: positive values gain life, negative values lose life
- Life cannot go below 0 (saturating arithmetic)
- Reaching 0 life ends the game through the separate `LoseOnZeroLife` slice

### Commands

#### AdjustLifeCommand
```rust
pub struct AdjustLifeCommand {
    pub player_id: PlayerId,
    pub life_delta: i32,
}
```

- `life_delta > 0`: gain life
- `life_delta < 0`: lose life
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

#### AdjustLifeOutcome

The current runtime returns an `AdjustLifeOutcome` containing:

- the required `LifeChanged` event
- zero or more `CreatureDied` events if a supported state-based action was pending
- an optional `GameEnded` if the player's life reached 0

## Domain Changes

- `Player` struct gains `life: u32` field
- Default starting life: 20
- `Game::adjust_life()` method handles life modification
- Life uses `saturating_add_signed` to prevent underflow

## Rules Reference

- 118.1
- 118.2

## Rules Support Statement

This slice implements player life tracking per rules 118.1 and 118.2. This supports basic life total management. Life totals now also participate in terminal game behavior through the separate `LoseOnZeroLife` slice. Damage, life gain, and life loss effects beyond explicit adjustment are not implemented.

## Tests

- Players start with 20 life
- AdjustLifeCommand with negative value decreases life
- AdjustLifeCommand with positive value increases life
- Life cannot go below 0
- Reaching 0 life ends the game
- Supported pending creature deaths may also be resolved through shared state-based action review
- AdjustLifeCommand fails for unknown player
- LifeChanged event is emitted correctly
