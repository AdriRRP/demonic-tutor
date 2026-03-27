# Slice: Split Spell Rule Profiles Into Focused Submodules

Status: implemented

## Summary

The spell targeting and resolution hotspot has been extracted into a focused submodule so future spell growth no longer keeps piling onto one oversized rules file.

## What changed

- `SpellTargetingProfile`, `SpellResolutionProfile`, and `SupportedSpellRules` now live in `rules/spell_profiles.rs`
- the public API remains stable through re-exports from `rules.rs`
- the rest of the card-rules model stays explicit and enum-driven

## Why it matters

- reduces cognitive load in the central card rules module
- gives future spell slices a tighter place to land
- preserves closed enums and explicit semantics without reintroducing monolithic growth
