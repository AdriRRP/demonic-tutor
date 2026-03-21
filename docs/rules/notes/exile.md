# Rules Notes — Exile

## Purpose

Summarize the rule areas DemonicTutor currently uses to model the exile zone.

This is a repository-owned interpretation note, not a copy of the Comprehensive Rules.

## Relevant Rules

- 406 — Exile

## Current DemonicTutor Interpretation

- each player owns an exile zone
- cards can be moved to exile from:
  - the battlefield (permanents exiled by effects)
  - the graveyard (cards exiled from graveyard)
- the current runtime exposes a direct `ExileCard` command as a minimal public effect entrypoint for those moves
- exiled cards are kept face up by default
- any player may examine cards in exile
- the aggregate maintains an ordered collection of `CardInstance` per player's exile zone
- no "return from exile" behavior is currently modeled
- no exile-linked abilities are currently modeled
- no face-down exile is currently modeled

## Out of Scope

- return from exile to battlefield or other zones
- exile-linked abilities (CR 406.6)
- face-down exile (CR 406.3a, 406.4)
- exile pile organization (CR 406.5)
- triggered abilities that fire when cards are exiled
- suspend or other mechanics that depend on exile timing
- exile caused by replacement effects
- Auras or Equipment being attached from exile

## Related Features

- `features/zones/exile_zone.feature`
