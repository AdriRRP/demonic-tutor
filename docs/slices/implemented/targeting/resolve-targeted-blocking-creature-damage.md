# Resolve Targeted Blocking Creature Damage

## Summary

Allow a supported `BlockingCreature` damage spell to kill its blocking-creature target through the shared damage-and-SBA corridor.

## Scope

- resolve damage onto the blocking creature target
- rely on shared SBA review to destroy that creature when the damage is lethal
- keep the behavior inside the current post-blockers targeted-spell subset

## Out Of Scope

- nonlethal blocking-target damage
- richer combat-relative target rules beyond the supported single blocking creature
