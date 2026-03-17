# status: implemented
# rules: 117, 510, 511
# slices: post-combat-damage-priority-window.md

Feature: Resolving combat damage opens a priority window

  Scenario: Combat damage reopens priority for the active player
    Given Alice attacks with an unblocked creature
    When combat damage resolves
    Then the phase becomes EndOfCombat
    And Alice has priority
    And the stack is empty
