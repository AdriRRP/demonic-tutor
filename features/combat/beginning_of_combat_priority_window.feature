# status: implemented
# rules: 117, 506, 507, 508
# slices: beginning-of-combat-priority-window.md

Feature: Entering Combat opens a priority window

  Scenario: Advancing from FirstMain to Combat opens priority for the active player
    Given Alice enters Combat from FirstMain
    Then the phase becomes BeginningOfCombat
    And Alice has priority
    And the stack is empty
