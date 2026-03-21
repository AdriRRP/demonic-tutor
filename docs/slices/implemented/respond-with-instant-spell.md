# Slice — Respond With Instant Spell

## Goal

Allow the current priority holder to cast an instant spell in response to a spell already on the stack.

## Supported behavior

- when a priority window is open, the current holder may cast an instant spell from hand
- after a spell is cast, the caster keeps priority first
- the response spell is put on top of the stack
- priority then passes to the other player
- two consecutive passes resolve the top stack object first
- after the top resolves and the game is still active, priority reopens for the active player

## Explicit limits

- response spells are currently limited to `CardType::Instant`
- non-instant response spells are rejected
- response timing is still limited to the currently implemented priority windows
- the currently supported targeted instant subset is allowed, but broader targeting, modes, activated abilities, and triggered abilities remain out of scope

## Domain changes

- `Game::cast_spell()` now allows instant responses from the current priority holder
- `Game::pass_priority()` reopens priority for the active player after top-of-stack resolution when the game remains active
- `GameError::CastingTimingNotAllowed` now documents the current temporary timing limit through explicit spell timing semantics

## Rules support statement

This slice extends the minimal stack model with real spell responses. After a spell is cast, the caster keeps priority and may pass it. The next priority holder may then respond with an instant spell, that response becomes the new top object on the stack, and the stack continues to resolve in LIFO order through consecutive passes. Response timing is now available across the currently implemented priority windows, but broader response spell types and richer timing rules are still intentionally unsupported.

## Tests

- the opponent may cast an instant response while holding priority
- a response spell resolves before the original spell beneath it
- non-instant response spells are rejected explicitly
