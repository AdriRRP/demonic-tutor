# Slice Proposal - Support Optional May Triggers And Abilities

## Goal

Support the first explicit `you may` corridor for triggered or activated effects whose controller can choose whether the effect resolves.

## Why

- many real cards become usable only when optional resolution exists
- the UI needs a canonical yes/no decision model early
- it generalizes a very common prompt pattern without opening full replacement or prevention logic

## In Scope

- one explicit yes/no choice request surfaced when a supported `may` effect reaches the decision point
- yes/no decision stored in the stack/runtime corridor for that object
- supported examples:
  - one triggered `you may gain life`
  - optionally one activated `you may` profile if it falls out naturally
- resolution applies the effect only when the controller answered `yes`

## Out of Scope

- hidden-information optional choices
- optional targets chosen only on resolution
- repeated `any number of times`
- APNAP ordering refinements beyond the already supported trigger ordering

## Acceptance

- the controller receives a yes/no pending choice
- answering `no` resolves cleanly with no effect
- answering `yes` applies the supported effect exactly once
- no other player can answer the choice

## Notes

- keep the public contract explicit: this should look like a bounded binary choice, not an overloaded spell target
