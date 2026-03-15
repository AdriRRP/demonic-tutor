# ADR Template — DemonicTutor

Use this template to record a meaningful architectural decision in DemonicTutor.

An ADR should capture:

- what was decided
- why it was needed
- what trade-offs it introduces

An ADR is not:

- a backlog note
- a design essay
- a status report
- a slice document

Keep ADRs concise, explicit, and scoped.

---

## Title

Write a short, decision-focused title.

Good examples:

- `Game is the main aggregate in the play context`
- `Events are persisted and published from the application layer`
- `DrawCard is modeled as an explicit main-phase action`

Avoid vague titles such as:

- `Thoughts on turn flow`
- `Future architecture ideas`
- `Combat notes`

---

## Status

Use one of:

- `Proposed`
- `Accepted`
- `Superseded`

Use `Proposed` only when the decision is still under review.

Use `Accepted` when the decision is active project truth.

Use `Superseded` only when a newer ADR explicitly replaces it.

---

## Context

Describe the decision pressure that exists now.

This section should explain:

- what problem or ambiguity exists
- what design pressure must be resolved
- why the decision matters now

Keep it focused.
Do not narrate the full project history.

Good examples of context:

- ownership of gameplay legality is unclear
- a slice needs a temporary simplification
- event publication needs a clear boundary
- aggregate scope needs a stable rule

---

## Decision

State the decision directly.

Use explicit language such as:

- `The project will...`
- `The system uses...`
- `For slice X, the model introduces...`
- `In the play bounded context, Game will...`

This section should be short and concrete.

Avoid:

- vague wording
- alternatives analysis in prose
- speculative future design

---

## Consequences

Describe the effects of the decision.

Use two subsections when helpful:

### Positive

Examples:

- clearer ownership
- smaller design scope
- easier testing
- reduced ambiguity

### Negative

Examples:

- temporary limitation
- future refactoring pressure
- reduced flexibility
- increased explicit plumbing

Trade-offs should be honest.

---

## Notes

Optional.

Use this section only when needed for things like:

- temporary simplification disclaimers
- explicit supersession notes
- small clarifications that do not fit elsewhere

Do not use this section for general discussion.

---

# Minimal ADR Structure

## Title

Game is the main aggregate in the play context

## Status

Accepted

## Context

Early gameplay actions depend on globally coherent game state, including turn progression, phase progression, zone transitions, and legality checks. Splitting these concerns too early would increase orchestration complexity before the model is stable.

## Decision

In the `play` bounded context, `Game` will be the main aggregate root for early milestones.

`Game` will own the most important invariants related to gameplay legality, turn progression, phase progression, and zone-aware actions.

## Consequences

### Positive

- clearer ownership of gameplay legality
- simpler initial command handling
- stronger consistency model

### Negative

- aggregate may grow over time
- future refactoring may be required
- scope discipline becomes important

## Notes

This decision defines the initial aggregate strategy, not a permanent guarantee that all future concerns must remain inside `Game`.
