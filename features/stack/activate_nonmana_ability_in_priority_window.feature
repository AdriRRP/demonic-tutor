Feature: Activate non-mana ability in priority window
  Scenario: Alice activates a supported non-mana ability in first main
    Given Alice is in first main with a life-gain artifact on the battlefield and priority
    When Alice activates the tracked ability
    Then the activated ability is on the stack under Alice's control
    And the tracked permanent is tapped
    And Alice has priority
    When Alice passes priority
    And Bob passes priority
    Then Alice has 21 life
    And the stack is empty
