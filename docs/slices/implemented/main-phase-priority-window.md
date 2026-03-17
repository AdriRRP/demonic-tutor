# Slice — Main Phase Priority Window

## Goal

Open an explicit empty priority window when the active player enters `FirstMain` or `SecondMain`.

## Supported Behavior

- advancing into `FirstMain` opens a priority window for the active player
- advancing into `SecondMain` opens a priority window for the active player
- two consecutive passes close an empty main-phase priority window
- turn advancement is rejected while the priority window remains open
- if the stack is non-empty, unrelated gameplay actions are still rejected until the stack resolves or the window closes

## Explicit Limits

- only `FirstMain` and `SecondMain` open turn-flow priority windows in the current model
- the empty main-phase window is still a minimal stack-era simplification, not full Magic priority support
- broader turn-flow priority windows for upkeep, draw step, combat, and cleanup are still out of scope

## Domain Changes

- `advance_turn` now opens `PriorityState` when entering `FirstMain` or `SecondMain`
- `PassPriority` can close an empty main-phase window without resolving a stack object
- helper flows that advance turn state must explicitly close empty windows before continuing

## Rules Support Statement

This slice introduces the first non-casting priority windows in the runtime model. Main phases now behave more like real gameplay: entering `FirstMain` or `SecondMain` opens an empty priority window for the active player, that window can be passed away explicitly, and turn flow cannot jump past it implicitly.

## Tests

- advancing from `Draw` to `FirstMain` opens priority for the active player
- two consecutive passes close an empty main-phase priority window
- unit and BDD setup helpers close empty main-phase windows explicitly when they need to continue advancing turn flow
