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
- `src/domain/play/game/mod.rs` — Fix doc comment
- `src/application/game_service/mod.rs` — Fix doc comment
- `docs/architecture/game-aggregate-structure.md` — Fix comment

### Documentation Changes
- `docs/slices/implemented/stack/cast-spell.md` — Fix terminology

### Additional Cleanup
- `CardType::is_spell_card()` now expresses the same distinction in positive domain language

## Verification

Search for `non-land spell`, `nonland spell`, and `is_non_land` returns no live usage outside historical explanation.

## Notes

This slice maintains terminology consistency without changing behavior.
