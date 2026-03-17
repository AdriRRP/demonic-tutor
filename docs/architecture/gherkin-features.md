# Gherkin Features for Gameplay Modeling

## Purpose

This document defines how DemonicTutor should use Gherkin scenarios to support gameplay modeling.

The goal is to add an executable-specification layer between:

- Magic rules references
- slice design
- implementation behavior
- regression coverage

This is intended to improve semantic precision without turning the full Comprehensive Rules into a direct implementation backlog.

---

## Recommendation

DemonicTutor should use a **hybrid approach**:

1. use the Magic Comprehensive Rules as the normative source
2. maintain a repository-owned rules map and focused rules notes
3. express supported product behavior as Gherkin features
4. implement each feature through one or more vertical slices

The project should **not** treat the Comprehensive Rules as a literal feature catalog.

---

## Why This Is Viable

This approach is viable because it matches the repository's existing structure:

- vertical slices already model small observable capabilities
- the `Game` aggregate already owns legality and state transitions
- domain events already expose behavior that can be asserted from scenarios
- the project already uses explicit out-of-scope boundaries

Gherkin adds value here by making behavior:

- more readable
- more traceable to rules
- easier to review with domain language
- easier to preserve during refactors

---

## What Gherkin Should Represent

In DemonicTutor, a feature should represent:

- observable gameplay behavior of the `play` bounded context
- current supported behavior or an explicitly proposed behavior
- domain-canonical actions and results

It should not represent:

- literal paragraphs copied from the Comprehensive Rules
- hidden implementation details
- broad rule areas that the project does not yet support
- speculative mechanics with no active slice

---

## Traceability Model

Use this traceability chain:

`Comprehensive Rules -> rules map / rules notes -> Gherkin feature -> slice(s) -> code/tests`

This means:

- rules remain the normative reference
- features remain product-facing behavior specifications
- slices remain the implementation units

---

## Scope Rules

Each feature should:

- focus on one coherent behavior
- describe only currently supported or actively proposed behavior
- name commands and outcomes using the ubiquitous language
- reference the relevant slice documents
- reference the relevant rules sections
- state out-of-scope behavior explicitly when omission could mislead

If one feature needs multiple unrelated mechanics, split it.

---

## Status Model

Each feature should declare one of these statuses:

- `implemented`
- `proposed`
- `historical`

Only `implemented` features should be treated as current behavior.

---

## Relationship With Slices

Features do not replace slices.

Use this division:

- **feature**: observable behavior specification
- **slice**: implementation increment that delivers or extends the behavior

One feature may be delivered by:

- one slice
- multiple slices over time

One slice may support:

- one feature
- one clearly delimited part of a larger feature

---

## Relationship With Rules Changes

If the Comprehensive Rules change, do not re-audit the whole repository blindly.

Instead:

1. update the relevant rules note or rules map entry
2. identify the affected feature files
3. review only the slices and code tied to those features
4. update behavior claims if support has changed

This keeps maintenance tractable.

---

## Authoring Rules

Write features at the level of domain behavior.

Prefer:

- `Given Alice is in FirstMain`
- `When Alice casts a creature spell with enough mana`
- `Then the card leaves her hand and enters the battlefield`

Avoid:

- rule-number-only wording
- internal method names
- implementation jargon
- steps that imply unsupported stack or priority behavior

---

## Pilot Scope

The repository should start with a small pilot:

- turn progression
- casting a creature spell
- combat damage marking

That pilot is now extended into a broader executable acceptance layer covering:

- turn flow
- spell semantics
- combat semantics
- lethal creature destruction
- cleanup-based damage removal
- cleanup hand-size discard

Current executable BDD coverage:

- `features/turn-flow/turn_progression.feature`
- `features/spells/cast_creature_spell.feature`
- `features/combat/combat_damage_marking.feature`
- `features/combat/creature_destruction.feature`
- `features/turn-flow/cleanup_damage_removal.feature`
- `features/turn-flow/cleanup_hand_size_discard.feature`
- `features/turn-flow/lose_on_empty_draw.feature`
- `features/life/lose_on_zero_life.feature`
- `tests/bdd.rs`

Conventional Rust behavior tests remain separate under the aggregated `tests/unit.rs` target.

---

## Recommended Repository Layout

```text
docs/rules/
  rules-map.md
  notes/
    turn-flow.md
    casting-spells.md
    combat.md

features/
  README.md
  turn-flow/
    turn_progression.feature
    cleanup_damage_removal.feature
    cleanup_hand_size_discard.feature
    lose_on_empty_draw.feature
  life/
    lose_on_zero_life.feature
  spells/
    cast_creature_spell.feature
  combat/
    combat_damage_marking.feature
    creature_destruction.feature
```

---

## Future Direction

The repository now includes an executable acceptance layer using `cucumber-rs`.

Next worthwhile steps include:

- mapping implemented slices to features explicitly
- deciding how much of the acceptance layer should remain in Cucumber versus conventional Rust tests
