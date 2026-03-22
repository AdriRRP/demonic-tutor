# Cast Enchantment In Main Window

## Goal

Allow the active player to cast an enchantment spell during an empty `FirstMain` or `SecondMain`
priority window and resolve it onto the battlefield.

## Scope

In scope:

- casting an enchantment spell from hand while the active player holds priority in `FirstMain`
- casting an enchantment spell from hand while the active player holds priority in `SecondMain`
- resolving the enchantment spell from the stack to the battlefield

Out of scope:

- enchantment abilities
- targeting
- aura attachment rules

## Notes

- This slice follows the same sorcery-speed timing already supported for sorceries and artifacts.
- The enchantment is put onto the stack first and only enters the battlefield after stack
  resolution.
