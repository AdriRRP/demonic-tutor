# status: implemented
# rules: 117, 405
# slices: stack-foundation.md

Feature: Stack foundation exists in the aggregate before stack behavior is enabled

  Scenario: Starting a game initializes an empty stack and no open priority window
    Given a new two-player game has started
    Then the stack is empty
    And no priority window is open
