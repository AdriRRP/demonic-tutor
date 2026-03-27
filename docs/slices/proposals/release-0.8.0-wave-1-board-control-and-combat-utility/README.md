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
    - first explicit spell corridor that untaps a target creature on the battlefield
  - `support-cannot-block-this-turn-effects`
    - first temporary combat restriction effect applied to one target creature until turn end
  - `support-defender-and-static-cannot-attack-restrictions`
    - first bounded static attack restriction subset centered on defender-like creatures
- pending:

Why this wave has high return:

- these effects make combat-based games dramatically richer without needing full layers or rare rules families
