# Cast Own-Turn-Priority Artifact In Upkeep Window

## Summary

Allow a supported artifact card whose explicit casting rules include `OpenPriorityWindowDuringOwnTurn` to be cast in `Upkeep` while its controller is the active player.

## Scope

- add executable proof for a non-creature spell card with a turn-relative open-priority casting rule
- reuse the existing stack corridor for a permanent spell that resolves to the battlefield
- keep the rule constrained to the current controller-turn context model

## Out Of Scope

- full `Flash`
- temporary permission effects
- opponent-turn casting for this rule
