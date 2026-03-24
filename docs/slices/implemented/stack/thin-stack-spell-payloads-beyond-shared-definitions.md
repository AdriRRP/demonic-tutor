# Implemented Slice — Thin Stack Spell Payloads Beyond Shared Definitions

## Summary

Replace heavyweight stack spell carriers that kept `Arc<CardDefinition>` by compact spell-definition payloads tailored to runtime reconstruction and spell-rule lookup.

## Supported Behavior

- spells on the stack still expose stable public card ids, card type, and supported spell rules
- spell resolution still reconstructs truthful permanents or graveyard cards
- permanent cards that resolve from the stack keep their supported activated abilities and spell metadata

## Invariants

- stack spell behavior remains unchanged
- outward card ids remain stable and deterministic
- this slice does not expand supported Magic rules

## Implementation Notes

- `SpellPayload` now carries a compact copied definition profile instead of an `Arc<CardDefinition>`
- reconstruction of `CardInstance` uses that profile only when a resolved spell needs to re-enter a player-owned zone

## Tests

- full casting, resolution, combat, targeting, and BDD regression coverage remains green
