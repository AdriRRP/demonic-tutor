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

Today this app is a playable two-player hot-seat arena with a board-first tabletop presentation.

It currently provides:

- one shared wasm-backed game session
- two viewer-scoped seats over that same Rust-owned state
- a viewport-fitted SPA arena with dedicated landscape and portrait layouts
- a battlefield-first play surface with a clear opponent/player split
- a collapsible bottom hand fan that can be dragged onto the battlefield for simple legal plays
- visible library and graveyard anchors that open zone browsers instead of permanently occupying the table
- card inspection modals so the card itself is now the primary interaction object
- a modal replay log instead of a persistent sidebar dashboard
- explicit pass-the-device handoff, with only one private hand open at a time
- prompts and choices anchored near the seat area they belong to
- a more spatial combat lane for attackers and blockers
- real command execution for land play, mana, simple casting, combat, cleanup, and replay

It is still an early arena rather than a polished shipped client, but the UI now prioritizes a card-first premium table feel before adding richer motion or deeper spell UX.

## Guardrails

- do not duplicate gameplay rules in TypeScript
- do not invent a second public contract inside the client
- keep browser-specific Rust glue in `src/interfaces/web/`
- prefer small vertical slices over broad frontend framework work
