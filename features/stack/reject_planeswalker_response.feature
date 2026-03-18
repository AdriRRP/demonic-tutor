Feature: Reject a planeswalker as a response on the stack
  Scenario: Bob cannot respond with a planeswalker spell yet
    Given Alice has cast an instant spell and still holds priority with Bob's planeswalker card in hand
    When Alice passes priority
    And Bob tries to cast the planeswalker response spell
    Then the action is rejected because only instant responses are currently supported
