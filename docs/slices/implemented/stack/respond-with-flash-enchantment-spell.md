# Respond With Flash Enchantment Spell

## Summary

Allow the non-active player to cast a supported enchantment card with explicit `OpenPriorityWindow` permission while responding to a spell already on the stack.

## Scope

- an enchantment card with minimal `Flash`-like support may be cast as a response
- the response happens on an already-open stack window after the first pass
- the responding player retains priority after the flash enchantment is put on the stack

## Out Of Scope

- broader keyword support beyond the explicit casting-rule subset
- automatic enchantment response support for all enchantment cards
