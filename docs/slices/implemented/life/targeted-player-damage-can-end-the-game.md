# Slice Name

TargetedPlayerDamageCanEndTheGame

---

## Goal

Let targeted spell damage reuse zero-life game-end semantics.

---

## Supported Behavior

- targeted instant damage to a player may reduce that player to 0 life
- `GameEnded(ZeroLife)` is emitted through the shared state-based review path

---

## Rules Reference

- 104.3b
- 704.5a

---

## Rules Support Statement

The current targeted-spell subset can end the game when resolved player damage reduces the targeted player to 0 life.
