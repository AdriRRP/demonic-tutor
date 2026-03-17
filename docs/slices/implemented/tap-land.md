# Slice 11 — Tap Lands for Mana

## Goal

Enable lands to produce mana for future spell casting.

## Supported behavior

### Mana System
- Players have a mana pool (starts at 0)
- Lands can be tapped to produce 1 mana each
- Tapped lands cannot be tapped again until untapped
- Mana production is currently limited to the active player's `FirstMain` and `SecondMain`
- Mana pools are cleared when the game advances to the next phase or turn

### Commands

#### TapLandCommand
```rust
pub struct TapLandCommand {
    pub card_id: CardInstanceId,
}
```

- Card must be on the battlefield
- Card must be a Land type
- Card must not already be tapped
- Player must be the active player
- Current phase must be `FirstMain` or `SecondMain`
- Produces 1 generic mana for the active player

### Events

#### LandTapped
```rust
pub struct LandTapped {
    pub game_id: GameId,
    pub card_id: CardInstanceId,
}
```

#### ManaAdded
```rust
pub struct ManaAdded {
    pub game_id: GameId,
    pub player_id: PlayerId,
    pub amount: u32,
}
```

## Domain Changes

- `CardInstance` struct gains `tapped: bool` field
- Default tapped state: false
- `CardInstance::tap()` and `untap()` methods
- `Player` struct gains `mana: u32` field
- Default mana: 0
- `Game::tap_land()` method handles mana production
- Validation: CardNotOnBattlefield, CardAlreadyTapped, CardNotLand

## Rules Reference

- 605.1
- 605.3a

## Rules Support Statement

This slice implements mana production from lands per rules 605.1 and 605.3a. This implements basic mana production. Mana types, color identity, priority timing, and mana burn are not implemented.

## Tests

- TapLandCommand with unknown card fails
- TapLandCommand with non-land card fails
- TapLandCommand with already tapped land fails
- TapLandCommand adds 1 mana to player
- LandTapped and ManaAdded events are emitted correctly
- Players start with 0 mana
