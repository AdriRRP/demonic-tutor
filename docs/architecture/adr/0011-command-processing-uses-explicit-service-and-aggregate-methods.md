# ADR 0011 — Command processing uses explicit service and aggregate methods

## Status
Accepted

## Context

The trait-based command processing introduced in ADR 0010 added an extra architectural layer without actually reducing duplication in the codebase.

The project still maintained:

- explicit command methods on `Game`
- explicit command methods on `GameService`
- trait implementations for each command type

This created three places to touch for most gameplay operations and made the architecture harder to read. The generic entrypoint also exposed collection mutation patterns that were not helping the domain protect its own invariants.

At the current project size, explicit methods are easier to follow, easier to review, and better aligned with the repository's preference for small, reviewable changes.

## Decision

The project will use explicit command-specific methods in `GameService` and `Game` rather than a generic trait-based command execution layer.

Commands remain the language of intent in the model, but they are routed through explicit service methods and explicit aggregate operations.

Mutable zone internals should also remain encapsulated behind focused collection APIs instead of exposing raw mutable vectors.

## Consequences

### Positive

- less architectural duplication across application and domain layers
- simpler command flow to read and debug
- clearer aggregate API surface
- stronger protection of zone invariants
- easier incremental extension as slices are added

### Negative

- `GameService` still grows with one method per supported capability
- adding a new command still requires explicit plumbing
- future cross-cutting command concerns will need a different abstraction if they become substantial

## Notes

This decision supersedes ADR 0010.
