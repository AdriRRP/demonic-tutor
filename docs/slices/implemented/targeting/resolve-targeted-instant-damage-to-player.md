# Slice Name

ResolveTargetedInstantDamageToPlayer

---

## Goal

Apply a supported targeted instant's damage to its targeted player during resolution.

---

## Supported Behavior

- a supported targeted instant may resolve for damage to a player
- the target player's life total changes through shared life semantics
- `PassPriority` now emits `LifeChanged` when targeted spell resolution changes player life

---

## Rules Reference

- 608.2
- 609.1
- 118

---

## Rules Support Statement

When a supported targeted instant resolves with a player target, the target player's life total changes during resolution through the same life-change semantics already used elsewhere in the aggregate.
