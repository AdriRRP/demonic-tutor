# Project — DemonicTutor

## Vision

DemonicTutor is a lightweight client-side application for observing and analyzing Magic: The Gathering decks through real play sessions.

The system records gameplay as explicit domain events, allowing sessions to be replayed, inspected and analyzed.

It serves two purposes:

- a practical **deck playtesting laboratory**
- a **technical learning project** for Rust, WebAssembly, Domain-Driven Design and event-driven architecture

---

# Product Identity

DemonicTutor should feel like:

- a **laboratory for decks**
- a **rules-aware playtesting environment**
- an **observable gameplay engine**
- a **replayable and analyzable system**

It should **not** feel like:

- a generic card platform
- a backend-heavy service
- a monolithic simulator attempting to implement all of Magic

The system favors **precision, observability and architectural clarity** over feature breadth.

---

# Core Capabilities

The system is designed to support the following capabilities:

- representing game sessions through explicit domain state
- processing player intent through commands
- producing domain events as factual history
- replaying sessions from event history
- deriving gameplay statistics from real play

These capabilities enable both **live analysis during a game** and **post-game inspection**.

---

# Engineering Goals

DemonicTutor is also intended as a serious engineering practice project.

It exercises:

- Rust for domain modeling
- WebAssembly for client-side execution
- tactical Domain-Driven Design
- event-driven application design
- behavior-driven testing for observable gameplay
- controlled use of agent-assisted development

---

# Product Philosophy

The project prioritizes:

- **correctness over breadth**
- **clarity over cleverness**
- **explicit modeling over hidden behavior**
- **incremental delivery over speculative architecture**

The system should grow through small, coherent vertical slices.

---

# Initial Scope

The initial stage of the project intentionally focuses on foundations.

The repository should first establish:

- a clear project vision
- explicit project constraints
- a minimal ubiquitous language
- a stable starting point for domain modeling

Subsequent milestones introduce:

- bounded contexts
- aggregates
- commands and domain events
- an initial vertical slice such as `StartGame`
- an in-memory event store and event bus
- basic projections
- initial BDD scenarios

---

# Non-Goals (Initial Stage)

In its early stages, DemonicTutor is **not intended to be**:

- a complete implementation of the Magic Comprehensive Rules
- a full card database system
- a collection manager
- a marketplace
- a social platform
- a fully autonomous AI-driven system

The focus remains on **clear domain modeling and observable gameplay behavior**.

---

# Modeling Direction

The gameplay model evolves incrementally.

Only the rule subset required by the current milestone should be implemented.

Official Magic rules inform the model, but the system must **never imply full rules coverage unless it is explicitly supported**.

---

# Long-Term Direction

Once the core architecture is stable, future iterations may introduce:

- richer phase and priority handling
- stack-aware interactions
- replay browsing tools
- deck comparison workflows
- multiplayer synchronization
- deeper gameplay analytics
- AI-assisted analysis of gameplay logs

---

# Definition of Success — Phase 0

Phase 0 is considered successful when the project provides:

- a stable project identity
- a clear written vision
- explicit architectural constraints
- a coherent initial ubiquitous language
