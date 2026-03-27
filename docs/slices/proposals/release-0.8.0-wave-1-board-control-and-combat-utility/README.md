# Release 0.8.0 - Wave 1 - Board Control And Combat Utility

Goal:

- add the most common nonlethal combat-control effects used by limited cards

Slice count:

- `4`

Slices:

- implemented:
  - `support-tap-target-creature-effects`
    - first explicit spell corridor that taps a target creature on the battlefield
- `support-untap-target-creature-effects`
  - support spells and abilities that untap a creature for combat and activation play patterns
- `support-cannot-block-this-turn-effects`
  - support temporary combat restriction effects applied during the turn
- `support-defender-and-static-cannot-attack-restrictions`
  - support creatures and a bounded static subset that cannot attack under the modeled rules

Why this wave has high return:

- these effects make combat-based games dramatically richer without needing full layers or rare rules families
