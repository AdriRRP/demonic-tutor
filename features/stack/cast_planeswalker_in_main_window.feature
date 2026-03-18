Feature: Cast a planeswalker during an empty main-phase priority window
  Scenario: Alice casts and resolves a planeswalker in FirstMain
    Given Alice is the active player in FirstMain with a planeswalker card in hand and priority
    When Alice casts the planeswalker spell
    Then the spell is on the stack under Alice's control
    And Alice has priority
    When Alice passes priority
    And Bob passes priority
    Then the game emits StackTopResolved
    And the game emits SpellCast with outcome EnteredBattlefield

  Scenario: Alice casts and resolves a planeswalker in SecondMain
    Given Alice is the active player in SecondMain with a planeswalker card in hand and priority
    When Alice casts the planeswalker spell
    Then the spell is on the stack under Alice's control
    And Alice has priority
    When Alice passes priority
    And Bob passes priority
    Then the game emits StackTopResolved
    And the game emits SpellCast with outcome EnteredBattlefield
