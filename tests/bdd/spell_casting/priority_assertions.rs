//! BDD coverage for bdd spell casting priority assertions.

use {
    crate::world::GameplayWorld,
    cucumber::{given, then, when},
};

#[when(expr = "{word} passes priority")]
fn player_passes_priority(world: &mut GameplayWorld, player: String) {
    world.pass_priority(&player);
}

#[given("Alice has priority")]
#[then("Alice has priority")]
fn alice_has_priority(world: &mut GameplayWorld) {
    let priority = world
        .game()
        .priority()
        .expect("priority window should be open");
    assert_eq!(
        priority.current_holder(),
        &GameplayWorld::player_id("Alice")
    );
}

#[given("Bob has priority")]
#[then("Bob has priority")]
fn bob_has_priority(world: &mut GameplayWorld) {
    let priority = world
        .game()
        .priority()
        .expect("priority window should be open after casting");
    assert_eq!(priority.current_holder(), &GameplayWorld::player_id("Bob"));
}

#[then("Alice has priority again")]
fn alice_has_priority_again(world: &mut GameplayWorld) {
    let priority = world
        .game()
        .priority()
        .expect("priority window should be open");
    assert_eq!(
        priority.current_holder(),
        &GameplayWorld::player_id("Alice")
    );
}

#[then("the spell is on the stack under Alice's control")]
fn the_spell_is_on_the_stack_under_alices_control(world: &mut GameplayWorld) {
    let event = world
        .last_spell_put_on_stack
        .as_ref()
        .expect("expected a SpellPutOnStack event");
    let top = world
        .game()
        .stack()
        .top()
        .expect("stack should contain a spell");
    assert_eq!(event.player_id, GameplayWorld::player_id("Alice"));
    assert_eq!(top.controller_id(), &GameplayWorld::player_id("Alice"));
}

#[then("the spell is on the stack under Bob's control")]
fn the_spell_is_on_the_stack_under_bobs_control(world: &mut GameplayWorld) {
    let top = world
        .game()
        .stack()
        .top()
        .expect("stack should contain a top spell");
    assert_eq!(top.controller_id(), &GameplayWorld::player_id("Bob"));
}

#[then("Bob's instant is on top of the stack under Bob's control")]
fn bobs_instant_is_on_top_of_the_stack_under_bobs_control(world: &mut GameplayWorld) {
    let top = world
        .game()
        .stack()
        .top()
        .expect("stack should contain a top spell");
    assert_eq!(top.controller_id(), &GameplayWorld::player_id("Bob"));
}

#[then("Alice's original spell remains on the stack")]
fn alices_original_spell_remains_on_the_stack(world: &mut GameplayWorld) {
    let tracked_card_id = world
        .tracked_card_id
        .as_ref()
        .expect("original tracked card should exist");
    assert_eq!(world.game().stack().len(), 1);
    assert_eq!(
        world
            .game()
            .stack()
            .top()
            .expect("stack should contain original spell")
            .source_card_id(),
        tracked_card_id
    );
}

#[then("the spell has not resolved yet")]
fn the_spell_has_not_resolved_yet(world: &mut GameplayWorld) {
    assert!(world.last_spell_cast.is_none());
}
