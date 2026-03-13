# Current State — DemonicTutor

## Implemented slices

1. StartGame
2. DrawOpeningHands
3. PlayLand
4. AdvanceTurn
5. DrawCard

## Current model

The system currently supports:

* game creation
* opening hand assignment
* explicit card draw
* land play with minimal legality
* minimal turn advancement

## Explicit temporary constraints

* exactly 2 players
* opening hand size fixed to 7
* only `Phase::Main` exists
* no stack
* no priority
* no spell casting
* no mulligan
* no automatic draw step
* no persistence or event store yet
* no event bus yet

## Quality state

* tests passing
* strict clippy passing
* formatting passing
* documentation aligned with implementation

## Recommended next step

Do not add new functionality immediately.

First decide whether the next iteration should focus on:

* another gameplay slice
* internal refactor
* or first infrastructure extraction
