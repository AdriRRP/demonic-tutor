# Slice 8 — Player Life

## Goal

Add player life tracking and a minimal explicit life-effect entrypoint to support gameplay semantics that depend on life totals.

## Supported behavior

### Player Life
- Players start with 20 life
- Life can be modified by `AdjustPlayerLifeEffectCommand`
- Life change uses `i32`: positive values gain life, negative values lose life
- Life cannot go below 0 (saturating arithmetic)
- Reaching 0 life ends the game through the separate `LoseOnZeroLife` slice

### Commands

#### AdjustPlayerLifeEffectCommand
```rust
pub struct AdjustPlayerLifeEffectCommand {
    pub caster_id: PlayerId,
    pub target_player_id: PlayerId,
    pub life_delta: i32,
}
```

- `life_delta > 0`: gain life
- `life_delta < 0`: lose life
- `caster_id` and `target_player_id` must both reference players in the game
- the effect changes the target player's life total

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

#### AdjustPlayerLifeEffectOutcome

The current runtime returns an `AdjustPlayerLifeEffectOutcome` containing:

- the required `LifeChanged` event
- zero or more `CreatureDied` events if a supported state-based action was pending
- an optional `GameEnded` if the player's life reached 0

## Domain Changes

- `Player` struct gains `life: u32` field
- Default starting life: 20
- `Game::adjust_player_life_effect()` handles explicit life-effect resolution
- Life uses `saturating_add_signed` to prevent underflow

## Rules Reference

- 118.1
- 118.2

## Rules Support Statement

This slice implements player life tracking per rules 118.1 and 118.2. This supports basic life total management plus a minimal explicit targeted life-effect entrypoint. Life totals now also participate in terminal game behavior through the separate `LoseOnZeroLife` slice. Damage, life gain, and life loss effects beyond explicit adjustment are not implemented.

## Tests

- Players start with 20 life
- AdjustPlayerLifeEffectCommand with negative value decreases the target player's life
- AdjustPlayerLifeEffectCommand with positive value increases the target player's life
- Life cannot go below 0
- Reaching 0 life ends the game
- Supported pending creature deaths may also be resolved through shared state-based action review
- AdjustPlayerLifeEffectCommand fails for unknown caster or unknown target
- LifeChanged event is emitted correctly
