# Slice Name

ConsumePoppedStackObjectsDuringResolution

---

## Goal

Consume a popped `StackObject` directly during resolution instead of borrowing it and cloning its kind payload.

---

## Why This Slice Exists Now

The stack had already moved to cheaper internal object ids and spell snapshots, but resolution still cloned `StackObjectKind` after `pop()`.

This slice exists to:

1. consume the just-popped stack object directly
2. remove an unnecessary clone from a hot resolution path
3. align stack resolution with the cheaper internal stack carrier

---

## Supported Behavior

- `pass_priority` pops a stack object and hands ownership directly to resolution
- spell resolution extracts metadata and spell payload from the owned stack object
- externally visible events and outcomes remain unchanged

---

## Invariants / Legality Rules

- stack resolution semantics remain unchanged
- no new timing or targeting behavior is introduced
- stack object ids remain stable at the public boundary

---

## Out of Scope

- broader stack redesign
- trigger support
- new stack-visible object kinds

---

## Domain Impact

### Aggregate Impact

- stack resolution now consumes its owned runtime carrier directly

### Entity / Value Object Impact

- `StackObject` gains an owned `into_kind()` extraction path

### Commands

- no new public command required

### Events

- no event payload changes

### Errors

- no new public error required

---

## Documentation Impact

- this slice document

---

## Test Impact

- full unit and BDD regression coverage remains green

---

## Rules Reference

- none beyond the current supported stack semantics

---

## Rules Support Statement

This slice is a runtime-resolution refactor only. It does not expand Magic rules support.
