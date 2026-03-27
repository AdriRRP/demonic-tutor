# Release 0.7.0 - Wave 4 - Runtime Hardening

Status:

- historical
- fully implemented

Goal:

- reduce structural drag in the current engine without widening Magic rules support

Slice count:

- `9`

Slices:

- [expose-readonly-legal-action-queries-for-public-contract.md](/Users/adrianramos/Repos/demonictutor/docs/slices/proposals/release-0.7.0-wave-4-runtime-hardening/expose-readonly-legal-action-queries-for-public-contract.md)
- [unify-pending-stack-decisions.md](/Users/adrianramos/Repos/demonictutor/docs/slices/proposals/release-0.7.0-wave-4-runtime-hardening/unify-pending-stack-decisions.md)
- [use-explicit-location-lookup-for-aura-detach.md](/Users/adrianramos/Repos/demonictutor/docs/slices/proposals/release-0.7.0-wave-4-runtime-hardening/use-explicit-location-lookup-for-aura-detach.md)
- [split-spell-rule-profiles-into-focused-submodules.md](/Users/adrianramos/Repos/demonictutor/docs/slices/proposals/release-0.7.0-wave-4-runtime-hardening/split-spell-rule-profiles-into-focused-submodules.md)
- [remove-repeated-linear-lookups-from-combat-damage-step.md](/Users/adrianramos/Repos/demonictutor/docs/slices/proposals/release-0.7.0-wave-4-runtime-hardening/remove-repeated-linear-lookups-from-combat-damage-step.md)
- [derive-public-target-candidates-from-canonical-legality-queries.md](/Users/adrianramos/Repos/demonictutor/docs/slices/proposals/release-0.7.0-wave-4-runtime-hardening/derive-public-target-candidates-from-canonical-legality-queries.md)
- [derive-public-blocker-options-from-canonical-combat-queries.md](/Users/adrianramos/Repos/demonictutor/docs/slices/proposals/release-0.7.0-wave-4-runtime-hardening/derive-public-blocker-options-from-canonical-combat-queries.md)
- [rename-aggregate-location-owner-index-to-player-index.md](/Users/adrianramos/Repos/demonictutor/docs/slices/proposals/release-0.7.0-wave-4-runtime-hardening/rename-aggregate-location-owner-index-to-player-index.md)
- [split-spell-resolution-effects-into-focused-submodules.md](/Users/adrianramos/Repos/demonictutor/docs/slices/proposals/release-0.7.0-wave-4-runtime-hardening/split-spell-resolution-effects-into-focused-submodules.md)

Why this wave has high return:

- it keeps `0.7.0` honest as the point where UI work can begin
- it removes avoidable cost from the public read path and from combat
- it keeps public choices and combat prompts aligned with aggregate legality instead of UI-local approximations
- it sharpens owner-versus-controller semantics before broader control-changing support arrives
- it lowers the cognitive and change cost of the next gameplay waves without inventing speculative engines
