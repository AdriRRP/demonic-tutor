Feature: Cast an enchantment in an empty main-phase priority window
  Scenario: Alice casts and resolves an enchantment in FirstMain
    Given Alice is the active player in FirstMain with an enchantment card in hand and priority
    When Alice casts the enchantment spell
    Then the spell is on the stack under Alice's control
    And Alice has priority
    When Alice passes priority
    And Bob passes priority
    Then the game emits StackTopResolved
    And the game emits SpellCast with outcome EnteredBattlefield

  Scenario: Alice casts and resolves an enchantment in SecondMain
    Given Alice is the active player in SecondMain with an enchantment card in hand and priority
    When Alice casts the enchantment spell
    Then the spell is on the stack under Alice's control
    And Alice has priority
    When Alice passes priority
    And Bob passes priority
    Then the game emits StackTopResolved
    And the game emits SpellCast with outcome EnteredBattlefield
