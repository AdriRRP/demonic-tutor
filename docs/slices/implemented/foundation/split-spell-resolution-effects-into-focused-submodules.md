# Slice: Split Spell Resolution Effects Into Focused Submodules

Status: implemented

## Summary

The spell-resolution corridor now uses focused internal submodules for shared helpers, battlefield effects, zone and stack effects, and compact misc effects, leaving a thinner dispatcher at the boundary.

## What changed

- `resolution/effects` now splits shared legality and SBA helpers from effect families
- battlefield and life effects, zone and stack effects, and token or mill effects now live in separate focused modules
- the dispatcher keeps the same supported spell semantics while no longer carrying every effect family inline

## Why it matters

- lowers the cognitive load of the current spell-resolution hotspot
- makes the next gameplay waves cheaper to review and safer to extend
- preserves explicit enums and deterministic routing without introducing a generic effect engine
