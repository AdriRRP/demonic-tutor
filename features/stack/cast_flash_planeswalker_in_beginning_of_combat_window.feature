# slices: generalize-flash-support-for-noncreature-spells.md

Feature: Cast a flash planeswalker at beginning of combat
  Scenario: Alice casts and resolves a flash planeswalker at BeginningOfCombat
    Given Alice is the active player in BeginningOfCombat with a flash planeswalker card in hand and priority
    When Alice casts the planeswalker spell
    Then the spell is on the stack under Alice's control
    And Alice has priority
    When Alice passes priority
    And Bob passes priority
    Then the game emits StackTopResolved
    And the game emits SpellCast with outcome EnteredBattlefield
    And Alice has priority again
