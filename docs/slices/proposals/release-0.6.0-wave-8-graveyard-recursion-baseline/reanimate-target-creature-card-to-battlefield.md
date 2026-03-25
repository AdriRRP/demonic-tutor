# Slice Proposal - Reanimate Target Creature Card To Battlefield

## Goal

Introduce the first supported graveyard-to-battlefield corridor for creature cards.

## Why This Slice

Reanimation is one of the biggest jumps from a toy subset toward a recognizably usable Magic engine.

## Scope

- target creature card in a graveyard
- move it directly onto the battlefield under the supported controller
- ETB triggers fire through the existing trigger corridor
- the returned creature enters with normal runtime initialization for the supported subset

## Out of Scope

- tapped reanimation variants
- reanimate with counter modifications
- aura/equipment attachment semantics on reentry

## Notes

- this slice should stay explicit and profile-based
- do not imply generic support for every graveyard-reentry wording
