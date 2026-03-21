use cucumber::{given, when};

use crate::world::GameplayWorld;

#[given(
    "Alice has cast a creature spell and still holds priority with Bob's flash artifact in hand"
)]
fn alice_has_cast_a_creature_spell_and_still_holds_priority_with_bobs_flash_artifact_in_hand(
    world: &mut GameplayWorld,
) {
    world.setup_spell_response_stack_with_flash_artifact();
    world.ensure_tracked_land_provides_mana();
    world.cast_tracked_spell("Alice");
    let priority = world
        .game()
        .priority()
        .expect("priority window should be open");
    assert_eq!(
        priority.current_holder(),
        &GameplayWorld::player_id("Alice")
    );
}

#[when("Bob casts the flash artifact response spell")]
fn bob_casts_the_flash_artifact_response_spell(world: &mut GameplayWorld) {
    world.cast_tracked_response_spell("Bob");
}

#[given(
    "Alice has cast a creature spell and still holds priority with Bob's flash enchantment in hand"
)]
fn alice_has_cast_a_creature_spell_and_still_holds_priority_with_bobs_flash_enchantment_in_hand(
    world: &mut GameplayWorld,
) {
    world.setup_spell_response_stack_with_flash_enchantment();
    world.ensure_tracked_land_provides_mana();
    world.cast_tracked_spell("Alice");
    let priority = world
        .game()
        .priority()
        .expect("priority window should be open");
    assert_eq!(
        priority.current_holder(),
        &GameplayWorld::player_id("Alice")
    );
}

#[when("Bob casts the flash enchantment response spell")]
fn bob_casts_the_flash_enchantment_response_spell(world: &mut GameplayWorld) {
    world.cast_tracked_response_spell("Bob");
}
