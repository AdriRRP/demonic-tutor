---
name: ddd-review
description: Verify that a proposed change or slice respects Domain-Driven Design boundaries, aggregate ownership, and ubiquitous language in DemonicTutor.
---

# DDD Review Skill

## Purpose

Use this skill to evaluate whether a change respects the Domain-Driven Design structure of the DemonicTutor repository.

Typical use cases:

- after designing a slice
- after implementing a slice
- during architectural review
- when changes affect aggregates or domain concepts
- when domain rules or terminology evolve

This skill evaluates consistency with the domain model.
It does not redefine domain truth.

---

## Load Required Context

### Always load

- `docs/domain/DOMAIN_GLOSSARY.md`
- `docs/domain/context-map.md`
- `docs/domain/aggregate-game.md`
- `docs/domain/current-state.md`

### Also load when needed

- relevant slice documentation
- `docs/architecture/system-overview.md` if architectural layering is affected
- ADRs if the change touches architectural decisions

Avoid loading unrelated domain or slice documents.

---

## Core Goal

Verify that the proposed change preserves:

- bounded context boundaries
- aggregate ownership
- ubiquitous language
- domain rule locality
- canonical domain actions and event language

The goal is not to block change, but to ensure that the domain model remains coherent.

---

## Review Dimensions

Evaluate the change across the following dimensions.

---

### 1. Ubiquitous Language

Verify that all domain concepts:

- use terms defined in `DOMAIN_GLOSSARY.md`
- use consistent terminology across code and documentation
- avoid introducing near-synonyms for existing domain terms

Potential issues:

- new terms that duplicate existing concepts
- implementation jargon appearing as domain language
- inconsistent naming across slices
- convenience commands that contradict real game terminology
- duplicate public actions that split one real domain action across multiple names

If new domain terms appear, they may require glossary updates.

---

### 2. Bounded Context Integrity

Check that the change does not blur bounded context responsibilities.

Questions to ask:

- Does this behavior clearly belong to the gameplay domain?
- Is any responsibility leaking into infrastructure or UI layers?
- Does the change implicitly create a new bounded context?

Bounded context boundaries should remain explicit and stable.

---

### 3. Aggregate Ownership

Verify that domain behavior is implemented within the correct aggregate.

Check:

- who owns the rule
- who enforces legality
- who maintains invariants

Gameplay legality and turn logic should remain inside the gameplay domain.

Potential violations:

- rules implemented in helpers
- legality checks outside aggregates
- domain logic in controllers or infrastructure
- duplicate commands for one real domain action when one entrypoint should be canonical
- event streams that require consumers to reconstruct basic intent from technical deltas instead of explicit domain facts
- new gameplay actions that ignore an already-open priority window

---

### 4. Invariant Protection

Verify that invariants are enforced by the domain model.

Check whether the change:

- introduces new invariants
- weakens existing invariants
- shifts responsibility for invariants outside the aggregate

Domain invariants must remain explicit and centrally enforced.

---

### 5. Slice Minimality

Check whether the slice remains minimal.

Questions:

- Does the slice introduce unrelated behaviors?
- Does it attempt to solve future problems prematurely?
- Could the slice be split into smaller slices?

Vertical slices should introduce **one coherent capability**.

---

### 6. Architectural Leakage

Verify that domain logic is not leaking into inappropriate layers.

Potential leaks include:

- domain rules implemented in services or utilities
- gameplay logic inside infrastructure code
- domain state manipulated outside aggregates

Domain rules should remain in the domain model.

---

### 7. Future Assumption Risk

Check whether the change assumes future mechanics.

Common anti-patterns:

- generic “rule engines”
- generic “card effect systems”
- “flexible frameworks” anticipating unknown rules
- preserving semantically wrong shortcuts because they are already implemented
- trait-object heavy orchestration where a small enum or explicit module split would be clearer

Unless justified, prefer **specific behavior over speculative infrastructure**.

---

## Review Procedure

Follow this sequence.

### Step 1 — Describe the change

Write a short description of what the change introduces.

Focus on behavior, not implementation details.

---

### Step 2 — Identify domain concepts involved

List the domain terms, aggregates, and contexts involved.

---

### Step 3 — Evaluate review dimensions

Check the change against:

- ubiquitous language
- bounded contexts
- aggregate ownership
- invariant protection
- slice minimality
- architectural leakage
- future assumption risk

---

### Step 4 — Identify issues

Classify issues as:

- **blocking** (must be corrected)
- **warning** (should be reviewed)
- **acceptable simplification**

---

### Step 5 — Recommend adjustments

If issues exist, suggest the smallest possible correction that restores DDD integrity.

Avoid proposing large architectural refactors unless strictly necessary.

Prefer:

- collapsing duplicate actions into the canonical domain action
- enriching existing events instead of creating parallel near-duplicate events

---

## Expected Output Format

When using this skill, produce output in the following structure:

### Change Summary

Short description of the change being reviewed.

### Domain Concepts Involved

List aggregates, contexts, and key domain terms involved.

### DDD Review

For each dimension:

- Ubiquitous Language
- Bounded Context Integrity
- Aggregate Ownership
- Invariant Protection
- Slice Minimality
- Architectural Leakage
- Future Assumption Risk

Provide a short assessment.

### Issues

List any blocking or warning issues discovered.

### Recommended Adjustments

Suggest the smallest changes required to restore domain integrity.

### Verdict

One of:

- `DDD compliant`
- `DDD compliant with warnings`
- `DDD violations detected`

---

## Anti-Patterns

Do not:

- treat implementation style issues as domain violations
- enforce unnecessary abstractions
- introduce new domain concepts during review
- redesign architecture unnecessarily
- block progress due to minor naming differences

Focus only on **domain model integrity**.

---

## Notes for DemonicTutor

This project evolves through **small vertical slices applied to a DDD gameplay model**.

During reviews:

- prefer extending the `Game` aggregate rather than introducing new aggregates prematurely
- keep slices narrow and behavior-focused
- document simplifications explicitly
- ensure implemented rule support matches documentation
- prefer explicit module splits by capability before introducing more abstraction

The goal is **domain clarity and correctness**, not architectural purity.
