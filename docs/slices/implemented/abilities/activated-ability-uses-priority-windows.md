# Slice — Activated Ability Uses Priority Windows

## Outcome

The first supported non-mana activated ability now uses the same explicit priority-window and stack corridor as supported spells.

## Supported Behavior

- a supported non-mana activated ability may be activated only by the current priority holder
- the source permanent must be on the battlefield
- the activation uses the stack
- the activating player retains priority after putting the ability on the stack
- the first supported corridor is `Tap: you gain 1 life`

## Notes

- mana abilities remain separate and stack-free
- this slice does not imply broad activated-ability support, targets, modes, or loyalty abilities

## Executable Coverage

- unit coverage for main-window activation, response-window activation, priority rejection, and resolution
- BDD coverage in `features/stack/activate_nonmana_ability_in_priority_window.feature`
