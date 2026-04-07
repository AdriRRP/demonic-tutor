# apps/web — DemonicTutor duel arena

This directory contains the current browser-facing client for DemonicTutor.

It is intentionally thin:

- Solid for UI composition
- Vite for dev/build
- the real Rust engine embedded through WebAssembly

Gameplay rules do not live here.

The client consumes the public gameplay contract exported by the Rust crate through the browser adapter in:

- `src/interfaces/web/`

For the architectural rationale, see:

- `docs/architecture/web-client.md`

## Commands

Install dependencies:

```bash
cd apps/web
npm install
```

Start the development server:

```bash
cd apps/web
npm run dev
```

Build the client:

```bash
cd apps/web
npm run build
```

The web scripts regenerate the wasm package from the repository root crate before bundling.

Quality commands:

```bash
cd apps/web
npm run format
npm run format:check
npm run lint
npm run typecheck
npm run build
npm run check
npm run audit
npm run deps:check
```

`npm run check` is the frontend quality gate used by CI: format, strict lint, production build, `npm audit`, and dependency freshness against the pinned manifest.

`deps:check` only fails when the installed frontend dependencies are behind the exact versions pinned in `package.json`. Newer releases are reported for Dependabot to handle without leaving CI permanently red.

The repository CI now treats these frontend checks as first-class quality gates, and Dependabot also tracks `apps/web` dependencies directly.

## Current Scope

Today this app is a playable two-player arena with a board-first tabletop presentation, oriented around one local seat per browser instance and a same-origin local duel room across two browser windows.

It currently provides:

- one shared wasm-backed game session
- one same-origin `BroadcastChannel` bridge so a second browser window can join the same duel without a backend
- a host-authoritative local room model where one window owns the Rust runtime and the peer window sends public commands to it
- a manual remote-pairing modal that can establish a direct WebRTC data channel between two browsers through copy-pasted offer/answer payloads
- a first host-authoritative WebRTC command relay so the paired peer can issue the existing public commands through the host browser runtime
- authoritative public state broadcast back to the paired peer so both browsers converge from the same host-owned state
- explicit one-seat-per-device remote ownership, with both browsers rejecting cross-seat command attempts before they reach the host runtime
- viewer-scoped WebRTC payloads so the peer only receives its own hand and prompts in clear while the opposing viewer stays redacted
- reconnect-aware WebRTC pairing state plus fresh authoritative resync when the remote channel recovers after a transient interruption
- a generated duel HUD with a graphical phase track and compact stat pips instead of the earlier text-heavy cockpit
- two viewer-scoped seats over that same Rust-owned state
- a viewport-fitted SPA arena with dedicated landscape and portrait layouts
- a battlefield-first play surface with a clear opponent/player split
- a dedicated left rail that now owns player identity, life, hand count, and the local `Pass / Concede` actions
- a draggable mana-pool dock beside each avatar that appears only when mana exists and breaks the pool down by color, including colorless
- a priority halo around the speaking player's avatar so turn conversation stays legible without reintroducing textual badges
- a selected-card highlight shared across hand hover, inspect detail, and battlefield action focus
- a hidden opponent hand fan rendered with simplified classic-inspired generated card backs instead of text counters
- a local bottom hand fan that can be dragged onto the battlefield for simple legal plays
- zone-aware face-up card rendering so battlefield and stack previews read as header-plus-fullart, while hand and inspect cards keep the fuller classic layout with mana symbols
- a locally rearrangeable battlefield so permanents already in play can be positioned freely inside the owning seat
- generated card piles for library, graveyard, and exile using simplified classic-inspired CSS-built backs and compact face-up zone tops
- a stack dock that only appears when the stack has objects and opens a dedicated modal for detailed resolution reading
- focused zone browsers that open on demand instead of permanently occupying the table
- card inspection modals so the card itself is now the primary interaction object
- a modal replay log instead of a persistent sidebar dashboard
- prompts and choices anchored near the seat area they belong to
- a more spatial combat lane for attackers and blockers
- real command execution for land play, mana, simple casting, combat, cleanup, and replay

It is still an early arena rather than a polished shipped client, but the UI now prioritizes a card-first premium table feel before adding richer motion or deeper spell UX.

## Remote Pairing Foundation

The current remote multiplayer baseline is still intentionally narrow:

- it establishes browser-to-browser transport only
- it now also relays the peer's public gameplay commands through the authoritative host
- it uses manual WebRTC signaling through a pairing modal
- it now also broadcasts authoritative public state back from the host to the peer
- it remains honest about transport state (`idle`, `connecting`, `connected`, `failed`)

It is still not the final remote product shape:

- reconnect and host-loss handling are still future slices
- the authoritative host still sees the full runtime and therefore remains a trusted participant, not a hostile-client-secure server

## Local Two-Window Multiplayer

The current multiplayer slice is intentionally narrow:

- it is local and same-origin only
- it does not require a backend
- it depends on duplicating or sharing the same room URL between two browser windows
- the host window must remain open because it owns the wasm-backed authoritative runtime

Practical flow:

1. open the app in one browser window
2. copy the room link from the cockpit
3. open that same link in a second browser window
4. the first window becomes the host and the second becomes the peer

Honesty note:

- this is not hidden-information-safe remote multiplayer yet; both same-origin windows still live in a trusted local environment

## Guardrails

- do not duplicate gameplay rules in TypeScript
- do not invent a second public contract inside the client
- keep browser-specific Rust glue in `src/interfaces/web/`
- prefer small vertical slices over broad frontend framework work
