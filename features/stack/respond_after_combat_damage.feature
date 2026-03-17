# status: implemented
# rules: 117, 510, 511, 601, 608
# slices: respond-after-combat-damage.md

Feature: Respond with an instant after combat damage resolves
  Scenario: Bob casts and resolves an instant after Alice passes priority after combat damage
    Given Bob has priority after combat damage with an instant card in hand
    And the stack is empty
    When Bob casts the instant spell
    Then the spell is on the stack under Bob's control
    And Bob has priority
    When Bob passes priority
    And Alice passes priority
    Then the game emits StackTopResolved
    And the game emits SpellCast with outcome ResolvedToGraveyard
    And Alice has priority again
