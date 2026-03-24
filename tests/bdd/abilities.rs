//! BDD coverage for bdd abilities.

use {
    crate::world::GameplayWorld,
    cucumber::{given, then, when},
};

#[given("Alice is in first main with a life-gain artifact on the battlefield and priority")]
fn alice_is_in_first_main_with_a_life_gain_artifact_on_the_battlefield_and_priority(
    world: &mut GameplayWorld,
) {
    world.setup_activated_life_ability_in_first_main();
    assert_eq!(world.game().phase(), &demonictutor::Phase::FirstMain);
    assert_eq!(
        world.game().priority().unwrap().current_holder(),
        &GameplayWorld::player_id("Alice")
    );
}

#[when(expr = "{word} activates the tracked ability")]
fn player_activates_the_tracked_ability(world: &mut GameplayWorld, player: String) {
    world.activate_tracked_ability(&player);
}

#[then("the activated ability is on the stack under Alice's control")]
fn the_activated_ability_is_on_the_stack_under_alices_control(world: &mut GameplayWorld) {
    let event = world
        .last_activated_ability_put_on_stack
        .as_ref()
        .expect("expected an ActivatedAbilityPutOnStack event");
    let top = world
        .game()
        .stack()
        .top()
        .expect("stack should contain an ability");
    assert_eq!(event.player_id, GameplayWorld::player_id("Alice"));
    assert_eq!(top.controller_index(), 0);
    assert_eq!(world.game().stack().len(), 1);
}

#[then("the tracked permanent is tapped")]
fn the_tracked_permanent_is_tapped(world: &mut GameplayWorld) {
    assert!(world.tracked_card("Alice").is_tapped());
}
