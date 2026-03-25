# Slice Proposals

This directory contains the live proposal backlog after the `v0.5.0` runtime-closure release.

The next release plan is organized as **functional waves** aimed at making the engine much more usable for real deck playtesting while staying truthful about the supported subset.

## Active Proposal Waves

### `release-0.6.0-wave-1-stack-interaction-baseline`

Highest-return interactive spell families that unlock much more realistic gameplay:

- `return-target-permanent-to-hand.md`
- `destroy-target-artifact-or-enchantment.md`
- `discard-target-player-card.md`

### `release-0.6.0-wave-2-triggered-abilities-baseline`

Minimal triggered-ability engine for common and high-value card behavior:

- `enter-the-battlefield-trigger-foundation.md`
- `dies-trigger-foundation.md`
- `upkeep-trigger-foundation.md`
- `end-step-trigger-foundation.md`

### `release-0.6.0-wave-3-activated-ability-usability`

Broader activated-ability support so more permanents become meaningfully playable:

- `generalize-tap-activated-abilities.md`
- `support-mana-costed-activated-abilities.md`
- `support-sacrifice-as-activation-cost.md`
- `planeswalker-loyalty-ability-foundation.md`

### `release-0.6.0-wave-4-combat-rules-usability`

Combat upgrades with large gameplay payoff and good rules coverage:

- `multiple-blockers-per-attacker.md`
- `combat-damage-assignment-order.md`
- `deathtouch-foundation.md`
- `double-strike-foundation.md`

## Recommended Release Shape

Strong `0.6.0` target:

1. complete wave 1
2. complete wave 2
3. complete at least the first two slices of wave 3

Stretch target:

- wave 4 after the interaction and trigger baseline is stable

This ordering aims for:

- more real deck interaction
- more cards behaving like cards instead of vanilla runtime carriers
- better approximation of the rules players expect first
