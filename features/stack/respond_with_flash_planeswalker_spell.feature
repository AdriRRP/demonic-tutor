# slices: generalize-flash-support-for-noncreature-spells.md

Feature: Respond with a flash planeswalker spell
  Scenario: Bob responds to Alice's spell with a flash planeswalker
    Given Alice has cast a creature spell and still holds priority with Bob's flash planeswalker in hand
    When Alice passes priority
    And Bob casts the planeswalker response spell
    Then the spell is on the stack under Bob's control
    And Bob has priority
    When Bob passes priority
    And Alice passes priority
    Then the game emits StackTopResolved
    And the game emits SpellCast with outcome EnteredBattlefield
    And Alice's original spell remains on the stack
    And Alice has priority again
