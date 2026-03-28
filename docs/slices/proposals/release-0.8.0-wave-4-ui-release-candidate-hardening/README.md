# Release 0.8.0 - Wave 4 - UI Release Candidate Hardening

Goal:

- make the public runtime contract stable enough for a first real playable client

Slice count:

- `4`

Slices:

- `expose-public-event-log-for-replay-and-animation`
  - provide a deterministic event stream suitable for UI playback and debugging
- `expose-stable-prompt-ordering-for-simultaneous-triggers-and-choices`
  - fix one deterministic ordering contract for surfaced prompts in the supported subset
- `support-concede-rematch-and-seeded-game-setup-commands`
  - support the minimum session loop needed for repeated real playtests
- `freeze-v1-playable-subset-and-reject-unsupported-card-loads`
  - lock the first playable scope so UI and content can rely on it

Why this wave has high return:

- it is the final bridge from "engine with slices" to "usable playable product shell"
