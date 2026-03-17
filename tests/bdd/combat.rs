use cucumber::{given, then, when};

use crate::world::GameplayWorld;

#[given("Alice attacks with a creature")]
fn alice_attacks_with_a_creature(world: &mut GameplayWorld) {
    world.setup_blocked_damage_marking();
}

#[given("Bob blocks with a creature")]
fn bob_blocks_with_a_creature(world: &mut GameplayWorld) {
    assert!(world.tracked_blocker_id.is_some());
    assert_eq!(world.blocker_assignments.len(), 1);
}

#[when("combat damage resolves")]
fn combat_damage_resolves(world: &mut GameplayWorld) {
    world.resolve_combat_damage();
}

#[when("combat damage resolution finishes")]
fn combat_damage_resolution_finishes(world: &mut GameplayWorld) {
    world.resolve_combat_damage();
}

#[then("the attacker's damage is marked on the blocking creature")]
fn attacker_damage_is_marked_on_the_blocking_creature(world: &mut GameplayWorld) {
    assert_eq!(world.tracked_blocker().damage(), 2);
}

#[then("the blocker's damage is marked on the attacking creature")]
fn blocker_damage_is_marked_on_the_attacking_creature(world: &mut GameplayWorld) {
    assert_eq!(world.tracked_attacker().damage(), 3);
}

#[then("the game emits CombatDamageResolved")]
fn the_game_emits_combat_damage_resolved(world: &mut GameplayWorld) {
    assert!(world.last_combat_damage.is_some());
}

#[given("Alice attacks with an unblocked creature")]
fn alice_attacks_with_an_unblocked_creature(world: &mut GameplayWorld) {
    world.setup_unblocked_combat();
}

#[given("Bob is the defending player")]
fn bob_is_the_defending_player(world: &mut GameplayWorld) {
    assert_eq!(world.player("Bob").life(), 20);
}

#[then("Bob loses life equal to the attacker's power")]
fn bob_loses_life_equal_to_the_attackers_power(world: &mut GameplayWorld) {
    assert_eq!(world.player_life("Bob"), 17);
}

#[given("a creature on the battlefield has damage marked on it equal to its toughness")]
fn a_creature_has_lethal_damage_marked(world: &mut GameplayWorld) {
    world.setup_lethal_damage_combat();
}

#[then("that creature leaves the battlefield")]
fn that_creature_leaves_the_battlefield(world: &mut GameplayWorld) {
    let card_id = world
        .tracked_card_id
        .as_ref()
        .expect("tracked creature should exist");
    assert!(!world.battlefield_contains("Alice", card_id));
}

#[then("that creature enters its controller's graveyard")]
fn that_creature_enters_its_controllers_graveyard(world: &mut GameplayWorld) {
    let card_id = world
        .tracked_card_id
        .as_ref()
        .expect("tracked creature should exist");
    assert!(world.graveyard_contains("Alice", card_id));
}

#[then("the game emits CreatureDied")]
fn the_game_emits_creature_died(world: &mut GameplayWorld) {
    assert!(!world.last_creature_died.is_empty());
}

#[given("a creature on the battlefield has damage marked on it less than its toughness")]
fn a_creature_has_nonlethal_damage_marked(world: &mut GameplayWorld) {
    world.setup_nonlethal_damage_combat();
}

#[then("that creature remains on the battlefield")]
fn that_creature_remains_on_the_battlefield(world: &mut GameplayWorld) {
    let card_id = world
        .tracked_card_id
        .as_ref()
        .expect("tracked creature should exist");
    assert!(world.battlefield_contains("Alice", card_id));
}

#[then("no CreatureDied event is emitted for that creature")]
fn no_creature_died_event_is_emitted(world: &mut GameplayWorld) {
    let card_id = world
        .tracked_card_id
        .as_ref()
        .expect("tracked creature should exist");
    assert!(!world
        .last_creature_died
        .iter()
        .any(|event| &event.card_id == card_id));
}

#[given("a creature survives combat with damage marked on it")]
fn a_creature_survives_combat_with_damage_marked(world: &mut GameplayWorld) {
    world.setup_end_step_with_surviving_damage();
    assert_eq!(world.tracked_card("Alice").damage(), 2);
}

#[when("the game advances from EndStep to the next player's Untap")]
fn the_game_advances_from_end_step_to_the_next_players_untap(world: &mut GameplayWorld) {
    world.advance_turn();
}

#[then("that surviving creature has no damage marked on it")]
fn that_surviving_creature_has_no_damage_marked_on_it(world: &mut GameplayWorld) {
    assert_eq!(world.game().phase().to_owned(), demonictutor::Phase::Untap);
    assert_eq!(world.tracked_card("Alice").damage(), 0);
}
