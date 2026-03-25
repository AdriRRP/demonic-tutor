# Slice Implemented - Restrict Reanimation To Own Graveyard

## Outcome

The supported reanimation corridor now truthfully targets only creature cards in the caster's own graveyard.

## Supported Behavior

- reanimation may target a creature card only in the acting player's graveyard
- the card enters the battlefield under the resolving spell controller
- target validation now rejects opponent-graveyard reanimation attempts at cast time

## Notes

- this closes an architectural mismatch between control and ownership in the current aggregate model
- cross-owner battlefield control remains out of scope for the current subset
