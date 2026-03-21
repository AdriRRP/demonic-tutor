# Reject Own-Turn-Priority Artifact Response

## Summary

Reject a supported artifact card whose explicit casting rules include `OpenPriorityWindowDuringOwnTurn` when its controller tries to cast it as a response during the opponent's turn.

## Scope

- prove that the turn-relative open-priority rule is narrower than generic `OpenPriorityWindow`
- keep the rejection in the current stack response corridor
- surface the error with casting-permission semantics rather than a generic timing failure

## Out Of Scope

- broader timing exceptions
- permanent changes to card permissions
