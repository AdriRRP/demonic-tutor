# Slice Proposal — Exile Target Card From Graveyard

## Goal

Support a targeted spell that exiles a card from a graveyard.

## Why This Slice Exists Now

The current targeting subset is centered on players and battlefield creatures. Graveyard targeting is the smallest new zone-relative expansion that adds real gameplay value.

## Supported Behavior

- a supported spell may choose a legal card in a graveyard
- on resolution, the target card moves from graveyard to its owner's exile zone

## Invariants / Legality Rules

- the target must exist in a graveyard at cast time
- the effect does not apply if the target is gone or no longer legal on resolution

## Out of Scope

- targeting cards in libraries, hands, or exile
- choosing among multiple graveyards in multiplayer

## Domain Impact

- extend target-legality rules beyond players and battlefield creatures
- extend supported resolution profiles with graveyard-to-exile movement

## Ownership Check

This is still aggregate-owned targeting and zone movement.

## Documentation Impact

- current-state
- glossary if graveyard-card target terminology becomes canonical
- implemented slice doc

## Test Impact

- unit coverage for legal cast, illegal cast, and lost target on resolution
- executable BDD if the setup remains small

## Rules Reference

- 114
- 406
- 608.2b

## Rules Support Statement

This slice adds graveyard-card targeting only for the explicit exile effect it introduces. It does not imply broad zone-target support.
