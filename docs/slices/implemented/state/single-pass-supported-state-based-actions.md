# Slice Name

SinglePassSupportedStateBasedActions

---

## Goal

Reduce the current supported SBA review to one battlefield sweep per iteration instead of separate zero-toughness and lethal-damage scans.

---

## Why This Slice Exists Now

The supported SBA subset is still small and explicit, but the implementation still scanned battlefields twice and allocated separate vectors for each creature check.

This slice exists to:

1. gather zero-toughness and lethal-damage deaths in one battlefield pass per iteration
2. keep zero-life game end explicit and ordered after creature checks
3. preserve the current deterministic supported SBA loop with less internal work

---

## Supported Behavior

- zero-toughness and lethal-damage creature checks are gathered in one battlefield sweep per iteration
- zero-life game end remains explicit and ordered after creature checks
- repeated supported SBA iterations still continue until no further supported change remains

---

## Invariants / Legality Rules

- no unsupported SBA are implied
- the currently supported checks remain explicit
- the review remains deterministic and aggregate-owned

---

## Out of Scope

- broader SBA coverage
- replacement effects
- trigger handling

---

## Domain Impact

### Aggregate Impact

- the `Game` aggregate now reviews the supported creature SBA subset through one battlefield pass per iteration

### Entity / Value Object Impact

- no new public entity type is introduced

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
- supported creature deaths and zero-life endings still occur through the shared SBA corridor

---

## Rules Reference

- 704

---

## Rules Support Statement

This slice optimizes the current explicit SBA subset only. It does not claim broader SBA support.
