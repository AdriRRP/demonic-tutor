# status: implemented
# rules: 117.1a, 117.3b, 511.3, 511.4
# slices: cast-flash-creature-after-combat-damage.md

Feature: Cast a flash creature after combat damage resolves
  Scenario: Alice casts and resolves a flash creature in EndOfCombat
    Given combat damage has resolved and Alice still has a flash creature card in hand with priority
    When Alice casts the flash creature spell
    Then the spell is on the stack under Alice's control
    And Alice has priority
    When Alice passes priority
    And Bob passes priority
    Then the game emits StackTopResolved
    And the game emits SpellCast with outcome EnteredBattlefield
    And the card enters Alice's battlefield
    And the card has summoning sickness
