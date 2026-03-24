# Implemented Slice — Thin Stack Payloads To Minimal In-Flight State

## Summary

`SpellPayload` now carries only the in-flight state needed by the current supported spell families.
Instead of cloning a generalized definition record into every stack object, the payload is specialized as `Effect`, `Permanent`, or `Creature`.

## What Changed

- effect spells keep only definition id, type, mana cost, and supported spell rules
- noncreature permanent spells keep only definition id, type, mana cost, and any supported non-mana activated ability that must survive onto the battlefield
- creature spells keep only definition id, mana cost, creature stats, and keywords

## Outcome

- stack objects are smaller and more semantically focused
- reconstruction on resolution remains truthful to the current subset
- observable casting and resolution behavior remains unchanged
