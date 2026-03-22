Feature: Pump target creature until end of turn

  Scenario: A spell gives a creature +2/+2 until end of turn
    Given Alice is the active player in FirstMain with a pump-creature instant spell and her creature on the battlefield
    When Alice casts the pump-creature instant spell targeting her creature
    Then the spell is on the stack targeting Alice's creature
    When both players pass priority in succession
    Then the stack resolves with the top spell
    And Alice's creature gets +2/+2 until end of turn
