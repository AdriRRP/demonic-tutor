# Slice Name

CastTargetedInstantAtPlayer

---

## Goal

Allow a supported instant spell to target a player when it is cast.

---

## Supported Behavior

- a targeted instant may be cast in a currently supported instant-speed window
- the caster must choose a player target at cast time
- `SpellPutOnStack` records the chosen player target

---

## Rules Reference

- 114
- 601

---

## Rules Support Statement

The current stack model supports casting a targeted instant at a player. The target is chosen during casting and remains attached to the spell while it is on the stack.
