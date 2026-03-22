# Target Self Player When Rule Allows It

## Status

Implemented

## Scope

- a supported `AnyPlayer` spell may target the acting player explicitly
- the self-target path is exercised in `FirstMain`
- the spell resolves through the shared player-target corridor and changes the acting player's life total

## Out Of Scope

- multiplayer self/teammate distinctions
- broader player-target restrictions beyond `AnyPlayer` and `OpponentOfActor`
