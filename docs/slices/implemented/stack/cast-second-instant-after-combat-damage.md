# Slice - Cast Second Instant After Combat Damage

## Goal

Allow the active player to cast a second instant spell after combat damage resolves, before passing priority after the first spell is put on the stack.

## Supported Behavior

- resolving combat damage reopens priority for the active player while the game remains active
- after the first instant is cast, the caster keeps priority
- while still holding priority, the active player may cast a second instant
- the second instant is placed on top of the stack
- after two consecutive passes, the second instant resolves first and the original spell remains on the stack

## Explicit Limits

- this slice only proves self-stacking in the post-combat-damage priority window
- it does not introduce additional end-of-combat or end-step timing
- it does not add new damage reassignment or prevention behavior

## Domain Changes

- no new public command is introduced
- the existing stack and priority model is now covered for consecutive instant casts after combat damage resolves

## Rules Support Statement

This slice proves that the minimal stack model remains coherent after combat damage has resolved: while the game is still in `Combat` and priority has reopened for the active player, they may cast one instant, keep priority, cast a second instant, and resolve the top object first through the normal two-pass flow.

## Tests

- the active player may cast a second instant while retaining priority after combat damage resolves
- both spells remain on the stack under the active player's control until passes begin
- the top spell resolves first and the original spell remains on the stack afterward
