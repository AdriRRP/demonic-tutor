# status: implemented
# rules: 117, 508, 509, 510, 511
# slices: combat-step-priority-progression.md

Feature: Combat step priority progression

  Scenario: Declaring attackers advances combat into DeclareBlockers with priority
    Given Alice has declared attackers in Combat
    Then the phase becomes DeclareBlockers
    And Alice has priority
    And the stack is empty

  Scenario: Declaring blockers advances combat into CombatDamage with priority
    Given Bob has declared blockers in Combat
    Then the phase becomes CombatDamage
    And Alice has priority
    And the stack is empty

  Scenario: Resolving combat damage advances combat into EndOfCombat with priority
    Given Alice attacks with an unblocked creature
    When combat damage resolves
    Then the phase becomes EndOfCombat
    And Alice has priority
    And the stack is empty
