# DemonicTutor

```text
----:--:-::::::::::::::::::::::::::::::::::::::::::::::-:::-::::::::::-::::-::::-:::-::::-:::::-::::
---:-:--::-::::::::::::::::::---=+++-.::::::::::::-:::::::::::::::::::-::::::::::::::::::::::::::::-
:-::----:::::::::::::::::-----===++*+*++++.:::::::::::-::::::@:::::::::-:::::::::::::-::::-:::::::::
::::.-:::::.:..:..::::--------==+*#%@%#*+++=:::::::::::::::::.#::::::-::::::::::::::::::-::::::::--:
:-:.-::::::..:::..:::--:-::---==+*#%###*++++=:#%#++=:::::::::.-::::::::::::::::::::::::::::::::::-::
::.::@:::::*+#%%%%.:..::::.::::=++-+*****+++==#..-+%--:.::::.::*::::::::::::::-::::::::::::::::::::-
::::.-::::=-#::::::-..::......--=:-+-*****++==..:--:...#:-:.:.:=:::::::::::::::::::::::::::::::-::::
:.::..#:-#:.:::::--..::.......::---=:++*#**=--....:......%#::.-+.:::::::::::::::::::::::::::::::::::
:::....:=......::...:.:...:....:.:..==**%@-++=----...=::*-...:-*-::::::::::::::::::::::-:::-::::::--
-..:..:-#***-::::::-:.:...::::-=-::++*#:-++=+=::------.......-+=::::::::::::::::::::::::::::::::::::
:-:......=:.:::::::.::...:..::=+=::::..-==#:++::--------:...-.---:::::::::::::::::::::::::-::::::::-
:-:.....:.:::::::::-:::.....:...::%:::##.:*++#:---:---------------:-:::::::::::::::::-:::::::::-::::
:::::::::::::::::::.:-:.....--::.:#+*.#-%**#**+=-:------:--------------:---:::-:::::::-:::::::::::-:
--::::::::::::::::::-::::::::.:::::#+=++++*+**+=---------------------------::--::--:::::-::::-::----
----::::-:-:::::::::.:-:::::::::::.+=#@=-+****=------------=------------=--::::-::::::::::::--------
------::-:::::-:::::.--:::::-::::::=#=@#*++**+=-------=---------------------:::----=--::::-----::---
:=--------:---:::::::--::::-::=:::.:.#*++++**=:-----------------=-------=----::======-----==---::---
------:--::::::::::::.---::--:::--:-+*++++==*=--------------------:----------::-=+*++*++=+++=-------
-----:--::::::::::::::----------::::-%@*-.==*+----------------------------------++**##@****++=------
------:.-::-:::::::::-:.-----:::----=+==++==*:----=-..---%:.----------:--------==*=**::::..:+-------
---.-:.----::-::::-:::::.:--------:---=+===+-@#-=--.+#%%@:::..-----------------::=-:+:::::=..-.:----
-::::.----:-:::::::-:-=-.:..:.=-==--=%**+*.@@+*#=-=.*##*#.:::.:.--------------::---:+:::-:...:::---=
::::::---:--::::::::----:::-:::....--==-.:*=#*--++.+**##:.:-:::::-----------:::=:--:::---::::::-:---
-::::::.--::::--::------::.::----::......*#=:-+*+-:**%%.::::::::::.-------::--::-----::--:::----:-:-
-:::::::......:.........-:::-:-=--::....**++===++:**#**.::::::--::::-----:-::-----+---:-:--:--------
::::::..::.:.........-:---:-=-------++++++++====:.+#*#..:::::::-:=:::.-----------=---:--:-----------
-:::::.:.....:::..:::-:------------=+++=++====--:*#*#+.:.:::::::::.::-.--=----==---------------=----
:.-:::::::::::..-:--:-------::---:+++++++++====.##+**.:::::::::::::--::-..----------=------------=-=
.::::::::::::.:-:-:::--:::-::@%%#*+++++++=+====:#*+*+.::::.::::.:.::-::...:.--------=======------=-=
:-:::::-::::::------:-:-----%%%*++=+==++==:==-.#*+++.:.:::::::::::::::--:::::--=-----==-----=------=
::-:---:::::.:----:---:-=--=##*-=============.:%*++:.:....:::::::::::.....:::.====-----------------=
..:-.::--::::.-----::---===+++--====::=======:%#+++..::..::.:::::::::::.-%:::::=----------------=--=
:-.:-:::*::.:*--------=-=++=+---======+=..==.###++.:::::::::::::::.:.:..*:::-:.=-------------------=
:...::::::...::#------==+==+-+:-===-====--...+#*+-..::::::::::::::::...-::::::.=-------=------------
..:.:+-...:----:-----=+==-+--::-===-=+=+===-:#+**....::..:..:.::::::....:::::::--=-==--------------=
...::::-=:---.:-..---==--+-::.:-=========+..::::-=++ ..:::::.::::.:..:.::::::::.--==--=-------------
:.:::::--:+-.-:::.:.------:::::-=====+=--====+=###*-=++%:::.....:.....-:::::::::.-=-----------------
:::-:::--==:.:::::...==-::.::-.:====++:--=---::#%#**+**+*=............:::::::::::.------------------
::::::----:--=-..=@::.:-.::::..:==+==+*:......:-=*++=+++++=..........-::::::::::-:------------------
:::::::-:-::--=-:..:.*.::.:::-:-==++==-.##+-=--=+++=*===++==.%.......::-:::::::::::-----------------
-:::::.:-:::--=---::......:.+==-+=+++.::++=--:-+**+==++*=====.++*=..::::-:-:::-::::.:..=.--.:-------
:::::::.::---==+++==-...=.....==+++==:::-----==*++++++++=-*+-..-+**+...----::::::::.--------.....---
```

[![CI](https://github.com/AdriRRP/demonic-tutor/actions/workflows/ci.yml/badge.svg)](https://github.com/AdriRRP/demonic-tutor/actions/workflows/ci.yml)
[![Coverage](https://github.com/AdriRRP/demonic-tutor/actions/workflows/coverage.yml/badge.svg)](https://github.com/AdriRRP/demonic-tutor/actions/workflows/coverage.yml)
[![Security](https://github.com/AdriRRP/demonic-tutor/actions/workflows/security.yml/badge.svg)](https://github.com/AdriRRP/demonic-tutor/actions/workflows/security.yml)
[![codecov](https://codecov.io/gh/AdriRRP/demonic-tutor/branch/main/graph/badge.svg)](https://codecov.io/gh/AdriRRP/demonic-tutor)

DemonicTutor is a Rust-first, client-oriented laboratory for modeling, playing, observing, and analyzing **a deliberately small, explicit subset of Magic: The Gathering**. It is not trying to implement all of Magic at once; it is trying to build a coherent, replayable, rules-aware gameplay core that can grow safely through vertical slices.

## What this repository is for

- model gameplay with strong Domain-Driven Design boundaries
- exercise real play flows instead of only card-level simulation
- make gameplay observable through domain events, projections, and executable features
- evolve rules support incrementally without overstating what is implemented
- keep the core compatible with browser-oriented constraints and eventual WebAssembly use

## Current state

The current runtime supports a deliberately constrained but honestly playable limited shell, including:

- two-player setup with opening hands, simplified London mulligan, deterministic seeded bootstrapping, and rematch support
- a stable public gameplay contract for clients:
  snapshot, legal actions, choice requests, deterministic command envelopes, and persisted replay log
- full explicit phase progression:
  `Setup -> Untap -> Upkeep -> Draw -> FirstMain -> BeginningOfCombat -> DeclareAttackers -> DeclareBlockers -> CombatDamage -> EndOfCombat -> SecondMain -> EndStep`
- stack and priority support for the current bounded subset:
  spells, public `PassPriority`, non-mana activated abilities, triggered abilities, and supported loyalty abilities
- combat with explicit subphases, multiple blockers in declared order, ordered damage assignment, and the current supported combat-keyword subset
- bounded targeted effects, Auras, anthem-style static bonuses, tokens, `+1/+1` counters, graveyard recursion, loot/rummage, scry, and surveil
- curated-set authoring validation plus executable golden matchups for the first playable archetypes
- replay-friendly events, in-memory infrastructure, and executable BDD coverage

For the authoritative snapshot, read [`docs/domain/current-state.md`](docs/domain/current-state.md).

## What this repository is not

- not a full Magic rules engine
- not a card database or comprehensive oracle implementation
- not a generic board-game framework
- not a “simulate everything first, design later” prototype

Unsupported behavior must remain explicit. If code and docs disagree, the code wins and the docs must be reconciled.

## Architecture in one minute

The project is organized around a single implemented bounded context, `play`.

- `src/domain/play/`
  the gameplay core, with the `Game` aggregate as the central consistency boundary
- `src/application/`
  orchestration between commands, aggregate calls, event persistence, and event publication
- `src/interfaces/`
  thin external adapters, currently the browser-facing wasm bridge
- `src/infrastructure/`
  in-memory event store, event bus, and projections
- `apps/web/`
  a Solid + Vite web shell that embeds the real Rust engine through WebAssembly
- `docs/domain/`
  canonical domain truth
- `docs/architecture/`
  structural and evolutionary guidance
- `docs/slices/`
  implemented and proposed vertical slices
- `features/`
  repository-owned gameplay specifications, many of them executable with Cucumber
- `.agents/`
  operational context and reusable skills for agent-assisted work

For the architectural picture, start with:

- [`docs/domain/aggregate-game.md`](docs/domain/aggregate-game.md)
- [`docs/architecture/system-overview.md`](docs/architecture/system-overview.md)
- [`docs/architecture/web-client.md`](docs/architecture/web-client.md)
- [`docs/architecture/vertical-slices.md`](docs/architecture/vertical-slices.md)

## Source of truth

When repository sources disagree, precedence is:

1. code in `src/`
2. accepted ADRs
3. canonical documentation
4. operational agent context
5. skills

That rule matters because this repository deliberately keeps documentation honest and incremental.

## How to navigate the repository

### If you are new to the project

Read, in order:

1. [`PROJECT.md`](PROJECT.md)
2. [`CONSTRAINTS.md`](CONSTRAINTS.md)
3. [`docs/README.md`](docs/README.md)
4. [`docs/domain/current-state.md`](docs/domain/current-state.md)

### If you want the domain model

- [`docs/domain/DOMAIN_GLOSSARY.md`](docs/domain/DOMAIN_GLOSSARY.md)
- [`docs/domain/context-map.md`](docs/domain/context-map.md)
- [`docs/domain/aggregate-game.md`](docs/domain/aggregate-game.md)
- [`docs/domain/current-state.md`](docs/domain/current-state.md)

### If you want rules support and gameplay specs

- [`docs/rules/README.md`](docs/rules/README.md)
- [`docs/rules/rules-map.md`](docs/rules/rules-map.md)
- [`features/README.md`](features/README.md)

### If you are working with agents

Start with:

1. [`AGENTS.md`](AGENTS.md)
2. [`docs/architecture/agent-architecture.md`](docs/architecture/agent-architecture.md)
3. [`.agents/context/core-agent.md`](.agents/context/core-agent.md)

## Development workflow

The repository grows through **small, coherent vertical slices**. Broad refactors are allowed when they clearly improve semantic clarity, cognitive load, or architectural honesty, but they still close like slices: code, tests, docs, and agent context must end aligned.

Useful commands:

```bash
./scripts/check-all.sh
cargo check --target wasm32-unknown-unknown
cargo test --test unit
cargo test --test bdd
cd apps/web && npm install
cd apps/web && npm run dev
cd apps/web && npm run build
```

The authoritative development guidance lives in [`docs/development/development.md`](docs/development/development.md).

For the browser client specifically, also read [`apps/web/README.md`](apps/web/README.md).

## Documentation map

- [`docs/README.md`](docs/README.md): full documentation map
- [`docs/rules/README.md`](docs/rules/README.md): how rules notes and rules mapping are used
- [`features/README.md`](features/README.md): how feature files are organized and executed

## Releases

- release history: [`CHANGELOG.md`](CHANGELOG.md)
- current crate version: [`Cargo.toml`](Cargo.toml)

## Guiding idea

The long-term value of DemonicTutor is not “how many rules it already supports”, but **how honestly and cleanly it supports each rule it does model**. The repository optimizes for explicit semantics, replayable behavior, and maintainable growth.
