# Vertical Slices — DemonicTutor

This document defines the strategy used to evolve the DemonicTutor system.

The project grows through **vertical slices**.

A vertical slice is a minimal end-to-end implementation of a coherent gameplay capability.

Slices allow the system to evolve safely while keeping the domain model precise and understandable.

---

# What Is a Vertical Slice

A vertical slice introduces a **complete behavior** that spans the system.

A slice typically includes:

- a domain command
- domain validation
- domain state transitions
- domain events
- optional projections or read models
- tests validating the behavior

A slice should represent something that can be **observed in gameplay**.

Examples:

- starting a game
- drawing cards
- playing a land
- casting a spell

Slices represent **capabilities**, not technical layers.

---

# Why Vertical Slices

The vertical slice approach provides several advantages.

### Incremental Domain Modeling

The domain model grows only when behavior requires it.

This prevents speculative abstractions.

---

### Architectural Safety

Each slice introduces a small amount of change.

This reduces the risk of large refactors.

---

### Observability

Every slice produces domain events.

This makes behavior observable, replayable and analyzable.

---

### Testability

Slices are easy to test because they represent isolated capabilities.

---

# Structure of a Slice

A slice typically introduces the following elements.

### Command

Represents the intent to perform a domain operation.

Example:

```
StartGameCommand
PlayLandCommand
PlayCreatureCommand
CastSpellCommand

```

---

### Domain Logic

The aggregate evaluates the command and enforces invariants.

---

### Events

If the command succeeds, one or more events are emitted.

Examples:

```
GameStarted
LandPlayed
CreatureEnteredBattlefield
CardDrawn
SpellCast

```

Events represent **facts that have already occurred**.

---

### Optional Projections

Some slices introduce read models derived from domain events.

Examples:

- game log timeline
- statistics
- replay models

Projections never influence domain legality.

---

# Slice Boundaries

Each slice must introduce **exactly one new gameplay capability**.

A slice must not introduce:

- multiple unrelated mechanics
- speculative infrastructure
- unused abstractions

If a change feels large, it should probably be split into multiple slices.

---

# Domain Discipline

Slices must follow strict domain discipline.

Prefer:

- explicit commands
- explicit domain events
- deterministic behavior
- small domain extensions

Avoid:

- generic rule engines
- speculative mechanics
- implicit state transitions
- infrastructure leaking into domain code

---

# Relationship With the Aggregate

Most slices extend the behavior of the `Game` aggregate.

The aggregate remains responsible for:

- enforcing gameplay invariants
- validating commands
- producing domain events

Slices must never bypass the aggregate.

---

# Relationship With Bounded Contexts

Slices usually operate inside a single bounded context.

Most gameplay slices belong to:

```
Play Context

```

Other contexts may observe slices through events.

Example:

```
Play → emits events → Analytics

```

---

# Lifecycle of a Slice

A slice typically progresses through these stages.

### Proposal

A slice is proposed and described conceptually.

---

### Implementation

The domain behavior is implemented.

Commands, events and tests are introduced.

---

### Documentation

The slice is documented under:

```
docs/slices/

```

---

### Integration

The slice becomes part of the playable system.

---

# Designing New Slices

When introducing a new slice, ask:

1. What gameplay capability does this introduce?
2. Which invariants must be enforced?
3. Which domain events describe the outcome?
4. Which parts of the model must evolve?

If the slice requires large model changes, it should be split.

---

# Example Slice Evolution

A simplified slice evolution might look like:

```
StartGame
↓
DrawOpeningHands
↓
PlayLand
↓
AdvanceTurn
↓
TapLand
↓
CastSpell
↓
PlayCreature
↓
RemoveSummoningSickness

```

Each step introduces a single new capability.

---

# Long-Term Strategy

The system should evolve through **small, observable domain capabilities**.

Future slices may include:

- creature combat and damage
- declare attackers step
- declare blockers step
- stack resolution
- triggered abilities

These should be introduced incrementally.

---

# Guiding Principle

The system grows by **behavior first**, not by architecture first.

Slices introduce behavior.

Architecture exists to support those behaviors.
