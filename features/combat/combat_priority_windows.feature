# status: implemented
# rules: 117, 508, 509
# slices: combat-priority-windows.md

Feature: Combat actions open priority windows

  Scenario: Declaring attackers opens priority for the active player
    Given Alice has declared attackers in Combat
    Then the phase becomes DeclareBlockers
    And Alice has priority
    And the stack is empty

  Scenario: Declaring blockers opens priority for the active player
    Given Bob has declared blockers in Combat
    Then the phase becomes CombatDamage
    And Alice has priority
    And the stack is empty
