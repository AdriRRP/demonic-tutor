# status: proposed
# rules: 117, 405, 608
# slices: stack-priority-minimal.md

Feature: Passing priority resolves the top object when both players pass

  Scenario: Two consecutive passes resolve the top spell on the stack
    Given Alice has cast a spell and it is on the stack
    And Alice has priority
    When Alice passes priority
    And Bob passes priority
    Then the top object on the stack resolves
    And the stack becomes empty
    And the game emits StackTopResolved

  Scenario: Two consecutive passes with an empty stack close the current priority window
    Given the game is in a priority window with an empty stack
    And Bob has priority
    When Bob passes priority
    And Alice passes priority
    Then the priority window closes
    And the current step or phase may continue
