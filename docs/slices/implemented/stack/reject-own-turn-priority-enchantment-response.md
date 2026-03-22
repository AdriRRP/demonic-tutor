# Reject Own-Turn-Priority Enchantment Response

## Goal

Reject a supported enchantment card whose explicit casting rules include `OpenPriorityWindowDuringOwnTurn` when its controller tries to cast it as a response during the opponent's turn.

## Implemented behavior

- the active player may open the stack with an instant as before
- after the active player passes, the non-active player cannot cast the supported enchantment
- the action fails with the shared casting-permission error corridor for own-turn open-priority spells

## Out of scope

- broader contextual casting permissions beyond the current explicit card-face rule
