Feature: Cast a creature during the second main priority window
  Scenario: Alice casts and resolves a creature in SecondMain
    Given Alice is the active player in SecondMain with a creature card in hand and priority
    When Alice casts the creature spell
    Then the spell is on the stack under Alice's control
    And Alice has priority
    When Alice passes priority
    And Bob passes priority
    Then the game emits StackTopResolved
    And the game emits SpellCast with outcome EnteredBattlefield
