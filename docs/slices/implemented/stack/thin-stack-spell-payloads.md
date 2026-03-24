# Implemented Slice — Thin Stack Spell Payloads

## Summary

Replace generalized stack spell snapshots with slimmer spell payload variants that only carry resolution-relevant data.

## Supported Behavior

- stack-borne spells now carry `SpellPayload` instead of a generalized snapshot object
- non-creature and creature spells use distinct payload shapes
- resolution still reconstructs `CardInstance` only when a destination corridor needs it

## Invariants

- stack semantics remain unchanged
- immutable spell definition data stays shared
- this slice does not expand supported Magic rules

## Implementation Notes

- `SpellPayload` now models supported stack spell families explicitly
- `SpellOnStack` stores payload plus payment and target data
- payload reconstruction stays local to stack resolution

## Tests

- full repository validation remains green after the payload simplification
