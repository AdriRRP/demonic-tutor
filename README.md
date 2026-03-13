# DemonicTutor

[![CI](https://github.com/AdriRRP/demonic-tutor/actions/workflows/ci.yml/badge.svg)](https://github.com/AdriRRP/demonic-tutor/actions/workflows/ci.yml)
[![Coverage](https://github.com/AdriRRP/demonic-tutor/actions/workflows/coverage.yml/badge.svg)](https://github.com/AdriRRP/demonic-tutor/actions/workflows/coverage.yml)
[![Security](https://github.com/AdriRRP/demonic-tutor/actions/workflows/security.yml/badge.svg)](https://github.com/AdriRRP/demonic-tutor/actions/workflows/security.yml)
[![codecov](https://codecov.io/gh/AdriRRP/demonic-tutor/branch/main/graph/badge.svg)](https://codecov.io/gh/AdriRRP/demonic-tutor)

DemonicTutor is a client-side application for playing, observing and analyzing Magic: The Gathering deck behavior through real game sessions, event logging and live statistics.

The project is designed as a practical laboratory for:
- deck testing
- game observability
- replayability
- rules-aware domain modeling
- analytics derived from actual play

## Current status

This repository is in active development.

Five vertical slices have been implemented:
1. **StartGame** — Initialize a game with two players
2. **DrawOpeningHands** — Deal opening hands to players
3. **PlayLand** — Play a land from hand to battlefield
4. **AdvanceTurn** — Advance to the next player's turn
5. **DrawCard** — Draw a card from library to hand

## Guiding idea

DemonicTutor is not intended to be a full implementation of all Magic rules from the beginning.

It is intended to become:
- a precise and fast gameplay core
- a replayable event-driven system
- a deck analysis tool based on real sessions
- a solid Rust + WebAssembly learning project

## Initial documentation

- `PROJECT.md` defines the product vision and scope.
- `CONSTRAINTS.md` defines technical and modeling restrictions.
- `DOMAIN_GLOSSARY.md` defines the initial ubiquitous language.

## Development philosophy

The system will be developed incrementally, with narrow vertical slices and explicit decisions.

The first priority is correctness and clarity of the domain model.
Breadth, advanced UX and richer rules support come later.

## Development

### Local checks

Run the complete quality check suite:

```bash
./scripts/check-all.sh
```

Individual checks:

```bash
./scripts/fmt.sh       # Format check
./scripts/test.sh      # Run tests
./scripts/clippy.sh    # Strict clippy
./scripts/security.sh  # Dependency security audit
./scripts/coverage.sh  # Generate coverage report
```

### CI

GitHub Actions runs:
- Format check
- Strict clippy
- Tests
- Security audit (cargo-audit)
- Coverage (cargo-llvm-cov)

### Dependencies

- Rust stable channel (pinned via `rust-toolchain.toml`)
- `cargo-audit` for security vulnerability scanning
- `cargo-llvm-cov` for coverage reporting
