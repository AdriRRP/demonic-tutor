# Slice Name

SpellTargetFoundation

---

## Goal

Introduce an explicit target value object for stack-borne spells.

---

## Supported Behavior

- add `SpellTarget` as a play-domain value object
- support `SpellTarget::Player(PlayerId)`
- support `SpellTarget::Creature(CardInstanceId)`
- store the chosen target on `CastSpellCommand`
- persist the chosen target on the spell object while it stays on the stack
- expose the target through `SpellPutOnStack`

---

## Out Of Scope

- target legality changes after cast
- multiple targets
- modal spells
- target replacement or redirection

---

## Rules Reference

- 114
- 601.2c

---

## Rules Support Statement

DemonicTutor now supports a minimal explicit targeting model for stack-borne spells. A supported spell may carry exactly one explicit target, and that target is preserved on the stack object until resolution.
