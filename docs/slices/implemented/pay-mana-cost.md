# Slice 14 — Pay Mana Cost

## Rules Reference

- 202.1
- 202.1a

## Rules Support Statement

This slice implements mana cost payment per rules 202.1 and 202.1a. This implements basic mana cost deduction. Color mana, alternative costs, and cost adjustments are not implemented.

## Goal

Require mana payment for casting spells.

## Supported behavior

### Mana Cost
- Cards have a mana cost (u32)
- Player must have enough mana to cast a spell
- Mana is deducted from player's mana pool when spell is cast

### Domain Changes

- `CardInstance` gains `mana_cost: u32` field
- `PlayerDeckContents` now accepts `(CardDefinitionId, CardType, mana_cost)` tuples
- `Game::cast_spell()` checks mana availability before casting

### Errors

#### InsufficientMana
```rust
pub struct InsufficientMana {
    pub player_id: PlayerId,
    pub required: u32,
    pub available: u32,
}
```

Returned when player tries to cast a spell without enough mana.

## Tests

- CastSpellCommand fails with InsufficientMana when not enough mana
- CastSpellCommand succeeds and deducts mana when sufficient mana
- Mana cost of 0 allows free casting
