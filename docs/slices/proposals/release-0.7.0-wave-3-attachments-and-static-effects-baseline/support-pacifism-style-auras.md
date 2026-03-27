# Slice Proposal - Support Pacifism Style Auras

## Goal

Support a bounded Aura subset that prevents the enchanted creature from attacking and blocking while attached.

## Scope

In:

- explicit attached combat restriction profile
- legality checks in declare attackers and declare blockers
- automatic release of the restriction when the Aura leaves or becomes unattached

Out:

- activated ability suppression
- tap or untap restrictions
- broad text parsing for arbitrary "can't" clauses

## Dependency

Requires:

- `support-creature-aura-casting-and-attachment`

