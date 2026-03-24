# Implemented Slices

Implemented slice docs are grouped by capability so the directory stays navigable.

## Groups

- `foundation/`
  repository and gameplay foundations, historical cleanup, and setup-era slices
- `turn-flow/`
  phases, turn progression, draws, and turn-local priority windows
- `mana/`
  mana production, payment, and pool semantics
- `life/`
  player life and zero-life game loss
- `zones/`
  explicit zone movement capabilities such as lands and exile
- `state/`
  state-based reviews, automatic cleanup, and other automatic gameplay consequences
- `stack/`
  stack, casting, responses, flash-like support, and priority-window spell corridors
- `abilities/`
  explicit activated abilities and stack-free mana-ability boundaries
- `targeting/`
  target legality and the currently supported targeted-spell families
- `combat/`
  combat steps, combat windows, blocking rules, and combat-local constraints

Use canonical docs for live truth first, then the relevant slice group when you need historical implementation context.
