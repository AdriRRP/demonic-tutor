# status: implemented
# rules: 601.2, 117.3b
# slices: reject-own-turn-priority-enchantment-response.md

Feature: Reject an own-turn-priority enchantment as a response
  Scenario: Bob cannot cast an own-turn-priority enchantment during Alice's turn
    Given Alice has cast an instant spell and still holds priority with Bob's own-turn enchantment card in hand
    When Alice passes priority
    And Bob tries to cast the enchantment spell
    Then the action is rejected because the spell only supports open-priority casting during its controller's turn
