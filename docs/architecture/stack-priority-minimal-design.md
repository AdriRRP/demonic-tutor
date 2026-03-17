# Stack and Priority — Minimal Design

## Purpose

This document proposes the first coherent design for introducing stack and priority into DemonicTutor.

It is a design note, not an accepted architectural decision yet.

The goal is to make the next gameplay expansion deliberate, reviewable, and incremental.

---

## Why This Design Exists Now

The current gameplay core is already rich enough that further growth without stack and priority would start to distort real Magic semantics.

Today the model still simplifies these areas:

- spell cards resolve immediately after casting
- explicit draw effects are direct commands rather than stack objects
- players do not pass priority
- steps and phases advance without stack-aware windows

At this point, adding more spell-like behavior without stack and priority would increase semantic debt faster than it would increase value.

---

## Normative Rules Areas

This design is based on the official Magic Comprehensive Rules published by Wizards and currently linked from the official rules page.

Relevant rules areas for the minimal design are:

- 117 — timing and priority
- 405 — stack
- 601 — casting spells
- 608 — resolving spells and abilities
- 704 — state-based actions

Repository rule notes remain the local source of truth once the slice is implemented.

---

## Design Goals

The first implementation of stack and priority should:

- make `CastSpell` semantically real
- model a deterministic two-player priority loop
- introduce a real stack zone for spells
- resolve the top object only after both players pass in succession
- keep state-based action review centralized
- remain readable and wasm-friendly

It should **not** try to implement the full Magic timing engine in one move.

---

## Non-Goals For The First Iteration

The first stack/priority implementation should explicitly leave out:

- triggered abilities
- activated abilities on the stack
- replacement effects
- targeting
- modes
- responses with cards other than simple spell casting
- combat tricks beyond what falls out naturally from instant-speed casting
- multiplayer priority rules
- APNAP handling beyond the current two-player deterministic turn order

---

## Core Modeling Decision

The first stack/priority implementation should stay **inside the `Game` aggregate**.

It should not introduce a separate “rules engine” aggregate or a generic interpreter.

The right level of abstraction is:

- explicit aggregate state
- explicit commands
- explicit events
- small internal transition functions

This keeps legality, determinism, and replayability under one authority.

---

## Recommended Internal Model

### Stack Zone

Add a dedicated stack zone to the aggregate state:

```rust
pub struct StackZone {
    objects: Vec<StackObject>,
}
```

Use `Vec` with push/pop semantics.

Reason:

- LIFO behavior is natural
- memory footprint stays low
- the code remains obvious

### Stack Object

Model stack entries as a closed enum, not trait objects:

```rust
pub enum StackObjectKind {
    Spell(SpellOnStack),
}

pub struct StackObject {
    id: StackObjectId,
    controller_id: PlayerId,
    source_card_id: CardInstanceId,
    kind: StackObjectKind,
}
```

For the first slice, the only supported stack object kind should be `Spell`.

Reason:

- enums are clearer than trait hierarchies here
- no dynamic dispatch needed
- easy to extend later to abilities
- easier to serialize and replay

### Spell On Stack

Represent the pending spell with the minimum data needed to resolve it later:

```rust
pub struct SpellOnStack {
    card_type: CardType,
    mana_cost_paid: u32,
}
```

Do **not** duplicate full card definitions or speculative text-box behavior.

### Priority State

Model priority explicitly as a small state machine:

```rust
pub struct PriorityState {
    current_holder: PlayerId,
    passes_in_row: u8,
}
```

Semantics:

- `current_holder` is the player who may act now
- `passes_in_row` counts consecutive passes since the last stack-changing action

This is intentionally simpler than encoding a more abstract turn-order engine.

### Aggregate Additions

Conceptually, `Game` grows with:

- `stack: StackZone`
- `priority: Option<PriorityState>`

`priority: None` means the game is in a state without an open priority window.

For example:

- no priority during untap
- no priority while resolving a stack object

---

## Recommended Domain Commands

The first stack/priority epic should introduce these commands incrementally:

### Slice 1

- `OpenPriorityWindowCommand` should **not** exist publicly

Opening a priority window should be an internal aggregate transition tied to steps and spell casting.

### Slice 2

- `PassPriorityCommand { player_id }`

This becomes the public way to say “I take no further action in this window.”

### Existing Command Reinterpretation

- `CastSpellCommand` remains the public casting intent

But its meaning changes from:

- validate, spend mana, resolve immediately

to:

- validate, spend mana, remove card from hand, put a spell object on the stack, hand priority to the next player

This is the canonical path and avoids duplicate “cast to stack” commands.

---

## Recommended Domain Events

The first stack/priority epic should add explicit events rather than hiding the new semantics behind old ones.

Recommended new events:

- `PriorityPassed`
- `SpellPutOnStack`
- `StackTopResolved`

Recommended event policy:

- `SpellCast` should no longer mean “the spell fully resolved”
- `SpellCast` should be superseded or narrowed once stack support is introduced

The cleaner option is:

- `SpellPutOnStack` for casting completion
- `SpellResolved` or reuse `StackTopResolved` with payload for actual resolution result

That separation is semantically much healthier than stretching `SpellCast` to cover both “was cast” and “already resolved.”

---

## Resolution Semantics

When both players pass in succession:

- if the stack is not empty, resolve the top object
- if the stack is empty, the current step or phase may advance

This rule should be implemented explicitly in one place, not re-derived ad hoc in command handlers.

Recommended internal transition:

```rust
fn handle_priority_pass(...) -> PriorityPassOutcome
```

where the outcome may be:

- priority moves to the next player
- top stack object resolves
- step/phase becomes eligible to advance

---

## State-Based Actions Interaction

State-based actions should remain centralized in the shared SBA review module.

Priority integration rule:

- each time the game is about to grant priority, it must first run supported SBA review

That matches the current direction of the model and avoids scattering automatic consequences again.

Do **not** build a generic recursive engine yet.

The first implementation should simply:

1. run supported SBA review
2. if a terminal state happened, stop
3. if stack/trigger work remains unsupported, continue with the current supported subset
4. then grant priority

---

## Turn Flow Integration

The existing phase state pattern should stay.

Do **not** replace it with a unified mega state machine.

Instead:

- phases continue to define turn-based automatic behavior
- priority windows become an orthogonal runtime concern layered on top of the current phase model

This is the cleanest and most maintainable split:

- **phase behavior** decides what turn-based actions happen
- **priority state** decides who may act now
- **stack zone** decides what remains to resolve

---

## Recommended Module Shape

When implementation starts, the most coherent internal organization would be:

```text
src/domain/play/game/
  model/
    player.rs
    priority.rs
    stack.rs
  rules/
    stack_priority/
      mod.rs
      casting.rs
      passing.rs
      resolution.rs
```

This keeps:

- aggregate state in `model`
- legality and transitions in `rules`

It also avoids dumping stack logic into `resource_actions.rs` or `turn_flow.rs`.

---

## Patterns To Use

### Prefer

- explicit state machine for priority
- closed enums for stack object kinds
- small outcome enums/structs for command results
- deterministic transition helpers
- single ownership by `Game`

### Avoid

- trait-object based rule engines
- visitor-heavy abstractions
- generic effect interpreters
- trying to solve abilities, spells, and triggered effects with one abstraction on day one

The most idiomatic Rust design here is:

- enums for domain variants
- structs for aggregate-owned state
- modules of pure transition functions

That is both elegant and maintainable.

---

## Slice Plan

### Slice A — Stack Foundation

- add stack zone and priority state to `Game`
- add internal priority opening rules
- no public interaction yet except what is needed to keep the aggregate coherent

### Slice B — Cast Spell Goes On Stack

- `CastSpell` no longer resolves immediately
- it becomes `SpellPutOnStack`
- active player retains or passes according to the priority rules

### Slice C — Pass Priority

- add `PassPriorityCommand`
- if both players pass with stack non-empty, resolve top object
- if both players pass with stack empty, the current step/phase can advance

### Slice D — Spell Resolution Through Stack

- permanent spells enter battlefield on resolution
- instants and sorceries go to graveyard on resolution
- reuse current state-based action review after resolution

### Slice E — Turn Flow With Priority Windows

- update phase progression to stop assuming direct advancement after every action
- make `AdvanceTurn` valid only when the step is actually ready to end

This decomposition is the smallest path that stays semantically true.

---

## Risks

Main risks:

- overbuilding a generic rules engine too early
- leaving old direct-resolution events alive beside new stack semantics
- mixing phase progression and priority into one tangled state machine
- introducing hidden special cases for spells instead of modeling stack objects cleanly

The design above avoids those risks by keeping the first iteration narrow.

---

## Recommendation

The next work on stack and priority should begin with:

1. a proposal slice for minimal stack and priority
2. proposed Gherkin features
3. then a first implementation slice limited to “cast spell goes on stack” plus “pass priority resolves top object”

That is the highest-signal path that stays elegant, Rusty, and reviewable.
