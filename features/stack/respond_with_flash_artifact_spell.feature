# status: implemented
# rules: 117.1a, 117.3b, 601.2
# slices: respond-with-flash-artifact-spell.md

Feature: Respond with a flash artifact spell
  Scenario: Bob responds to Alice's spell with a flash artifact
    Given Alice has cast a creature spell and still holds priority with Bob's flash artifact in hand
    When Alice passes priority
    And Bob casts the flash artifact response spell
    Then the spell is on top of the stack under Bob's control
    And Bob has priority
    When Bob passes priority
    And Alice passes priority
    Then the game emits StackTopResolved
    And the game emits SpellCast with outcome EnteredBattlefield
    And Alice's original spell remains on the stack
    And Alice has priority again
