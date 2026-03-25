# Slice Implemented - Support Loot And Rummage Effects

## Outcome

The engine now supports explicit `loot` and `rummage` spell corridors through a pending hand-card choice at resolution time.

## What Landed

- two supported spell profiles:
  - `loot`: draw first, then choose a hand card to discard
  - `rummage`: choose a hand card to discard, then draw
- a pending hand-choice state in the `Game` aggregate with an explicit `ResolvePendingHandChoiceCommand`
- public legal actions and choice requests now expose the discard-selection prompt after the spell reaches the top of stack
- the selected card moves to graveyard through canonical discard semantics
- the supported spell itself still resolves to its normal destination after the pending hand choice completes

## Notes

- this slice intentionally keeps the corridor prompt-driven and deterministic instead of introducing a generic hidden-hand interaction framework
- `loot` performs its draw before the prompt opens; `rummage` performs its draw only after the chosen discard resolves
