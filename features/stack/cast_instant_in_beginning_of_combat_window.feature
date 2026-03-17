# status: implemented
# rules: 117, 506, 601, 608
# slices: cast-instant-in-beginning-of-combat-window.md

Feature: Cast an instant during the beginning-of-combat priority window

  Scenario: Alice casts and resolves an instant at the beginning of Combat
    Given Alice is at the beginning of Combat with an instant card in hand and priority
    And the stack is empty
    When Alice casts the instant spell
    Then the spell is on the stack under Alice's control
    And Alice has priority
    When Alice passes priority
    And Bob passes priority
    Then the game emits StackTopResolved
    And the game emits SpellCast with outcome ResolvedToGraveyard
    And Alice has priority again
