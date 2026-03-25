# Slice Proposal - Mill Cards Foundation

## Goal

Support explicit self-mill and target-player mill effects as the first graveyard-filling corridor not tied to draw.

## Why This Slice

Recursion becomes much more playable when the engine can intentionally move cards from library to graveyard without drawing them.

## Scope

- move the top `N` cards from a player's library to graveyard
- emit one event per milled card or an equivalent truthful aggregate event
- losing by draw from empty library remains unchanged

## Out of Scope

- replacement effects that modify milling
- triggers keyed specifically to mill events beyond the currently supported dies/ETB/upkeep/end-step set

## Notes

- keep milling distinct from drawing and discarding
- this slice is a strong enabler for recursion and graveyard targeting
