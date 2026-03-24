//! BDD coverage for bdd combat.

use {
    crate::world::GameplayWorld,
    cucumber::{given, then, when},
};

#[given("Alice attacks with a creature")]
fn alice_attacks_with_a_creature(world: &mut GameplayWorld) {
    world.setup_blocked_damage_marking();
}

#[given("Alice attacks with a trample creature and Bob blocks with a smaller creature")]
fn alice_attacks_with_a_trample_creature_and_bob_blocks_with_a_smaller_creature(
    world: &mut GameplayWorld,
) {
    world.setup_trample_combat();
}

#[given("Alice attacks with a first-strike creature and Bob blocks with an equal creature")]
fn alice_attacks_with_a_first_strike_creature_and_bob_blocks_with_an_equal_creature(
    world: &mut GameplayWorld,
) {
    world.setup_first_strike_combat();
}

#[given("Alice is in DeclareAttackers with a haste creature that entered this turn")]
fn alice_is_in_declare_attackers_with_a_haste_creature_that_entered_this_turn(
    world: &mut GameplayWorld,
) {
    world.setup_haste_attack();
}

#[given("Alice is in DeclareAttackers with a vigilance creature without summoning sickness")]
fn alice_is_in_declare_attackers_with_a_vigilance_creature_without_summoning_sickness(
    world: &mut GameplayWorld,
) {
    world.setup_vigilance_attack();
}

#[given("Alice has declared attackers in Combat")]
fn alice_has_declared_attackers_in_combat(world: &mut GameplayWorld) {
    world.setup_priority_after_attackers_declared();
}

#[given("Alice enters Combat from FirstMain")]
fn alice_enters_combat_from_first_main(world: &mut GameplayWorld) {
    world.setup_priority_when_entering_combat();
}

#[given("Bob has declared blockers in Combat")]
fn bob_has_declared_blockers_in_combat(world: &mut GameplayWorld) {
    world.setup_priority_after_blockers_declared();
}

#[given(
    "Alice attacks with a creature and Bob has two creatures that could block the same attacker"
)]
fn alice_attacks_with_a_creature_and_bob_has_two_creatures_that_could_block_the_same_attacker(
    world: &mut GameplayWorld,
) {
    world.setup_multiple_blockers_not_supported();
}

#[given("Bob blocks with a creature")]
fn bob_blocks_with_a_creature(world: &mut GameplayWorld) {
    assert!(world.tracked_blocker_id.is_some());
    assert_eq!(world.blocker_assignments.len(), 1);
}

#[when("combat damage resolves")]
fn combat_damage_resolves(world: &mut GameplayWorld) {
    world.resolve_combat_damage("Alice");
}

#[when("Alice declares that creature as an attacker")]
fn alice_declares_that_creature_as_an_attacker(world: &mut GameplayWorld) {
    let attacker_id = world
        .tracked_attacker_id
        .clone()
        .expect("tracked attacker should exist");
    world.try_declare_attacker("Alice", &attacker_id);
}

#[when("Bob tries to assign both blockers to that attacker")]
fn bob_tries_to_assign_both_blockers_to_that_attacker(world: &mut GameplayWorld) {
    world.try_declare_multiple_blockers_on_one_attacker("Bob");
}

#[when("combat damage resolution finishes")]
fn combat_damage_resolution_finishes(world: &mut GameplayWorld) {
    world.resolve_combat_damage("Alice");
}

#[then("the attacker's damage is marked on the blocking creature")]
fn attacker_damage_is_marked_on_the_blocking_creature(world: &mut GameplayWorld) {
    assert_eq!(world.tracked_blocker().damage(), 2);
}

#[then("the attacker declaration is accepted")]
fn the_attacker_declaration_is_accepted(world: &mut GameplayWorld) {
    assert!(
        world.last_error.is_none(),
        "unexpected error: {:?}",
        world.last_error
    );
}

#[then("that creature is attacking")]
fn that_creature_is_attacking(world: &mut GameplayWorld) {
    assert!(world.tracked_attacker().is_attacking());
}

#[then("that creature remains untapped")]
fn that_creature_remains_untapped(world: &mut GameplayWorld) {
    assert!(!world.tracked_attacker().is_tapped());
}

#[then("the blocker's damage is marked on the attacking creature")]
fn blocker_damage_is_marked_on_the_attacking_creature(world: &mut GameplayWorld) {
    assert_eq!(world.tracked_attacker().damage(), 3);
}

#[then("the game emits CombatDamageResolved")]
fn the_game_emits_combat_damage_resolved(world: &mut GameplayWorld) {
    assert!(world.last_combat_damage.is_some());
}

#[then("the action is rejected because multiple blockers per attacker are not yet supported")]
fn the_action_is_rejected_because_multiple_blockers_per_attacker_are_not_yet_supported(
    world: &mut GameplayWorld,
) {
    let error = world
        .last_error
        .as_ref()
        .expect("multiple blockers should be rejected");
    assert!(
        error.contains("cannot be assigned more than one blocker"),
        "unexpected error: {error}"
    );
}

#[given("Alice attacks with an unblocked creature")]
fn alice_attacks_with_an_unblocked_creature(world: &mut GameplayWorld) {
    world.setup_unblocked_combat();
}

#[given("Bob is the defending player")]
fn bob_is_the_defending_player(world: &mut GameplayWorld) {
    assert_eq!(world.player("Bob").life(), 20);
}

#[given("Bob is at 3 life as the defending player")]
fn bob_is_at_three_life_as_the_defending_player(world: &mut GameplayWorld) {
    world.setup_unblocked_combat_with_defender_life(3);
    assert_eq!(world.player("Bob").life(), 3);
}

#[then("Bob loses life equal to the attacker's power")]
fn bob_loses_life_equal_to_the_attackers_power(world: &mut GameplayWorld) {
    assert_eq!(world.player_life("Bob"), 17);
}

#[then("Bob loses 3 life")]
fn bob_loses_3_life(world: &mut GameplayWorld) {
    assert_eq!(world.player_life("Bob"), 17);
}

#[then("Bob loses 2 life from trample damage")]
fn bob_loses_2_life_from_trample_damage(world: &mut GameplayWorld) {
    assert_eq!(world.player_life("Bob"), 18);
}

#[then("Bob loses the game due to zero life")]
fn bob_loses_the_game_due_to_zero_life(world: &mut GameplayWorld) {
    let game_ended = world
        .last_game_ended
        .as_ref()
        .expect("combat should emit GameEnded");
    assert_eq!(game_ended.loser_id, GameplayWorld::player_id("Bob"));
    assert_eq!(game_ended.winner_id, GameplayWorld::player_id("Alice"));
    assert_eq!(world.player_life("Bob"), 0);
    assert!(world.game().is_over());
}

#[then("the game emits GameEnded for ZeroLife")]
fn the_game_emits_game_ended_for_zero_life(world: &mut GameplayWorld) {
    let game_ended = world
        .last_game_ended
        .as_ref()
        .expect("GameEnded event should exist");
    assert_eq!(game_ended.reason, demonictutor::GameEndReason::ZeroLife);
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

#[then("Alice's attacker survives combat")]
fn alices_attacker_survives_combat(world: &mut GameplayWorld) {
    let card_id = world
        .tracked_attacker_id
        .as_ref()
        .expect("tracked attacker should exist");
    assert!(world.battlefield_contains("Alice", card_id));
}

#[then("Alice's attacker gets +2/+2 until end of turn")]
fn alices_attacker_gets_plus_2_plus_2_until_end_of_turn(world: &mut GameplayWorld) {
    assert_eq!(world.tracked_attacker().creature_stats(), Some((4, 4)));
}

#[then("Bob's blocker dies in combat")]
fn bobs_blocker_dies_in_combat(world: &mut GameplayWorld) {
    let card_id = world
        .tracked_blocker_id
        .as_ref()
        .expect("tracked blocker should exist");
    assert!(!world.battlefield_contains("Bob", card_id));
    assert!(world.graveyard_contains("Bob", card_id));
}

#[then("Alice's attacker has no combat damage marked on it")]
fn alices_attacker_has_no_combat_damage_marked_on_it(world: &mut GameplayWorld) {
    assert_eq!(world.tracked_attacker().damage(), 0);
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

#[given("Alice attacks with a flying creature")]
fn alice_attacks_with_a_flying_creature(world: &mut GameplayWorld) {
    world.setup_flying_attack_and_block();
}

#[given("Bob controls a creature with flying")]
fn bob_controls_a_creature_with_flying(world: &mut GameplayWorld) {
    assert!(world.tracked_blocker().has_flying());
}

#[when("Bob declares that creature as a blocker against the flying attacker")]
fn bob_declares_that_creature_as_a_blocker_against_the_flying_attacker(world: &mut GameplayWorld) {
    let attacker_id = world.tracked_attacker_id.clone().unwrap();
    let blocker_id = world.tracked_blocker_id.clone().unwrap();
    world.declare_blocker_against("Bob", &blocker_id, &attacker_id);
}

#[then("the blocker assignment is accepted")]
fn the_blocker_assignment_is_accepted(world: &mut GameplayWorld) {
    assert!(world.last_error.is_none());
    assert_eq!(world.game().phase(), &demonictutor::Phase::CombatDamage);
}

#[given("Bob controls a creature with reach")]
fn bob_controls_a_creature_with_reach(world: &mut GameplayWorld) {
    world.setup_flying_attack_and_reach_block();
    assert!(world.tracked_blocker().has_reach());
}

#[given("Bob controls a creature without flying or reach")]
fn bob_controls_a_creature_without_flying_or_reach(world: &mut GameplayWorld) {
    world.setup_flying_attack_and_nonflying_block();
    assert!(!world.tracked_blocker().has_flying());
    assert!(!world.tracked_blocker().has_reach());
}

#[when("Bob tries to declare that creature as a blocker against the flying attacker")]
fn bob_tries_to_declare_that_creature_as_a_blocker_against_the_flying_attacker(
    world: &mut GameplayWorld,
) {
    let attacker_id = world.tracked_attacker_id.clone().unwrap();
    let blocker_id = world.tracked_blocker_id.clone().unwrap();
    world.try_declare_blocker_against("Bob", &blocker_id, &attacker_id);
}

#[then("the action is rejected because the blocker cannot block flying creatures")]
fn the_action_is_rejected_because_the_blocker_cannot_block_flying_creatures(
    world: &mut GameplayWorld,
) {
    let error = world
        .last_error
        .as_ref()
        .expect("action should be rejected");
    assert!(error.contains("cannot block flying creature"));
}

#[given("Alice attacks with a non-flying creature")]
fn alice_attacks_with_a_non_flying_creature(world: &mut GameplayWorld) {
    world.setup_nonflying_attack_and_block();
}

#[given("Bob controls a non-flying creature")]
fn bob_controls_a_non_flying_creature(world: &mut GameplayWorld) {
    assert!(!world.tracked_blocker().has_flying());
}

#[when("Bob declares that creature as a blocker")]
fn bob_declares_that_creature_as_a_blocker(world: &mut GameplayWorld) {
    let attacker_id = world.tracked_attacker_id.clone().unwrap();
    let blocker_id = world.tracked_blocker_id.clone().unwrap();
    world.declare_blocker_against("Bob", &blocker_id, &attacker_id);
}

#[given("Bob controls a creature with both flying and reach")]
fn bob_controls_a_creature_with_both_flying_and_reach(world: &mut GameplayWorld) {
    world.setup_flying_and_reach_block();
    assert!(world.tracked_blocker().has_flying());
    assert!(world.tracked_blocker().has_reach());
}

#[given("Alice attacks with a flying creature that has 3 power")]
fn alice_attacks_with_a_flying_creature_with_power(world: &mut GameplayWorld) {
    world.setup_unblocked_flying_attack();
    assert_eq!(world.tracked_attacker().power(), Some(3));
    assert!(world.tracked_attacker().has_flying());
}

#[given("Bob has no creatures that can block flying")]
fn bob_has_no_blockers_for_flying(world: &mut GameplayWorld) {
    assert!(world.player_battlefield_is_empty("Bob"));
}

#[given("the flying attacker is unblocked")]
fn the_flying_attacker_is_unblocked(world: &mut GameplayWorld) {
    assert!(world.blocker_assignments.is_empty());
}
