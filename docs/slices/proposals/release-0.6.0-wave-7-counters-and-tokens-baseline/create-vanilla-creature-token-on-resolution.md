# Slice Proposal - Create Vanilla Creature Token On Resolution

## Goal

Introduce the first supported token-creation corridor by allowing explicit spell or ability effects to create one vanilla creature token onto the battlefield.

## Why This Slice

Token creation is one of the highest-leverage gameplay capabilities still missing from the current subset.

It unlocks:

- board development without casting a full permanent card
- creature swarms and sacrifice fodder
- future ETB, death, and counter interactions

## Scope

- create exactly one explicit creature token profile on resolution
- token enters the battlefield under the controller's control
- token participates in combat, damage, SBA, and zone changes while on the battlefield
- token materialization is explicit and profile-based, not generic text parsing

## Out of Scope

- multiple token creation in one effect
- noncreature tokens
- token copies of existing permanents
- replacement effects that modify token entry

## Notes

- keep token identity explicit and engine-owned
- avoid implying support for arbitrary token text
