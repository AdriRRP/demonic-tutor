# Web Client Architecture — DemonicTutor

This document describes how the browser client fits into the DemonicTutor architecture.

It is intentionally narrow:

- what lives in `apps/web`
- what lives in the Rust crate
- how the browser talks to the gameplay core
- where UI responsibilities stop

For the broader system picture, see:

- `docs/architecture/system-overview.md`

---

# Current Structure

The browser-facing stack is split across two clear boundaries:

- `apps/web/`
  the Solid + Vite client
- `src/interfaces/web/`
  the wasm interface adapter exported by the Rust crate

The gameplay contract consumed by the browser remains owned by:

- `src/application/public_game/`

That application boundary projects the aggregate into:

- public game snapshots
- legal actions
- choice requests
- deterministic command results
- persisted replay log entries

The web client consumes that contract.

It does not reimplement gameplay rules in TypeScript.

---

# Responsibility Split

## Rust owns

- gameplay rules
- command execution
- event persistence
- public read-model projection
- replay timeline production
- viewer-scoped visibility rules

## TypeScript owns

- layout
- interaction flow
- local presentation state
- visual ordering and composition
- browser-only concerns such as focus, gestures, and component state

The web client may help the player choose among already-exposed legal actions, but it must not invent legality or resolve rules locally.

---

# Adapter Boundary

`src/interfaces/web/wasm.rs` is the browser-specific adapter layer.

Its job is to:

- expose a wasm-safe client API
- translate browser calls into public commands
- serialize public snapshots and replay data into wasm-friendly payloads

It is intentionally thin.

The adapter may compose:

- `GameService`
- `PublicGameCommand`
- `game_view`
- `legal_actions`
- `choice_requests`
- `public_event_log`

But it should not become a second application layer.

If browser-facing concerns grow, they should still stay in `src/interfaces/web/` rather than leaking back into the domain or into `src/application/public_game/`.

---

# Browser Flow

The current browser flow is:

```text
Solid UI event
-> wasm adapter call
-> public command execution
-> aggregate mutation + persistence
-> public snapshot / command result / replay log
-> Solid render update
```

This keeps one authoritative runtime.

There is no duplicated “frontend engine”.

When the local duel room bridge is active across two same-origin browser windows, the flow becomes:

```text
Peer Solid UI event
-> local BroadcastChannel request
-> host window wasm adapter call
-> public command execution
-> aggregate mutation + persistence
-> host broadcasts updated public snapshot
-> both windows render from the shared host-owned state
```

That local transport still keeps one authoritative runtime.

---

# Monorepo Decision

The browser client lives in the same repository as the Rust engine.

That is intentional for the current stage because it keeps:

- the gameplay contract close to the client that consumes it
- replay fixtures and integration slices easy to evolve together
- wasm compatibility visible in normal development flow
- UI experiments grounded in real engine behavior instead of mocks

The architectural decision behind this is recorded in:

- `docs/architecture/adr/0016-browser-client-stays-in-monorepo-and-uses-a-wasm-interface-adapter.md`

---

# Current Scope

The current `apps/web` client is now a playable two-player arena that supports both hot-seat and same-origin two-window local duel rooms.

Its job today is to provide:

- one shared Rust-owned game session embedded in the browser
- one same-origin `BroadcastChannel` bridge so a second browser window can join that session without a backend
- a host-authoritative browser room where only one window owns the wasm-backed engine at a time
- a generated duel HUD that renders the phase loop, active player, priority holder, and compact seat stats through CSS/SVG primitives instead of text-heavy badges
- two viewer-scoped seats over that same session
- a viewport-fitted SPA arena with portrait and landscape layouts
- a battlefield-first layout with a clear top/bottom duel split
- a shared selected-card highlight spanning hand focus, inspection, and battlefield action focus
- a hidden opponent hand fan built from simplified classic-inspired generated card backs so rival hand size reads as physical cards instead of a lone counter
- a collapsible bottom hand fan that can be dragged onto the battlefield for simple legal plays
- a locally rearrangeable battlefield where permanents already on the table can be dragged to presentation-only positions inside the owning seat
- generated zone piles for library, graveyard, and exile using simplified classic-inspired CSS-built card backs and compact face-up tops
- focused zone browsers that stay on demand instead of keeping textual zone panels on the table
- a handoff-first hot-seat flow where only one private hand is opened at a time
- card inspection modals so cards stay the main affordance rather than surrounding lists
- prompts placed near the seat zone they affect instead of inside a generic debug rail
- a battlefield-first combat lane for attackers and blockers
- modal replay/debug surfaces that stay discreet without turning the table into a dashboard
- real command execution for lands, mana, creature casting, combat, cleanup, and replay inspection
- a fallback back to hot-seat when no second local browser window is connected

It is still intentionally early-stage UI:

- focused on generating trustworthy play logs
- centered on a shared table surface rather than debug panels
- optimized for interaction coverage before deep motion/polish work
- not yet a secure remote multiplayer client
- still keeping free battlefield layout local to each browser window until a dedicated sync slice lands

Important constraint:

- the two-window room is designed for a trusted same-origin local setup; it does not treat private hands as secure against browser-side inspection

---

# Design Guardrails

The browser client should keep these rules:

- no gameplay legality in TypeScript
- no duplicated authoritative game state outside the Rust engine
- no new public contract invented inside `apps/web`
- browser adapters stay under `src/interfaces/`
- client features should grow through vertical slices just like the engine
