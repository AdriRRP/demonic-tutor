# status: implemented
# rules: 704, 704.5a, 704.5f, 704.5g
# slices: state-based-actions-review.md, zero-toughness-creature-dies.md, creature-destruction.md, lose-on-zero-life.md

Feature: Supported state-based actions are reviewed after relevant gameplay actions

  Scenario: A pending zero-toughness creature dies after another spell resolves
    Given Alice controls a creature on the battlefield with zero toughness
    And Alice is the active player in FirstMain
    And Alice has a castable spell in hand
    When Alice casts that spell
    Then the zero-toughness creature dies as part of state-based action review

  Scenario: A pending lethally damaged creature dies after a life adjustment
    Given Alice controls a creature on the battlefield with lethal damage marked on it
    When Alice's life total is adjusted
    Then the lethally damaged creature dies as part of state-based action review
