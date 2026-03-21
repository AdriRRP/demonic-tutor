# Respond With Flash Creature Spell

## Summary

Allow the non-active player to cast a supported creature card with explicit `OpenPriorityWindow` permission while responding to a spell already on the stack.

## Scope

- a creature card with minimal `Flash` support may be cast as a response
- the response happens on an already-open stack window after the first pass
- the responding player retains priority after the flash creature is put on the stack

## Out Of Scope

- broader keyword support beyond the explicit casting-rule subset
- automatic creature response support for all creature cards
