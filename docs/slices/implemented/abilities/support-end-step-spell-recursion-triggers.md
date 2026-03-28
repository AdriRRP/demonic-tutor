# Slice Implemented - Support End-Step Spell Recursion Triggers

## Outcome

The engine now supports one explicit beginning-of-end-step trigger profile that returns the first supported instant or sorcery card from its controller's graveyard to hand.

## What Landed

- one bounded `BeginningOfEndStep` triggered-effect profile for spell recursion
- triggered resolution through the existing stack corridor without adding a parallel delayed-effect engine
- reuse of the current graveyard-to-hand zone-move semantics during triggered resolution
- focused end-step coverage proving the trigger returns a previously used spell from graveyard to hand

## Notes

- this slice is intentionally narrow: it does not add generic graveyard targeting, optional choices, delayed trigger objects, or arbitrary selection among multiple spell cards
- the current profile returns the first supported instant or sorcery card in the controller's visible graveyard order
