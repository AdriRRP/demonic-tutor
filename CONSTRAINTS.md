# Constraints — DemonicTutor

This document defines the non-negotiable constraints that shape the project.

Constraints exist to preserve:

- domain integrity
- architectural coherence
- operational simplicity
- long-term maintainability

They are stronger than guidelines and should only change through explicit architectural decisions.

---

# Product Constraints

The application is designed as a lightweight, client-first system.

- The application must work primarily as a **client-side application**.
- The application must remain **deployable as a static web application**.
- Operational complexity must remain **minimal by design**.
- The system prioritizes **speed, precision, and clarity** over feature breadth.
- User experience may evolve gradually, but **the domain model must remain coherent**.

---

# Domain Constraints

The project models a **subset of Magic gameplay**, expanded incrementally.

- The system must **not attempt to model the full Magic ruleset from the start**.
- Only the rules required by the **current vertical slice** may be implemented.
- Domain behavior must be **explicit, observable, and traceable**.
- Unsupported rules must **never be implied as implemented**.
- Card-specific complexity should be postponed unless it is **required for a slice**.

---

# Modeling Constraints

The domain model must remain **clear, explicit, and language-driven**.

- The **ubiquitous language** must remain consistent across code and documentation.
- The domain model must be driven by **explicit domain concepts**, not UI convenience.
- Modeling choices must clearly distinguish **rules interpretation** from **implementation simplification**.
- Observable gameplay behavior has priority over speculative abstractions.
- The project must avoid **premature modeling of rare edge cases**.

---

# Architectural Constraints

The architecture enforces strict separation of concerns.

- No business logic may live in the **UI layer**.
- The **domain core must not depend on infrastructure** (storage, network, rendering).
- The domain model must remain **deterministic**.
- **Aggregates must not publish events directly**.
- Event publication is handled outside the aggregate.
- Analytics and telemetry must remain **separate from gameplay rules**.
- Concurrency is an **optimization**, not a requirement for correctness.
- The system must function correctly **without parallel execution**.

---

# Technology Constraints

Technology choices support the client-first architecture.

- **Rust** is the main language for the domain core.
- The core must be able to compile to **WebAssembly**.
- The design must remain compatible with **browser execution constraints**.
- The system must not assume **multithreaded browser execution** by default.
- Infrastructure choices should remain **simple until complexity is justified**.

---

# Testing Constraints

Testing must validate observable domain behavior.

- Important domain behavior must be **testable in isolation**.
- Tests should validate **observable behavior**, not internal implementation details.
- Vertical slices should include **focused tests before expanding scope**.
- Overly broad scaffolding without verification should be avoided.

---

# Development Constraints

The repository evolves through **small, coherent changes**.

- Architectural decisions with lasting impact should be **recorded explicitly** (ADR).
- The system should evolve through **incremental vertical slices**.
- Small, reviewable changes are preferred over large speculative refactors.
- New domain concepts should only appear when they **solve a real modeling problem**.
- The repository must remain understandable **without relying on agent memory**.

---

# Agent Constraints

Agents assist development but do not define project truth.

- Agents are **contributors**, not authorities on domain behavior.
- Agents must work from **canonical project documentation**.
- Agents must not silently redefine **scope, architecture, or rules support**.
- Agent outputs must remain **reviewable, constrained, and incremental**.

---

Changes to this document should normally require an ADR.
