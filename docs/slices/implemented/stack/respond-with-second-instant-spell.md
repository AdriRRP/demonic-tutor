# Slice — Respond With Second Instant Spell

## Goal

Allow the current non-active priority holder to cast a second instant spell in response before passing priority after already responding once to a spell on the stack.

## Supported Behavior

- Alice may cast a spell onto the stack and pass priority
- Bob may cast an instant response while holding priority
- after that first response is put on the stack, Bob keeps priority
- while still holding priority, Bob may cast a second instant response
- the second response is placed on top of the stack
- after two consecutive passes, the second response resolves first and Bob's original response remains above Alice's original spell
- after the top response resolves and the game remains active, priority reopens for the active player

## Explicit Limits

- second-response self-stacking is still limited to the currently supported instant subset
- this slice only proves responding-player self-stacking on an already non-empty stack
- it does not yet broaden this pattern across every supported empty priority window; those remain separate slices
- the currently supported targeted instant subset is allowed, but broader targeting, modes, activated abilities, and triggered abilities remain out of scope

## Domain Changes

- no new public command is introduced
- the existing stack and priority model is now covered for consecutive response casts by the non-active player

## Rules Support Statement

This slice completes the minimal stack model's baseline response pattern. Once the responding player legally casts an instant onto an existing stack, they keep priority and may cast a second instant before any player passes. Resolution continues in LIFO order, leaving the first response above the original spell after the top resolves.

## Tests

- the responding player may cast a second instant while retaining priority on an existing stack
- both responses remain above the original spell until passes begin
- the top response resolves first and the original response remains above the original spell afterward
