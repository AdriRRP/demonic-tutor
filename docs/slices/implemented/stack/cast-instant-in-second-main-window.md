# Cast Instant In Second Main Window

## Status

Implemented

## Summary

The active player may cast and resolve an instant during the empty priority
window that opens in `SecondMain`.

## Supported Behavior

- `SecondMain` opens priority for the active player when entered
- the active player may cast an instant while holding that priority
- the instant is put on the stack and resolves after two consecutive passes
- after resolution, priority reopens for the active player while the game
  remains active

## Out Of Scope

- non-instant spell responses outside normal main-phase casting semantics
- broader multi-object stack interaction beyond the currently supported subset
