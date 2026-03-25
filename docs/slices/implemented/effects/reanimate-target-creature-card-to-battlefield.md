# Slice Implemented - Reanimate Target Creature Card To Battlefield

## Outcome

The engine now supports one explicit graveyard-to-battlefield corridor for creature cards.

## Supported Behavior

- targets one creature card in a graveyard
- moves it onto the battlefield under the resolving spell controller
- reuses normal creature runtime initialization
- fires supported ETB triggers through the existing trigger stack corridor
