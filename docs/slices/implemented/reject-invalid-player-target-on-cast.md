# Slice Name

RejectInvalidPlayerTargetOnCast

---

## Goal

Reject a targeted spell if the chosen player target is not in the game.

---

## Supported Behavior

- player targets are validated at cast time
- an unknown player target makes the cast illegal

---

## Rules Reference

- 114.1
- 601.2c

---

## Rules Support Statement

The current targeted-spell subset validates player targets when the spell is cast and rejects unknown players immediately.
