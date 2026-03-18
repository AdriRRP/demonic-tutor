# Slice Name

RejectMissingTargetOnCast

---

## Goal

Reject casting a targeted spell when no explicit target is supplied.

---

## Supported Behavior

- a targeted spell fails to cast if no target is provided
- the failure happens at cast time before the spell enters the stack

---

## Rules Reference

- 601.2c

---

## Rules Support Statement

The current model requires supported targeted spells to receive their explicit target during casting. Missing targets make the cast illegal.
