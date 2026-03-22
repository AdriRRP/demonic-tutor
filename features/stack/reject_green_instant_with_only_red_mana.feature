# status: implemented
# rules: 106.1, 601.2f
# slices: colored-mana-foundation.md

Feature: Reject a green instant when only red mana is available

  Scenario: Alice cannot cast a green instant using only red mana
    Given Alice is the active player in FirstMain with a green instant card in hand and only a mountain available
    And Alice has only red mana available to pay its cost
    When Alice tries to cast the green instant spell
    Then the action is rejected because the available mana does not satisfy the colored cost
