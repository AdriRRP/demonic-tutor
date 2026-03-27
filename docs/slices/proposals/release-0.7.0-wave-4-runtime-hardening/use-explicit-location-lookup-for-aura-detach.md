# Slice Name

`UseExplicitLocationLookupForAuraDetach`

## Goal

Make Aura detach cleanup locate the enchanted permanent through explicit runtime location information instead of scanning all battlefields.

## Why This Slice Exists Now

The current attachment subset is already live, and the engine now has a transactional location index. Aura cleanup should use the same explicit location corridor rather than re-searching all battlefields on zone exit.

## Supported Behavior

- detach cleanup finds the enchanted permanent through explicit location lookup
- attached stat bonuses and pacifism-style restrictions are still removed when the Aura leaves the battlefield

## Invariants / Legality Rules

- only battlefield targets may receive detach cleanup
- if the attached permanent no longer exists on the battlefield, cleanup becomes a no-op

## Out of Scope

- adding new attachment families
- adding full attachment indexing or a generic continuous-effect engine

## Domain Impact

### Aggregate Impact
- zone-transition helpers consume explicit location information for Aura detach

## Ownership Check

This belongs to gameplay domain zone-transition rules because Aura cleanup is authoritative game-state maintenance, not a projection concern.

## Documentation Impact

- this slice document

## Test Impact

- stat Aura cleanup still removes the bonus
- pacifism-style Aura cleanup still removes the restriction
- detach cleanup does not scan unrelated battlefields

## Rules Reference

- 303.4 — Aura attachment semantics, simplified to the supported subset
- 704.5m — unattached Aura cleanup, simplified to the supported subset

## Rules Support Statement

This slice preserves the same Aura subset and only hardens how the cleanup is located internally.

## Open Questions

- none
