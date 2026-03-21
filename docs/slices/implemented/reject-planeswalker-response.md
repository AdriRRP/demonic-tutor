# Reject Planeswalker Response

## Goal

Make explicit that a planeswalker spell cannot currently be cast as a response while the stack is
already open.

## Scope

In scope:

- rejecting a planeswalker spell when the non-active player tries to cast it as a response after the
  caster has passed priority
- surfacing that rejection through the current explicit casting-permission error

Out of scope:

- broader planeswalker timing rules
- a richer “sorcery speed” error taxonomy

## Notes

- The current stack model allows non-instant responses only when the stack is empty, the active
  player holds priority, and the game is in `FirstMain` or `SecondMain`.
- This slice keeps planeswalkers aligned with the same active-player empty-main-phase permission already enforced for
  sorceries, artifacts, and enchantments.
