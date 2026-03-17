# ADR 0010 — Game aggregate uses Command pattern for unified command processing

## Status
Superseded

## Context

The `Game` aggregate currently processes commands through individual methods (`play_land`, `mulligan`, `advance_turn`, etc.). As the number of commands grows, this approach leads to:

- boilerplate proliferation in the aggregate interface
- inconsistent command handling patterns
- difficulty extending command processing logic
- lack of explicit command/command handler separation

The Command pattern (previously called CommandHandler pattern) offers a unified approach where each command implements a trait defining its processing logic, and the aggregate provides a single generic entry point.

This pattern aligns with tactical DDD principles (ADR 0001) and provides a cleaner architecture for the main aggregate (ADR 0002).

## Decision

The project will **implement the Command pattern** for all gameplay commands, with commands living in the application layer.

The implementation will:

1. Define a `Command` trait in the application layer (previously called `CommandHandler`)
2. Implement this trait for each existing command type
3. Replace individual command methods in `Game` with a single `execute_command<C: Command>(&mut self, command: C)` method
4. Maintain backward compatibility through gradual migration

This decision establishes a consistent pattern for command processing that scales better with additional gameplay commands.

## Consequences

### Positive

- Unified command processing interface in `Game` aggregate
- Reduced boilerplate and method proliferation
- Explicit separation between command intent and execution logic
- Commands live in the application layer where they belong (user intents)
- Easier to add new commands without modifying aggregate interface
- More consistent error handling across commands
- Better alignment with tactical DDD patterns (ADR 0001)

### Negative

- Significant refactoring of existing command handling code
- Changes to all command invocation points (GameService, tests)
- Initial complexity increase during transition
- Potential breaking changes for external callers
- Requires careful migration planning

## Notes

This implementation represents an evolution from ad-hoc command methods to a structured Command pattern. The pattern is particularly appropriate for Magic's command-driven gameplay where each action has specific validation and execution logic.

Migration was implemented gradually to minimize disruption, keeping backward compatibility through the transition period.

## Update (2026-03-16)

The pattern was renamed from `CommandHandler` to `Command` for better semantics, and the trait was moved to the application layer since commands represent user intents at the application level, not domain concepts.

## Update (2026-03-17)

Superseded by ADR 0011, which returns command handling to explicit application service methods and aggregate operations after the generic trait-based command layer proved more duplicative than simplifying.
