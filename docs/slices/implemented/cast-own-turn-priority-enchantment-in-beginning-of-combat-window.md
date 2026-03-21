# Cast Own-Turn-Priority Enchantment In Beginning Of Combat Window

## Goal

Allow a supported enchantment card whose explicit casting rules include `OpenPriorityWindowDuringOwnTurn` to be cast in `BeginningOfCombat` while its controller is the active player.

## Implemented behavior

- the active player may cast the supported enchantment in the `BeginningOfCombat` priority window during its own turn
- the spell uses the shared stack and priority corridor
- after two passes, the enchantment resolves onto the battlefield

## Out of scope

- opponent-turn casting
- richer contextual casting restrictions beyond the current explicit card-face rule
