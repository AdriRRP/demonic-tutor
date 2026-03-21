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

The current runtime supports a meaningful minimal playtest loop, including:

- two-player game setup with opening hands and simplified London mulligan
- full phase progression:
  `Setup -> Untap -> Upkeep -> Draw -> FirstMain -> BeginningOfCombat -> DeclareAttackers -> DeclareBlockers -> CombatDamage -> EndOfCombat -> SecondMain -> EndStep`
- land play, land tapping, mana payment, and cleanup discard
- spell casting through a canonical `CastSpell` action
- minimal stack and priority support with explicit stack objects and public `PassPriority`
- empty priority windows in the currently supported turn and combat moments
- active-player instant casting, non-active instant responses, and self-stacking in the supported windows
- sorcery-speed spells for the active player in empty main-phase windows
- minimal targeted instant support against players and creatures
- combat with explicit subphases, attacker/blocker declaration, damage resolution, and single-blocker-per-attacker simplification
- keyword abilities `Flying` and `Reach` for blocking legality
- player-owned `Exile` zone
- shared automatic consequences for zero life, empty-library draw, lethal damage, zero toughness, and cleanup damage removal
- in-memory event store, event bus, gameplay log projection, and executable BDD coverage

For the authoritative snapshot, read [`docs/domain/current-state.md`](/Users/adrianramos/Repos/demonictutor/docs/domain/current-state.md).

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
- `src/infrastructure/`
  in-memory event store, event bus, and projections
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

- [`docs/domain/aggregate-game.md`](/Users/adrianramos/Repos/demonictutor/docs/domain/aggregate-game.md)
- [`docs/architecture/system-overview.md`](/Users/adrianramos/Repos/demonictutor/docs/architecture/system-overview.md)
- [`docs/architecture/vertical-slices.md`](/Users/adrianramos/Repos/demonictutor/docs/architecture/vertical-slices.md)

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

1. [`PROJECT.md`](/Users/adrianramos/Repos/demonictutor/PROJECT.md)
2. [`CONSTRAINTS.md`](/Users/adrianramos/Repos/demonictutor/CONSTRAINTS.md)
3. [`docs/README.md`](/Users/adrianramos/Repos/demonictutor/docs/README.md)
4. [`docs/domain/current-state.md`](/Users/adrianramos/Repos/demonictutor/docs/domain/current-state.md)

### If you want the domain model

- [`docs/domain/DOMAIN_GLOSSARY.md`](/Users/adrianramos/Repos/demonictutor/docs/domain/DOMAIN_GLOSSARY.md)
- [`docs/domain/context-map.md`](/Users/adrianramos/Repos/demonictutor/docs/domain/context-map.md)
- [`docs/domain/aggregate-game.md`](/Users/adrianramos/Repos/demonictutor/docs/domain/aggregate-game.md)
- [`docs/domain/current-state.md`](/Users/adrianramos/Repos/demonictutor/docs/domain/current-state.md)

### If you want rules support and gameplay specs

- [`docs/rules/README.md`](/Users/adrianramos/Repos/demonictutor/docs/rules/README.md)
- [`docs/rules/rules-map.md`](/Users/adrianramos/Repos/demonictutor/docs/rules/rules-map.md)
- [`features/README.md`](/Users/adrianramos/Repos/demonictutor/features/README.md)

### If you are working with agents

Start with:

1. [`AGENTS.md`](/Users/adrianramos/Repos/demonictutor/AGENTS.md)
2. [`docs/architecture/agent-architecture.md`](/Users/adrianramos/Repos/demonictutor/docs/architecture/agent-architecture.md)
3. [`.agents/context/core-agent.md`](/Users/adrianramos/Repos/demonictutor/.agents/context/core-agent.md)

## Development workflow

The repository grows through **small, coherent vertical slices**. Broad refactors are allowed when they clearly improve semantic clarity, cognitive load, or architectural honesty, but they still close like slices: code, tests, docs, and agent context must end aligned.

Useful commands:

```bash
./scripts/check-all.sh
cargo test --test unit
cargo test --test bdd
```

The authoritative development guidance lives in [`docs/development/development.md`](/Users/adrianramos/Repos/demonictutor/docs/development/development.md).

## Documentation map

- [`docs/README.md`](/Users/adrianramos/Repos/demonictutor/docs/README.md): full documentation map
- [`docs/rules/README.md`](/Users/adrianramos/Repos/demonictutor/docs/rules/README.md): how rules notes and rules mapping are used
- [`features/README.md`](/Users/adrianramos/Repos/demonictutor/features/README.md): how feature files are organized and executed

## Releases

- release history: [`CHANGELOG.md`](/Users/adrianramos/Repos/demonictutor/CHANGELOG.md)
- current crate version: [`Cargo.toml`](/Users/adrianramos/Repos/demonictutor/Cargo.toml)

## Guiding idea

The long-term value of DemonicTutor is not “how many rules it already supports”, but **how honestly and cleanly it supports each rule it does model**. The repository optimizes for explicit semantics, replayable behavior, and maintainable growth.
