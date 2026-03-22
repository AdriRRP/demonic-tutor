# Slice Proposal — Consume Popped Stack Objects During Resolution

## Goal

Make stack resolution consume the popped `StackObject` directly so the engine does not clone stack-borne spell state during extraction.

## Why This Slice Exists Now

The stack already stores a lighter spell snapshot, but the current resolution corridor still clones `StackObjectKind` after `pop()`. That wastes one more copy in a hot path we already know is central.

## Supported Behavior

- resolving the top stack object consumes the popped object directly
- spell extraction keeps producing the same resolution data and events
- no observable stack behavior changes

## Invariants / Legality Rules

- the top stack object is removed exactly once before resolution
- extracted spell metadata remains identical to current supported behavior
- no new stack object kinds are implied

## Out of Scope

- abilities on the stack
- broader stack redesign
- event schema changes

## Domain Impact

- tighten the resolution corridor around owned `StackObject` consumption
- keep explicit extraction, destination, and effect phases separated

## Ownership Check

Stack resolution remains aggregate-owned gameplay flow inside `Game`.

## Documentation Impact

- `docs/domain/current-state.md` only if stack carrier wording changes materially
- this proposal file

## Test Impact

- regression tests around spell resolution outcomes
- no new BDD required if behavior stays identical

## Rules Reference

- none beyond the current supported stack subset

## Rules Support Statement

This slice removes an internal clone in stack resolution. It does not expand stack rules support.
