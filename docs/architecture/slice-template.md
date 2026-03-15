# Slice Template — DemonicTutor

Use this template to design or document a vertical slice in DemonicTutor.

A slice must introduce **one coherent, testable, observable behavior**.

It must remain:

- minimal
- truthful about rules support
- aligned with Domain-Driven Design
- easy to review

This template is optimized for small, truthful, DDD-aligned slices.
If a proposed slice cannot be expressed clearly with this template, it is probably too large.

---

## Slice Name

Short, explicit, behavior-oriented name.

Example:

`DrawCard`

---

## Goal

State in one short paragraph:

- what this slice enables
- why it is needed now
- what immediate behavior becomes possible

Keep this concrete and observable.

---

## Why This Slice Exists Now

Explain why this slice is the next sensible step.

Good reasons include:

- it unlocks a necessary gameplay behavior
- it closes an inconsistency in the current model
- it supports the next planned slice
- it makes the domain more truthful without over-expanding scope

Avoid speculative future-oriented justification.

---

## Supported Behavior

List only the behaviors that this slice will actually support.

Be explicit and concrete.

Examples:

- accept a `DrawCardCommand`
- verify the player exists
- verify the player is the active player
- draw exactly one card from the player library into the hand
- emit `CardDrawn`

Do not describe hoped-for future behavior here.

---

## Invariants / Legality Rules

List the minimum domain rules enforced by this slice.

Examples:

- only the active player may draw through this command
- drawing fails if the library is empty
- drawing is only legal during the supported phase(s)

These must be **domain rules**, not implementation notes.

---

## Out of Scope

Explicitly list what is intentionally unsupported after this slice.

This section is mandatory.

Examples:

- automatic draw step
- priority
- stack
- replacement effects
- drawing multiple cards
- losing by decking

This section protects truthfulness.

---

## Domain Impact

Describe only the domain elements that must change.

Possible categories:

### Aggregate Impact
- changes to `Game`
- changes to ownership or legality responsibilities

### Entity / Value Object Impact
- changes to `Player`
- changes to `CardInstance`
- new value objects if strictly required

### Commands
- new commands
- changed command semantics

### Events
- new domain events
- changed event semantics

### Errors
- new domain errors
- changed validation outcomes

Do not describe infrastructure or UI here unless they are directly relevant.

---

## Ownership Check

State clearly where the behavior belongs.

Possible answers include:

- `Game` aggregate
- gameplay domain outside the aggregate
- application orchestration
- infrastructure
- analytics

Also state why it belongs there.

This section protects DDD boundaries.

---

## Documentation Impact

List only the documents whose **owned truth** changes.

Typical examples:

- `docs/domain/current-state.md`
- `docs/domain/aggregate-game.md`
- `docs/domain/DOMAIN_GLOSSARY.md`
- `docs/domain/context-map.md`
- `docs/architecture/vertical-slices.md`
- the slice document itself
- relevant ADRs

Do not update documentation mechanically.

---

## Test Impact

List the smallest set of tests needed to make the slice reviewable.

Prefer focused behavior-oriented tests.

Examples:

- succeeds for valid input
- fails when legality condition is not met
- emits the expected event
- preserves invariant X

Do not inflate this section with exhaustive implementation detail.

---

## Rules Support Statement

State clearly what this slice means for actual Magic rules support.

Use wording like:

- “This slice introduces a minimal explicit draw action.”
- “This slice does not model the full draw step.”
- “This slice supports a simplified legality model only.”

This section is mandatory whenever the slice touches gameplay rules.

---

## Open Questions

Include only questions that materially affect correctness or ownership.

Do not include speculative brainstorming.

If there are no such questions, omit this section.

---

## Review Checklist

Before finalizing the slice, verify:

- Is the slice minimal?
- Does it introduce one coherent behavior?
- Are the legality rules explicit?
- Is out-of-scope behavior stated clearly?
- Does it avoid implying unsupported rules?
- Is ownership clear?
- Does it preserve bounded context and aggregate boundaries?
- Are documentation updates limited to changed truth owners?
- Is the slice easy to review and test?

---

# Minimal Example Structure

## Slice Name

DrawCard

## Goal

Allow the active player to draw exactly one card from their library into their hand.

## Why This Slice Exists Now

This slice enables direct hand progression and supports later gameplay actions that depend on card movement from library to hand.

## Supported Behavior

- accept `DrawCardCommand`
- verify player exists
- verify player is active
- draw exactly one card
- move the card to hand
- emit `CardDrawn`

## Invariants / Legality Rules

- only the active player may draw through this command
- drawing fails if no card is available
- drawing is only legal in supported phases

## Out of Scope

- automatic draw step
- decking loss
- replacement effects
- drawing multiple cards
- stack and priority

## Domain Impact

### Aggregate Impact
- extend `Game` with explicit draw behavior

### Commands
- add `DrawCardCommand`

### Events
- add `CardDrawn`

## Ownership Check

This behavior belongs to the `Game` aggregate because it affects gameplay legality, player state, and zone transitions.

## Documentation Impact

- `docs/domain/current-state.md`
- relevant slice document

## Test Impact

- draw succeeds for active player
- draw fails for inactive player
- draw fails when library is empty
- draw emits `CardDrawn`

## Rules Support Statement

This slice introduces a minimal explicit draw action and does not model the full Magic draw step.
