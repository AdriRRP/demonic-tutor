# status: implemented
# rules: 506, 508, 509, 510
# slices: combat-subphases-foundation.md

Feature: Combat uses explicit subphases

  Scenario: Advancing from FirstMain enters BeginningOfCombat
    Given Alice is the active player in FirstMain
    And Alice has priority
    And the stack is empty
    When Alice passes priority
    And Bob passes priority
    And the game advances the turn
    Then the phase becomes BeginningOfCombat
    And Alice has priority

  Scenario: Declaring attackers moves combat into DeclareBlockers
    Given Alice has declared attackers in Combat
    Then the phase becomes DeclareBlockers
    And Alice has priority

  Scenario: Declaring blockers moves combat into CombatDamage
    Given Bob has declared blockers in Combat
    Then the phase becomes CombatDamage
    And Alice has priority

  Scenario: Resolving combat damage moves combat into EndOfCombat
    Given combat damage has resolved and Alice still has an instant card in hand with priority
    Then the phase becomes EndOfCombat
    And Alice has priority
