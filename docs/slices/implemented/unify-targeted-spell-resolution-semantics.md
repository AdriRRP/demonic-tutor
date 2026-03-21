# Slice Name

UnifyTargetedSpellResolutionSemantics

---

## Goal

Centralize the minimal targeted-spell resolution path so player and creature targets do not diverge semantically.

---

## Supported Behavior

- supported targeted spell effects resolve through a shared spell-effect path
- player damage, creature damage, and post-resolution SBA review are coordinated in one place
- `PassPriority` observes targeted resolution through the same event surface as other spells
- supported spell targeting and resolution are read from explicit card-face profiles instead of string-matching on card-definition ids
- supported targeting now carries explicit legal-target rules rather than collapsing everything into a generic `AnyTarget` shortcut
- a targeted damage spell whose only creature target is no longer legal on resolution leaves the stack normally but does not apply its effect
- a provided target of the wrong legal class is rejected explicitly at cast time rather than being reported as a non-targeted spell

---

## Why This Slice Exists Now

Targeting introduces the first real branch in spell resolution semantics. Centralizing that path now keeps `stack_priority` small, explicit, and DDD-friendly before more spell effects are added.

---

## Rules Support Statement

The current implementation keeps targeted spell resolution in one explicit path rather than scattering it across unrelated casting or priority code. This preserves a small and readable stack model while the supported effect catalog is still intentionally tiny.
