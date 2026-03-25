# Slice Implemented - Support Optional May Triggers And Abilities

## Outcome

The engine now supports the first explicit `you may` corridor through a pending binary choice on stack resolution.

## What Landed

- a pending optional-effect state in the `Game` aggregate
- an explicit `ResolveOptionalEffectCommand` with `yes` or `no`
- the first supported optional profile:
  - triggered `ETB: you may gain life`
- public legal actions and choice requests now expose bounded binary `Yes/No` decisions
- answering `no` resolves the stack object cleanly with no effect
- answering `yes` resolves the supported effect exactly once

## Notes

- this slice intentionally covers one explicit triggered `may` corridor first
- it establishes the public UI contract for binary choices without opening full hidden-information or replacement logic
