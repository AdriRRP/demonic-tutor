# Slice Name

CastTargetedInstantAtCreature

---

## Goal

Allow a supported instant spell to target a creature on the battlefield.

---

## Supported Behavior

- a supported targeted instant may target a creature by `CardInstanceId`
- the target creature must already be on the battlefield when the spell is cast
- `SpellPutOnStack` records the chosen creature target

---

## Rules Reference

- 114
- 601.2c

---

## Rules Support Statement

The current targeted-spell subset supports targeting a creature already on the battlefield. The target is selected during casting and preserved on the stack object.
