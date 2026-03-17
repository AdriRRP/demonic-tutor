# Slice 13 — Cast Spells

## Goal

Enable casting non-land spells from hand with simplified resolution.

## Supported behavior

### Supported Card Types
- `CardType::Creature` - creature spells
- `CardType::Instant` - instant spells
- `CardType::Sorcery` - sorcery spells
- `CardType::Enchantment` - enchantment spells
- `CardType::Artifact` - artifact spells
- `CardType::Planeswalker` - planeswalker spells

### Rejected Card Types
- `CardType::Land` - lands are played with `PlayLandCommand`

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
- Card must not be a land
- Player must be the active player

### Events

#### SpellCast
```rust
pub struct SpellCast {
    pub game_id: GameId,
    pub player_id: PlayerId,
    pub card_id: CardInstanceId,
    pub card_type: CardType,
    pub mana_cost_paid: u32,
    pub outcome: SpellCastOutcome,
}
```

Emitted when a spell is cast successfully, including the spell card type, the mana cost paid, and whether it entered the battlefield or resolved to the graveyard in the simplified model.

`CastSpell` now returns a runtime outcome that may also include `CreatureDied` events when a creature with 0 toughness immediately dies after entering the battlefield under the repository's current narrow state-based check.

## Domain Changes

- `CardType` enum expanded with specific types
- `Game::cast_spell()` handles spell casting
- zero-toughness creature checks run immediately after creature-spell resolution
- New error: `CannotCastLand` - when trying to cast a land as a spell

## Rules Reference

- 601.1
- 601.2

## Rules Support Statement

This slice implements a simplified spell-casting model. Permanent non-land spells enter the battlefield, while instants and sorceries resolve directly to the graveyard. The current runtime also performs a narrow automatic check for creatures with 0 toughness after creature-spell resolution, moving them to the graveyard with `CreatureDied`. The full casting process (targets, modes, stack, timing, alternative costs, and resolution rules) is not implemented.

## Tests

- CastSpellCommand moves permanent spells from hand to battlefield
- CastSpellCommand moves instants and sorceries from hand to graveyard
- CastSpellCommand emits SpellCast event
- Zero-toughness creature spells die immediately after entering the battlefield
- CastSpellCommand fails for land cards (CannotCastLand)
- CastSpellCommand fails when not player's turn (NotYourTurn)
- CastSpellCommand fails when card not in hand (CardNotInHand)
