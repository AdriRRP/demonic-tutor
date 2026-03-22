# Slice — Exile Target Card From Graveyard

## Goal

Support a targeted spell that exiles a card from a graveyard.

## Supported Behavior

- a supported spell may choose a legal card in a graveyard
- on resolution, the target card moves from graveyard to its owner's exile zone
- the shared resolution corridor emits `CardExiled` for the effect

## Invariants

- the target must exist in a graveyard at cast time
- the effect does not apply if the target is gone or no longer legal on resolution
- this slice adds graveyard-card targeting only for this explicit effect

## Notes

- this does not imply broad targeting support for all non-battlefield zones
