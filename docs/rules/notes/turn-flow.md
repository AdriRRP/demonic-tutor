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
- if a player must draw from an empty library, the game ends immediately with `GameEnded(EmptyLibraryDraw)`
- explicit draw effects are modeled separately from the automatic draw step, are limited to main phases, and may draw multiple cards one by one
- if the active player is above the maximum hand size at `EndStep`, the turn cannot advance until they discard down to the maximum
- marked damage is cleared automatically when the game leaves `EndStep` for the next turn
- entering `FirstMain` or `SecondMain` opens an empty priority window for the active player
- entering `Combat` opens an empty priority window for the active player before attackers are declared
- turn-flow advancement is rejected while a priority window remains open
- no distinct cleanup step phase is modeled yet

## Out of Scope

- broader turn-flow priority windows beyond main phases, combat entry, and post-declaration combat windows
- skipped phases
- extra turns
- repeated cleanup loops from state-based actions or triggered abilities
- triggered abilities tied to phase transitions
