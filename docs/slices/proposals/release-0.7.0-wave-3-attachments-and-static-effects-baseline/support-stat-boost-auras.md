# Slice Proposal - Support Stat Boost Auras

## Goal

Support a bounded Aura subset that gives the enchanted creature an explicit `+N/+N` bonus while attached.

## Scope

In:

- one explicit attached stat bonus profile
- power and toughness projection including the Aura bonus
- cleanup when the Aura leaves the battlefield or becomes unattached

Out:

- mixed keyword plus stat Auras
- layer-general static effect composition
- multiple simultaneous Aura boosts beyond deterministic additive stacking

## Dependency

Requires:

- `support-creature-aura-casting-and-attachment`

