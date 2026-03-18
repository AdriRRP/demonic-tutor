# status: implemented
# rules: 117, 307.1, 307.5, 601, 608
# slices: cast-sorcery-in-main-window.md

Feature: Cast a sorcery during an empty main-phase priority window
  Scenario: Alice casts and resolves a sorcery in FirstMain
    Given Alice is the active player in FirstMain with a sorcery card in hand and priority
    And the stack is empty
    When Alice casts the sorcery spell
    Then the spell is on the stack under Alice's control
    And Alice has priority
    When Alice passes priority
    And Bob passes priority
    Then the game emits StackTopResolved
    And the game emits SpellCast with outcome ResolvedToGraveyard
    And Alice has priority again

  Scenario: Alice casts and resolves a sorcery in SecondMain
    Given Alice is the active player in SecondMain with a sorcery card in hand and priority
    And the stack is empty
    When Alice casts the sorcery spell
    Then the spell is on the stack under Alice's control
    And Alice has priority
    When Alice passes priority
    And Bob passes priority
    Then the game emits StackTopResolved
    And the game emits SpellCast with outcome ResolvedToGraveyard
    And Alice has priority again
