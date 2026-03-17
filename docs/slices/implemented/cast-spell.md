# Slice 13 — Cast Spells

## Goal

Enable casting spell cards from hand with minimal stack-aware resolution.

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
- `CardType::is_spell_card()` - returns true for all spell card types

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
- Outside an open priority window, casting remains limited to the active player in `FirstMain` or `SecondMain`
- During an open priority window, the current holder may cast only instant spells in the current minimal stack model

#### PassPriorityCommand
```rust
pub struct PassPriorityCommand {
    pub player_id: PlayerId,
}
```

- Used to advance the currently open minimal priority loop
- Two consecutive passes resolve the top stack object

### Events

#### SpellPutOnStack
```rust
pub struct SpellPutOnStack {
    pub game_id: GameId,
    pub player_id: PlayerId,
    pub card_id: CardInstanceId,
    pub card_type: CardType,
    pub mana_cost_paid: u32,
    pub stack_object_id: StackObjectId,
}
```

Emitted when casting successfully moves a spell card from hand onto the stack and opens the current minimal priority loop.

The casting player keeps priority immediately after `SpellPutOnStack`. The opposing player may respond only after that first pass.

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

Emitted when the top spell on the stack resolves successfully, including the spell card type, the mana cost paid, and whether it entered the battlefield or resolved to the graveyard in the simplified model.

`PassPriority` may now produce `SpellCast`, `CreatureDied`, or `GameEnded` when resolution and the shared review of currently supported state-based actions produce additional automatic consequences.

## Domain Changes

- `CardType` enum expanded with specific types
- `Game::cast_spell()` now puts a spell card on the stack
- `Game::pass_priority()` advances the current minimal priority loop
- supported state-based actions are reviewed after spell resolution
- New error: `CannotCastLand` - when trying to cast a land as a spell

## Rules Reference

- 601.1
- 601.2

## Rules Support Statement

This slice now implements a minimal stack-aware spell-casting model. Casting moves a spell card from hand onto the stack, and the casting player keeps priority immediately afterward. Resolution happens only after two consecutive passes. Permanent spells resolve from the stack to the battlefield, while instants and sorceries resolve from the stack to the graveyard. The current runtime also triggers the shared review of currently supported state-based actions after spell resolution, which can produce `CreatureDied` or `GameEnded` in addition to `SpellCast`. The current priority-holder may now cast instant response spells, but broader timing rules, non-instant responses, targets, and modes remain out of scope.

## Tests

- CastSpellCommand moves spell cards from hand to stack
- PassPriorityCommand resolves the top spell after two consecutive passes
- SpellPutOnStack is emitted when casting succeeds
- SpellCast is emitted when the spell resolves
- Zero-toughness creature spells die immediately after entering the battlefield
- CastSpellCommand fails for land cards (CannotCastLand)
- CastSpellCommand fails when not player's turn (NotYourTurn)
- CastSpellCommand fails when card not in hand (CardNotInHand)
