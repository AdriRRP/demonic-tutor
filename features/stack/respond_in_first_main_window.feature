# status: implemented
# rules: 117, 505, 601, 608
# slices: respond-in-first-main-window.md

Feature: Respond with an instant during the first main priority window
  Scenario: Bob casts and resolves an instant after Alice passes priority in FirstMain
    Given Bob has priority in FirstMain with an instant card in hand
    And the stack is empty
    When Bob casts the instant spell
    Then the spell is on the stack under Bob's control
    And Bob has priority
    When Bob passes priority
    And Alice passes priority
    Then the game emits StackTopResolved
    And the game emits SpellCast with outcome ResolvedToGraveyard
    And Alice has priority again
