# ADR 0009 — Phase transitions use the State pattern for maintainability and extensibility

## Status
Accepted

## Context

At the time of this decision, the turn progression system in `src/domain/play/game/rules/turn_flow.rs` modeled phase transitions using simple enum methods (`Phase::next()`, `Phase::requires_player_change()`, `Phase::triggers_auto_draw()`).

As phase logic grows in complexity with new slices, the `advance_turn` function becomes harder to maintain due to:
- High cyclomatic complexity from conditional logic
- Tight coupling between phase transitions and game state updates
- Difficulty extending phase-specific behavior
- Lack of clear separation between phase-specific logic

The State pattern offers a solution by encapsulating phase-specific behavior in dedicated implementations, making the system more:
- **Maintainable**: Each phase's behavior is isolated
- **Extensible**: New phases can be added without modifying core logic
- **Testable**: Phase behavior can be tested in isolation
- **Clear**: Phase transitions become explicit state machines

## Decision

The project will **implement the State pattern** for phase transitions to improve maintainability and prepare for future phase complexity.

The implementation will:
1. Define a `PhaseBehavior` trait with methods for phase entry/exit and transition logic
2. Implement this trait for each `Phase` variant
3. Refactor `advance_turn` to delegate to the current phase's behavior
4. Maintain backward compatibility with existing phase logic

This decision aligns with the project's architectural evolution from minimal viable models (ADR 0007) to more maintainable patterns as complexity grows.

## Consequences

### Positive

- Reduces cyclomatic complexity in `advance_turn` function
- Encapsulates phase-specific behavior for better separation of concerns
- Makes phase logic more testable in isolation
- Provides clear pattern for adding new phase behaviors
- Improves code organization and readability
- Aligns with tactical DDD principles (ADR 0001) of explicit domain modeling

### Negative

- Increases initial implementation complexity
- Requires refactoring of phase-related code
- Adds abstraction layer that may be overhead for simple cases
- Changes must be coordinated across multiple files

## Notes

This implementation represents an evolution from the "minimal turn model" (ADR 0007) to a more structured approach as phase logic grows. The State pattern is particularly appropriate for Magic's turn structure, which has well-defined phases with specific rules.

The implementation will preserve all existing behavior while providing a cleaner architecture for future extensions.
