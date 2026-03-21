# status: implemented
# rules: 117, 307.1, 601
# slices: reject-sorcery-response.md

Feature: Sorceries cannot be cast as responses on an open stack
  Scenario: Bob cannot respond to Alice's instant with a sorcery
    Given Alice has cast an instant spell and still holds priority with Bob's sorcery card in hand
    When Alice passes priority
    And Bob tries to cast the sorcery response spell
    Then the action is rejected because the spell timing is not legal in the current window
