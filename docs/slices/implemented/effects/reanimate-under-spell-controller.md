# Slice Implemented - Reanimate Under Spell Controller

## Outcome

The supported reanimation corridor now puts the target creature card onto the battlefield under the resolving spell controller, even when the card came from an opponent's graveyard.

## Supported Behavior

- targets one creature card in any graveyard
- on resolution, the card leaves its current graveyard
- the card enters the battlefield under the controller of the resolving reanimation spell
- supported ETB triggers still reuse the existing trigger stack corridor

## Notes

- this closes a correctness bug in the first reanimation implementation
- it does not widen the target matrix beyond the already supported graveyard-card target subset
