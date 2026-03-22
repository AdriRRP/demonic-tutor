# status: implemented
# rules: 106.1, 202, 601.2f
# slices: reject-mixed-cost-without-required-color.md

Feature: Reject a mixed green instant when the required green symbol is missing

  Scenario: Alice cannot cast a 1G instant using only red and white mana
    Given Alice is the active player in FirstMain with a mixed green instant card in hand and only red and white mana available
    And Alice has enough mana to pay its cost
    When Alice tries to cast the green instant spell
    Then the action is rejected because the available mana does not satisfy the colored cost
