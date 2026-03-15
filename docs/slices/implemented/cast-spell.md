# Slice 13 — Cast Non-Land Spells

## Goal

Enable casting non-land spells (creatures, instants, sorceries, etc.) from hand to battlefield.

## Supported behavior

### Card Types
- `CardType::Land` - lands
- `CardType::Creature` - creature cards
- `CardType::Instant` - instant spells
- `CardType::Sorcery` - sorcery spells
- `CardType::Enchantment` - enchantment cards
- `CardType::Artifact` - artifact cards
- `CardType::Planeswalker` - planeswalker cards

Helper methods:
- `CardType::is_land()` - returns true for Land type
- `CardType::is_non_land()` - returns true for all non-land types

### Commands

#### CastSpellCommand
```rust
pub struct CastSpellCommand {
    pub player_id: PlayerId,
    pub card_id: CardInstanceId,
}
```

- Card must be in the player's hand
- Card must NOT be a land (lands are played with PlayLandCommand)
- Player must be the active player

### Events

#### SpellCast
```rust
pub struct SpellCast {
    pub game_id: GameId,
    pub player_id: PlayerId,
    pub card_id: CardInstanceId,
}
```

Emitted when a spell is cast successfully.

## Domain Changes

- `CardType` enum expanded with specific types
- `Game::cast_spell()` method handles spell casting
- New error: `CannotCastLand` - when trying to cast a land as a spell

## Rules Reference

- 601.1
- 601.2

## Rules Support Statement

This slice implements a simplified spell-casting model in which non-land cards are moved from hand into play according to the current game model. The full casting process (targets, modes, stack, timing, alternative costs, and resolution rules) is not implemented.

## Tests

- CastSpellCommand moves card from hand to battlefield
- CastSpellCommand emits SpellCast event
- CastSpellCommand fails for land cards (CannotCastLand)
- CastSpellCommand fails when not player's turn (NotYourTurn)
- CastSpellCommand fails when card not in hand (CardNotInHand)
