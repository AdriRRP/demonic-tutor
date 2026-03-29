# System Overview — DemonicTutor

This document describes the high-level architecture of the DemonicTutor system.

It explains:

- the main layers of the system
- how gameplay actions flow through the system
- how domain state and events are managed
- how analytics and replay capabilities are derived

Detailed design decisions are documented in ADRs.

For a human-first explanation of the main runtime abstractions used by the current engine, see:

- `docs/architecture/runtime-abstractions.md`

---

# Architectural Principles

The system follows several core architectural principles:

- **Client-first architecture**  
- **Domain-Driven Design**
- **Event-driven state evolution**
- **Vertical slice development**
- **Deterministic domain logic**

These principles ensure that the system remains:

- predictable
- observable
- maintainable
- easy to evolve incrementally.

---

# System Layers

The architecture separates responsibilities into several conceptual layers.

```
UI
↓
Interface Adapters
↓
Application Layer
↓
Domain Core
↓
Event Store
↓
Projections
↓
Analytics

```

Each layer has a clear responsibility and dependency direction.

---

## UI Layer

The UI layer provides player interaction.

Responsibilities:

- presenting game state
- collecting player actions
- displaying statistics and projections
- triggering commands

The current repository now includes a browser-facing duel arena in `apps/web/`.
It is a thin Solid/Vite client that consumes the public gameplay contract through
WebAssembly, adds a small viewer-scoped private-hand overlay for hot-seat play,
and keeps gameplay rules inside the Rust application/domain layers.

The UI layer must **not contain business logic**.

It communicates with the application layer through explicit interface adapters.

---

## Interface Adapter Layer

The interface adapter layer bridges external clients into the internal public gameplay contract.

Responsibilities:

- exposing client-safe entrypoints
- translating transport or host-specific payloads
- serializing public snapshots and command results
- keeping client integration code out of domain and application modules

The current browser adapter lives in:

- `src/interfaces/web/`

This layer is intentionally thin.

It may depend on the application layer, but it must not own gameplay rules.

---

## Application Layer

The application layer orchestrates interactions between the UI and the domain.

Responsibilities:

- receiving commands
- validating command context
- invoking domain aggregates
- coordinating event persistence
- publishing domain events

The application layer may internally split public command handlers and event-adapter helpers by domain capability while keeping a small public service facade.

This layer acts as the boundary between the UI and the domain model.

The current public client-facing contract remains concentrated in:

- `src/application/public_game/`

Browser-specific glue belongs in `src/interfaces/`, not in that contract module.

---

## Domain Core

The domain core contains the gameplay model.

Responsibilities:

- enforcing domain invariants
- modeling game state
- processing commands
- producing domain events

Key properties:

- deterministic
- independent from infrastructure
- explicit domain language

Aggregates enforce gameplay rules and maintain consistent domain state.

When the domain core uses compact internal carriers such as handles, aggregate location indexes, or in-flight spell payloads, the intended explanatory reference is:

- `docs/architecture/runtime-abstractions.md`

---

## Event Store

The event store persists domain events.

Responsibilities:

- storing gameplay event history
- enabling deterministic replay
- reconstructing game state from events

The event store represents the **source of historical truth** for game sessions.

Early versions may use an in-memory implementation.

---

## Projections

Projections derive read models from domain events.

Responsibilities:

- building queryable state views
- preparing UI-friendly representations
- enabling efficient statistics calculation

Projections do not modify domain state.

They are derived views of the event stream.

---

## Analytics / Read Side

Analytics currently extract insights from gameplay events through projections and other read-side processing.

Examples:

- deck performance statistics
- draw distributions
- turn progression analysis
- replay inspection

Analytics operate on projections or event streams.

They remain separate from gameplay rule enforcement. In the current repository, this is primarily an observational read side rather than a fully separate bounded context.

---

# Command → Event Flow

Gameplay evolves through commands and events.

```
Player Action
↓
Command
↓
Application Layer
↓
Aggregate
↓
Domain Event(s)
↓
Event Store
↓
Projections
↓
UI / Analytics

```

Steps:

1. The player performs an action in the UI.
2. The UI issues a **command**.
3. The application layer routes the command to the domain.
4. The aggregate evaluates the command and enforces rules.
5. The domain produces **events** describing what occurred.
6. Events are persisted in the event store.
7. Projections update derived views.
8. The UI and analytics consume these projections.

This event flow makes gameplay:

- observable
- replayable
- analyzable.

---

# Vertical Slice Evolution

The system evolves through **vertical slices**.

Each slice introduces:

- a coherent gameplay behavior
- new commands and events
- minimal domain extensions
- focused tests

Slices must remain:

- small
- deterministic
- reviewable.

When a slice materially changes a core runtime abstraction, also update:

- the relevant implemented slice document
- `docs/architecture/runtime-abstractions.md` if the human explanation changed

Examples of slices include:

- `StartGame`
- `DrawOpeningHands`
- `PlayLand`
- `AdvanceTurn`

Slice documentation lives in:

```
docs/slices/

```

---

# Domain Boundaries

Several boundaries are enforced across the architecture.

The domain core:

- must not depend on UI
- must not depend on network or storage
- must remain deterministic

Aggregates:

- enforce gameplay invariants
- produce domain events
- do not publish events directly

Infrastructure concerns remain outside the domain model.

---

# Client-Side Execution Model

DemonicTutor is designed to run primarily in the browser.

The domain core is compiled to **WebAssembly**.

This architecture provides:

- low operational complexity
- offline playtesting capability
- deterministic local execution

Browser constraints influence several design decisions:

- limited multithreading
- deterministic execution
- minimal infrastructure dependencies.

---

# Future Evolution

As the system matures, additional capabilities may be introduced:

- richer phase and priority handling
- stack-aware gameplay
- advanced replay inspection
- multiplayer synchronization
- deeper analytics

These extensions should preserve the core architectural principles.

---

# Relationship to Other Documents

This document describes **system structure**.

Related documents define other aspects of the project:

| Document | Responsibility |
|--------|-------------|
| `PROJECT.md` | project vision |
| `CONSTRAINTS.md` | architectural limits |
| `docs/domain/` | domain model |
| `docs/architecture/vertical-slices.md` | slice strategy |
| `docs/architecture/adr/` | architecture decisions |
| `docs/development/development.md` | development guidelines |
