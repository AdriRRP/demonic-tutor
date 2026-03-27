# Support One Shot Cast From Graveyard With Exile On Resolution

`SupportOneShotCastFromGraveyardWithExileOnResolution`

## Goal

Narrow the current cast-from-own-graveyard corridor so one explicit supported spell profile is exiled when it resolves from that graveyard cast.

## Supported Behavior

- a supported spell profile may be cast from its controller's own graveyard
- if it was cast through that graveyard corridor and carries the explicit exile-on-resolution rule, it moves to exile when it resolves
- the same card cast normally from hand still resolves to graveyard

## Out Of Scope

- generic flashback, escape, retrace, or replacement-effect infrastructure
- alternate graveyard costs or timing rewrites
- broader destination-replacement systems

## Test Impact

- one explicit supported instant resolves to graveyard from hand
- the same instant resolves to exile when cast again from its own graveyard

## Rules Support Statement

This slice adds only a bounded flashback-like corridor for explicit supported cards that are exiled after resolving from their owner's graveyard.
