# Implemented Slice — Remove Duplicated Spell Metadata From Stack Objects

## Summary

Make `SpellOnStack` carry one canonical spell representation by deriving immutable spell metadata from the spell snapshot instead of storing duplicate stack-local copies.

## Supported Behavior

- stack behavior remains unchanged
- spell resolution still reads the same metadata it needs
- spell stack objects now use the spell snapshot as their single canonical spell representation

## Invariants

- stack resolution semantics remain deterministic
- observable event payloads remain unchanged
- this slice does not expand Magic rules support

## Implementation Notes

- `SpellOnStack` no longer duplicates `source_card_id`, `card_type`, or supported spell rules next to the snapshot
- immutable spell facts are now derived from the snapshot on demand

## Tests

- full stack, targeting, casting, and resolution regression coverage remains green
