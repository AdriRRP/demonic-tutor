# status: implemented
# rules: 117, 405, 601
# slices: cast-spell.md

Feature: Casting a spell puts it onto the stack before it resolves

  Scenario: Casting an instant in FirstMain creates a stack object instead of resolving immediately
    Given Alice is the active player in FirstMain
    And Alice has an instant card in hand with enough mana
    When Alice casts the instant spell
    Then the card leaves Alice's hand
    And the spell is on the stack under Alice's control
    And the spell has not resolved yet
    And the game emits SpellPutOnStack
