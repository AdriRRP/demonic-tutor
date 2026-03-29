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

`deps:check` only fails when the installed frontend dependencies are behind the exact versions pinned in `package.json`. Newer releases are reported for Dependabot to handle without leaving CI permanently red.

The repository CI now treats these frontend checks as first-class quality gates, and Dependabot also tracks `apps/web` dependencies directly.

## Current Scope

Today this app is a playable two-player hot-seat arena.

It currently provides:

- one shared wasm-backed game session
- two viewer-scoped seats over that same Rust-owned state
- private hand reveal per seat for pass-the-device play
- real command execution for land play, mana, simple casting, combat, cleanup, and replay

It is still a debugging-forward arena rather than a polished shipped client.

## Guardrails

- do not duplicate gameplay rules in TypeScript
- do not invent a second public contract inside the client
- keep browser-specific Rust glue in `src/interfaces/web/`
- prefer small vertical slices over broad frontend framework work
