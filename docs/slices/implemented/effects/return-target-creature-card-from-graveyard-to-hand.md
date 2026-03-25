# Slice Implemented - Return Target Creature Card From Graveyard To Hand

## Outcome

The supported subset can now return a target creature card from a graveyard to its owner's hand.

## Supported Behavior

- targets one explicit creature card in a graveyard
- revalidates that the card is still in a graveyard when the spell resolves
- moves the card to hand without implying broader recursion semantics
