# ADR 0003 — Events are persisted and published from the application layer

## Status
Accepted

## Context

DemonicTutor needs a model that is:
- deterministic
- testable
- replayable
- compatible with event persistence
- free from direct infrastructure concerns in the domain core

If the aggregate itself publishes events directly, the domain becomes coupled to:
- event bus concerns
- infrastructure timing
- application orchestration
- side effects

That would weaken domain purity and make testing harder.

## Decision

Aggregates emit domain events as return values from command handling, but they do not publish them directly.

Application services are responsible for:
1. loading or creating the aggregate
2. executing the command
3. obtaining produced events
4. persisting those events
5. publishing them to the event bus
6. triggering projections or side effects

## Consequences

### Positive
- cleaner domain model
- easier unit testing
- explicit orchestration flow
- easier replay and persistence model
- better separation of concerns

### Negative
- application layer becomes more important
- command flow requires more explicit plumbing
- some additional boilerplate is introduced

## Notes

The initial event bus should remain simple and in-process.

More advanced concurrency or actor-based patterns may be considered later, but they are not required for the initial architecture.
