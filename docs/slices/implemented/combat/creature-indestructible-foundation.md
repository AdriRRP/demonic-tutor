# Creature Indestructible Foundation

## Summary

Supported creatures with `Indestructible` now survive lethal damage and destroy effects while still dying to zero toughness.

## Scope

- lethal combat or spell damage no longer moves an indestructible creature to the graveyard
- `destroy target creature` does not destroy an indestructible creature
- zero toughness remains a supported way for a creature to die

## Notes

- this slice implements the subset needed by the current engine
- it does not yet claim full support for every rules interaction around replacement or prevention effects
