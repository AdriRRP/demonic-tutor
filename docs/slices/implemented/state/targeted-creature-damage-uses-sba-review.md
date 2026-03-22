# Slice Name

TargetedCreatureDamageUsesSBAReview

---

## Goal

Let targeted creature damage flow through the shared review of supported state-based actions.

---

## Supported Behavior

- targeted spell damage may become lethal to the creature
- lethal creature damage is converted into `CreatureDied` through the shared SBA review path

---

## Rules Reference

- 704.5g

---

## Rules Support Statement

The current targeted-spell subset does not destroy creatures ad hoc. It marks damage first and then relies on the shared supported-SBA review to emit `CreatureDied` if that damage is lethal.
