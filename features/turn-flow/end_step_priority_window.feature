# status: implemented
# rules: 117, 500, 507
# slices: end-step-priority-window.md

Feature: EndStep opens a priority window before cleanup can finish the turn

  Scenario: Advancing from SecondMain to EndStep opens priority for the active player
    Given Alice is the active player in SecondMain
    And Alice has priority
    And the stack is empty
    When Alice passes priority
    And Bob passes priority
    And the game advances the turn
    Then the phase becomes EndStep
    And Alice has priority
    And the stack is empty

  Scenario: Two consecutive passes close an empty EndStep priority window
    Given Alice is the active player in EndStep
    And Alice has priority
    And the stack is empty
    When Alice passes priority
    And Bob passes priority
    Then no priority window is open
