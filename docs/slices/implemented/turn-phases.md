# Slice 10 — Turn Phases

## Goal

Add proper turn phase structure to enable future game actions.

## Supported behavior

### Phase Enum
```rust
pub enum Phase {
    Setup,    // Initial game setup
    Beginning, // Turn beginning (draw, untap - future)
    Main,     // Main phase - can play lands and spells
    Ending,   // End of turn cleanup
}
```

### Phase Transitions
- `Setup` → `Main` (advance_turn changes player)
- `Main` → `Ending`
- `Ending` → `Main` (next player's turn)

### AdvanceTurn Behavior
- First call: `Setup` → `Main` (changes to other player)
- Second call: `Main` → `Ending` (same player)
- Third call: `Ending` → `Main` (changes to other player, increments turn number)
- Lands played reset at start of each player's turn

### Phase Validation
- PlayLand: requires `Main` phase
- DrawCard: allowed in `Main`, `Beginning`, or `Setup` phases

### Events
- `PhaseChanged { game_id, from_phase, to_phase }`

## Domain Changes

- `Phase` enum expanded with `Main` and `Ending`
- `Game::advance_turn()` now returns 3 events: `TurnAdvanced`, `TurnNumberChanged`, `PhaseChanged`

## Rules Reference

- 501
- 502
- 503
- 504
- 505

## Rules Support Statement

This slice implements turn phase structure per rules 501-505. The model implements a simplified phase structure (Setup, Main, Ending) that maps loosely to CR 501-505. Specific steps like Draw, Untap, Begin Combat, Declare Blockers are not implemented.

## Tests

- advance_turn changes active player from Setup → Main
- advance_turn changes phase from Main → Ending
- advance_turn changes player and increments turn from Ending → Main
- PhaseChanged event is emitted correctly
- PlayLand only works in Main phase
- Lands played reset at start of new turn
