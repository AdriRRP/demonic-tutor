# Slice: Rename Aggregate Location Owner Index To Player Index

Status: implemented

## Summary

Aggregate location lookups and compact battlefield references now use `player_index` wording where they refer to the current runtime carrier arena, while persistent card ownership remains modeled through `owner_id`.

## What changed

- aggregate card locations now expose `player_index` instead of `owner_index`
- battlefield and stack-time compact references now use the same player-arena wording
- persistent ownership stays modeled on the card instance itself rather than in carrier indices

## Why it matters

- removes an owner/controller ambiguity from hot runtime corridors
- makes future control-changing work less error-prone
- keeps the ubiquitous language sharper around ownership versus current carrier state
