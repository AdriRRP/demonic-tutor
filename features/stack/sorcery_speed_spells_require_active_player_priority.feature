Feature: Sorcery-speed spells still require active-player priority
  Scenario: Bob cannot cast an artifact spell in Alice's empty FirstMain window
    Given Bob has priority in FirstMain with an artifact card in hand
    And the stack is empty
    When Bob tries to cast the artifact spell
    Then the action is rejected because the spell timing is not legal in the current window
