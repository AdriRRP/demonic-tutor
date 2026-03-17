# status: implemented
# rules: 117, 509, 601, 608
# slices: respond-after-blockers.md

Feature: Respond with an instant after blockers are declared
  Scenario: Bob casts and resolves an instant after Alice passes priority after blockers
    Given Bob has priority after blockers are declared with an instant card in hand
    And the stack is empty
    When Bob casts the instant spell
    Then the spell is on the stack under Bob's control
    And Bob has priority
    When Bob passes priority
    And Alice passes priority
    Then the game emits StackTopResolved
    And the game emits SpellCast with outcome ResolvedToGraveyard
    And Alice has priority again
