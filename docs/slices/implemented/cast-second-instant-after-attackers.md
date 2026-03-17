# Slice - Cast Second Instant After Attackers

## Goal

Allow the active player to cast a second instant spell after attackers are declared, before passing priority after the first spell is put on the stack.

## Supported Behavior

- declaring attackers reopens priority for the active player
- after the first instant is cast, the caster keeps priority
- while still holding priority, the active player may cast a second instant
- the second instant is placed on top of the stack
- after two consecutive passes, the second instant resolves first and the original spell remains on the stack

## Explicit Limits

- this slice only proves self-stacking in the post-attackers priority window
- it does not add new combat-step structure beyond the current simplified `Combat` phase
- it does not change attacker legality or damage assignment semantics

## Domain Changes

- no new public command is introduced
- the existing stack and priority model is now covered for consecutive instant casts after attackers are declared

## Rules Support Statement

This slice proves that the minimal stack model remains coherent once combat interaction has started: after attackers are declared and priority reopens for the active player, they may cast an instant, keep priority, cast a second instant, and resolve the top object first through the normal two-pass flow.

## Tests

- the active player may cast a second instant while retaining priority after attackers are declared
- both spells remain on the stack under the active player's control until passes begin
- the top spell resolves first and the original spell remains on the stack afterward
