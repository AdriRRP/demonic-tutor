# status: implemented
# rules: 117, 301.1, 301.2, 601, 608
# slices: cast-artifact-in-main-window.md

Feature: Cast an artifact during an empty main-phase priority window
  Scenario: Alice casts and resolves an artifact in FirstMain
    Given Alice is the active player in FirstMain with an artifact card in hand and priority
    And the stack is empty
    When Alice casts the artifact spell
    Then the spell is on the stack under Alice's control
    And Alice has priority
    When Alice passes priority
    And Bob passes priority
    Then the game emits StackTopResolved
    And the game emits SpellCast with outcome EnteredBattlefield
    And Alice has priority again

  Scenario: Alice casts and resolves an artifact in SecondMain
    Given Alice is the active player in SecondMain with an artifact card in hand and priority
    And the stack is empty
    When Alice casts the artifact spell
    Then the spell is on the stack under Alice's control
    And Alice has priority
    When Alice passes priority
    And Bob passes priority
    Then the game emits StackTopResolved
    And the game emits SpellCast with outcome EnteredBattlefield
    And Alice has priority again
