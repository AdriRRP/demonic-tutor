# Terminology: Spell vs Non-Land Spell

## Summary

Fix incorrect terminology: "non-land spell" is redundant in Magic. A spell is any card on the stack that is not a land. Using "non-land spell" implies some spells could be lands, which is incorrect.

## Motivation

Magic terminology precision matters for maintaining ubiquitous language consistency.

In Magic:
- A **spell** is a card on the stack
- A **land** is a card type, not a spell
- Therefore, "non-land spell" is redundant — simply "spell" is correct

The code already uses correct naming (`CastSpellCommand`), but comments and documentation used the redundant phrase.

## Scope

### Code Changes
- `src/domain/game.rs` — Fix doc comment
- `src/application/game_service.rs` — Fix doc comment
- `docs/architecture/game-aggregate-structure.md` — Fix comment

### Documentation Changes
- `docs/slices/implemented/cast-spell.md` — Fix terminology

### No Changes Needed
- `CardType::is_non_land()` — Method name is correct (boolean comparison)

## Verification

Search for "non-land spell" returns no results after this slice.

## Notes

This slice maintains terminology consistency without changing behavior.
