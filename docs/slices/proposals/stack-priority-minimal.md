# Slice Name

StackPriorityMinimal

---

## Goal

Introduce the first minimal, semantically honest implementation of stack and priority for two-player play.

---

## Why This Slice Exists Now

The current model already supports enough spell and turn behavior that immediate spell resolution is becoming the main remaining semantic distortion.

Adding more gameplay before stack and priority would likely increase inconsistency faster than value.

---

## Supported Behavior For The First Implementation Wave

- spell cards are cast onto a real stack zone instead of resolving immediately
- the game opens a priority window when appropriate
- players may pass priority
- if both players pass with a non-empty stack, the top object resolves
- if both players pass with an empty stack, the current window can close and turn flow may continue

---

## Explicitly Out Of Scope For The First Wave

- triggered abilities
- activated abilities on the stack
- targets
- modes
- replacement effects
- multiplayer
- combat-specific stack interactions beyond generic spell timing

---

## Proposed Aggregate Changes

- add stack state to `Game`
- add priority state to `Game`
- integrate supported SBA review before granting priority

---

## Proposed Commands

- reinterpret `CastSpellCommand` as “cast onto stack”
- add `PassPriorityCommand`

---

## Proposed Events

- `SpellPutOnStack`
- `PriorityPassed`
- `StackTopResolved`

`SpellCast` should not remain the canonical event for both casting and resolution once this slice starts landing.

---

## Proposed Internal Modules

```text
src/domain/play/game/
  model/
    priority.rs
    stack.rs
  rules/
    stack_priority/
      mod.rs
      casting.rs
      passing.rs
      resolution.rs
```

---

## Patterns / Abstractions

- use a small explicit state machine for priority
- use enums for stack object kinds
- use deterministic transition helpers
- avoid trait-object based rule engines
- keep everything aggregate-owned

---

## Slice Decomposition

1. `StackFoundation`
2. `CastSpellToStack`
3. `PassPriority`
4. `ResolveSpellFromStack`
5. `TurnFlowPriorityWindows`

---

## Rules Reference

- 117
- 405
- 601
- 608
- 704

---

## Rules Support Statement

This proposal is for a minimal two-player stack and priority model. It aims to make spell casting and resolution semantically real without attempting full Magic timing support in one iteration.
