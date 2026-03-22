# Cast Own-Turn-Priority Artifact In Beginning Of Combat Window

## Goal

Allow a supported artifact card whose explicit casting rules include `OpenPriorityWindowDuringOwnTurn` to be cast in `BeginningOfCombat` while its controller is the active player.

## Implemented behavior

- the active player may cast the supported artifact in the `BeginningOfCombat` priority window during its own turn
- the spell uses the shared stack and priority corridor
- after two passes, the artifact resolves onto the battlefield

## Out of scope

- opponent-turn casting
- richer contextual casting restrictions beyond the current explicit card-face rule
