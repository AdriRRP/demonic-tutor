# status: implemented
# rules: 117, 405, 601, 608
# slices: respond-with-instant-spell.md

  Feature: Responding with an instant spell while holding priority

  Scenario: Bob responds to Alice's spell with an instant
    Given Alice has cast a creature spell and still holds priority with Bob's instant in hand
    When Alice passes priority
    And Bob casts the instant response spell
    Then Bob's instant is on top of the stack under Bob's control
    And Bob has priority
    When Bob passes priority
    And Alice passes priority
    Then the game emits StackTopResolved
    And the game emits SpellCast with outcome ResolvedToGraveyard
    And Alice's original spell remains on the stack
    And Alice has priority again

  Scenario: Bob cannot respond with a creature spell yet
    Given Alice has cast an instant spell and still holds priority with Bob's creature card in hand
    When Alice passes priority
    And Bob tries to cast the creature response spell
    Then the action is rejected because the spell timing is not legal in the current window
