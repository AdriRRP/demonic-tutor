# Rules Notes — Turn Flow

## Purpose

Summarize the rule areas DemonicTutor currently uses to model turn progression.

This is a repository-owned interpretation note, not a copy of the Comprehensive Rules.

## Relevant Rules

- 500 — Turn structure
- 501 — Beginning phase
- 502 — Untap step
- 503 — Upkeep step
- 504 — Draw step
- 505 — Main phase
- 506 — Combat phase
- 507 — Ending phase

## Current DemonicTutor Interpretation

- the runtime uses a phase model of `Setup -> Untap -> Upkeep -> Draw -> FirstMain -> Combat -> SecondMain -> EndStep`
- turn progression emits `TurnProgressed`
- automatic untap applies only to the active player's permanents
- automatic draw happens in the Draw phase
- no priority windows are modeled
- no stack-based turn interaction is modeled
- no cleanup step is modeled yet

## Out of Scope

- priority
- skipped phases
- extra turns
- cleanup step
- triggered abilities tied to phase transitions
