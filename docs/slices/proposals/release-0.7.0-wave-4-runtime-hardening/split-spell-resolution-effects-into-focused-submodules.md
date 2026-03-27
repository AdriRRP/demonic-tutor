# Slice: Split Spell Resolution Effects Into Focused Submodules

Status: proposed

## Summary

Break the spell-resolution corridor into explicit internal modules by effect family, leaving a thinner dispatcher at the boundary.

## Scope

- split `resolution/effects.rs` into focused submodules for shared helpers and effect families
- preserve current resolution behavior and test coverage
- keep the aggregate boundary unchanged while reducing the monolithic hotspot

## Out of scope

- changing supported spell rules
- introducing a generic effect engine
- altering public command or event semantics

## Why now

The spell resolver is becoming the next crowded hotspot in the engine. Splitting it now keeps future waves readable and cheaper to review.
