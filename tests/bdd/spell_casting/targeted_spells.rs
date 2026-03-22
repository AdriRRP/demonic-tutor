use cucumber::{given, then, when};

use crate::world::GameplayWorld;

#[given("Alice is the active player in FirstMain with a targeted instant spell in hand")]
fn alice_is_the_active_player_in_first_main_with_a_targeted_instant_spell_in_hand(
    world: &mut GameplayWorld,
) {
    world.setup_targeted_player_spell();
}

#[given("Alice is the active player in FirstMain with an opponent-targeted instant spell in hand")]
fn alice_is_the_active_player_in_first_main_with_an_opponent_targeted_instant_spell_in_hand(
    world: &mut GameplayWorld,
) {
    world.setup_targeted_opponent_player_spell();
}

#[given("Bob is a valid target player")]
fn bob_is_a_valid_target_player(_world: &mut GameplayWorld) {}

#[given("Alice is the active player in FirstMain with a lethal targeted instant spell in hand")]
fn alice_is_the_active_player_in_first_main_with_a_lethal_targeted_instant_spell_in_hand(
    world: &mut GameplayWorld,
) {
    world.setup_lethal_targeted_player_spell();
}

#[given("Bob is at 2 life")]
fn bob_is_at_2_life(world: &mut GameplayWorld) {
    assert_eq!(world.player_life("Bob"), 2);
}

#[given("Alice is the active player in FirstMain with a targeted instant spell and Bob's creature on the battlefield")]
fn alice_is_the_active_player_in_first_main_with_a_targeted_instant_spell_and_bobs_creature_on_the_battlefield(
    world: &mut GameplayWorld,
) {
    world.setup_targeted_creature_spell();
}

#[given("Alice is the active player in FirstMain with a controlled-creature instant spell and Alice's creature on the battlefield")]
fn alice_is_the_active_player_in_first_main_with_a_controlled_creature_instant_spell_and_alices_creature_on_the_battlefield(
    world: &mut GameplayWorld,
) {
    world.setup_targeted_controlled_creature_spell();
}

#[given("Alice is the active player in FirstMain with a controlled-creature instant spell and only Bob's creature on the battlefield")]
fn alice_is_the_active_player_in_first_main_with_a_controlled_creature_instant_spell_and_only_bobs_creature_on_the_battlefield(
    world: &mut GameplayWorld,
) {
    world.setup_targeted_controlled_creature_spell_with_opponents_creature();
}

#[given("Alice is the active player in FirstMain with an opponents-creature instant spell and Bob's creature on the battlefield")]
fn alice_is_the_active_player_in_first_main_with_an_opponents_creature_instant_spell_and_bobs_creature_on_the_battlefield(
    world: &mut GameplayWorld,
) {
    world.setup_targeted_opponents_creature_spell();
}

#[given("Alice is the active player in FirstMain with a destroy-creature instant spell and Bob's creature on the battlefield")]
fn alice_is_the_active_player_in_first_main_with_a_destroy_creature_instant_spell_and_bobs_creature_on_the_battlefield(
    world: &mut GameplayWorld,
) {
    world.setup_destroy_target_creature_spell();
}

#[given("Alice is the active player in FirstMain with an exile-creature instant spell and Bob's creature on the battlefield")]
fn alice_is_the_active_player_in_first_main_with_an_exile_creature_instant_spell_and_bobs_creature_on_the_battlefield(
    world: &mut GameplayWorld,
) {
    world.setup_exile_target_creature_spell();
}

#[given("Alice is the active player in FirstMain with an exile-graveyard-card instant spell and Bob's card in the graveyard")]
fn alice_is_the_active_player_in_first_main_with_an_exile_graveyard_card_instant_spell_and_bobs_card_in_the_graveyard(
    world: &mut GameplayWorld,
) {
    world.setup_exile_target_graveyard_card_spell();
}

#[given("Alice is the active player in FirstMain with a pump-creature instant spell and her creature on the battlefield")]
fn alice_is_the_active_player_in_first_main_with_a_pump_creature_instant_spell_and_her_creature_on_the_battlefield(
    world: &mut GameplayWorld,
) {
    world.setup_pump_target_creature_spell();
}

#[given("Alice is the active player in FirstMain with an opponents-creature instant spell and only her creature on the battlefield")]
fn alice_is_the_active_player_in_first_main_with_an_opponents_creature_instant_spell_and_only_her_creature_on_the_battlefield(
    world: &mut GameplayWorld,
) {
    world.setup_targeted_opponents_creature_spell_with_controlled_creature();
}

#[when("Alice casts the targeted instant spell targeting Bob")]
fn alice_casts_the_targeted_instant_spell_targeting_bob(world: &mut GameplayWorld) {
    world.cast_tracked_targeted_player_spell("Alice", "Bob");
}

#[when("Alice casts the targeted instant spell targeting herself")]
fn alice_casts_the_targeted_instant_spell_targeting_herself(world: &mut GameplayWorld) {
    world.cast_tracked_targeted_player_spell("Alice", "Alice");
}

#[when("Alice casts the opponent-targeted instant spell targeting Bob")]
fn alice_casts_the_opponent_targeted_instant_spell_targeting_bob(world: &mut GameplayWorld) {
    world.cast_tracked_targeted_player_spell("Alice", "Bob");
}

#[when("Alice tries to cast the opponent-targeted instant spell targeting herself")]
fn alice_tries_to_cast_the_opponent_targeted_instant_spell_targeting_herself(
    world: &mut GameplayWorld,
) {
    world.try_cast_tracked_targeted_player_spell("Alice", "Alice");
}

#[when("Alice casts the targeted instant spell without a target")]
fn alice_casts_the_targeted_instant_spell_without_a_target(world: &mut GameplayWorld) {
    world.try_cast_tracked_spell("Alice");
}

#[when("Alice casts the targeted instant spell targeting a missing player")]
fn alice_casts_the_targeted_instant_spell_targeting_a_missing_player(world: &mut GameplayWorld) {
    world.try_cast_tracked_targeted_player_spell("Alice", "missing-player");
}

#[when("Alice casts the targeted instant spell targeting Bob's creature")]
fn alice_casts_the_targeted_instant_spell_targeting_bobs_creature(world: &mut GameplayWorld) {
    world.cast_tracked_targeted_creature_spell("Alice");
}

#[when("Alice casts the controlled-creature instant spell targeting her creature")]
fn alice_casts_the_controlled_creature_instant_spell_targeting_her_creature(
    world: &mut GameplayWorld,
) {
    world.cast_tracked_targeted_creature_spell("Alice");
}

#[when("Alice tries to cast the controlled-creature instant spell targeting Bob's creature")]
fn alice_tries_to_cast_the_controlled_creature_instant_spell_targeting_bobs_creature(
    world: &mut GameplayWorld,
) {
    world.try_cast_tracked_targeted_creature_spell("Alice");
}

#[when("Alice casts the opponents-creature instant spell targeting Bob's creature")]
fn alice_casts_the_opponents_creature_instant_spell_targeting_bobs_creature(
    world: &mut GameplayWorld,
) {
    world.cast_tracked_targeted_creature_spell("Alice");
}

#[when("Alice casts the destroy-creature instant spell targeting Bob's creature")]
fn alice_casts_the_destroy_creature_instant_spell_targeting_bobs_creature(
    world: &mut GameplayWorld,
) {
    world.cast_tracked_targeted_creature_spell("Alice");
}

#[when("Alice casts the exile-creature instant spell targeting Bob's creature")]
fn alice_casts_the_exile_creature_instant_spell_targeting_bobs_creature(world: &mut GameplayWorld) {
    world.cast_tracked_targeted_creature_spell("Alice");
}

#[when("Alice casts the exile-graveyard-card instant spell targeting Bob's graveyard card")]
fn alice_casts_the_exile_graveyard_card_instant_spell_targeting_bobs_graveyard_card(
    world: &mut GameplayWorld,
) {
    world.cast_tracked_targeted_graveyard_card_spell("Alice");
}

#[when("Alice casts the pump-creature instant spell targeting her creature")]
fn alice_casts_the_pump_creature_instant_spell_targeting_her_creature(world: &mut GameplayWorld) {
    world.cast_tracked_targeted_creature_spell("Alice");
}

#[when("Alice tries to cast the opponents-creature instant spell targeting her creature")]
fn alice_tries_to_cast_the_opponents_creature_instant_spell_targeting_her_creature(
    world: &mut GameplayWorld,
) {
    world.try_cast_tracked_targeted_creature_spell("Alice");
}

#[then("the spell is on the stack targeting Bob")]
fn the_spell_is_on_the_stack_targeting_bob(world: &mut GameplayWorld) {
    let event = world
        .last_spell_put_on_stack
        .as_ref()
        .expect("expected targeted spell on stack");
    assert_eq!(
        event.target,
        Some(demonictutor::SpellTarget::Player(GameplayWorld::player_id(
            "Bob"
        )))
    );
}

#[then("the spell is on the stack targeting Alice")]
fn the_spell_is_on_the_stack_targeting_alice(world: &mut GameplayWorld) {
    let event = world
        .last_spell_put_on_stack
        .as_ref()
        .expect("expected targeted spell on stack");
    assert_eq!(
        event.target,
        Some(demonictutor::SpellTarget::Player(GameplayWorld::player_id(
            "Alice"
        )))
    );
}

#[then("the spell is on the stack targeting Alice's creature")]
fn the_spell_is_on_the_stack_targeting_alices_creature(world: &mut GameplayWorld) {
    let event = world
        .last_spell_put_on_stack
        .as_ref()
        .expect("expected targeted spell on stack");
    assert_eq!(
        event.target,
        Some(demonictutor::SpellTarget::Creature(
            world
                .tracked_blocker_id
                .clone()
                .expect("tracked creature should exist")
        ))
    );
}

#[then("the spell is on the stack targeting Alice's attacker")]
fn the_spell_is_on_the_stack_targeting_alices_attacker(world: &mut GameplayWorld) {
    let event = world
        .last_spell_put_on_stack
        .as_ref()
        .expect("expected targeted spell on stack");
    assert_eq!(
        event.target,
        Some(demonictutor::SpellTarget::Creature(
            world
                .tracked_attacker_id
                .clone()
                .expect("tracked attacker should exist")
        ))
    );
}

#[then("the spell is on the stack targeting Bob's creature")]
fn the_spell_is_on_the_stack_targeting_bobs_creature(world: &mut GameplayWorld) {
    let event = world
        .last_spell_put_on_stack
        .as_ref()
        .expect("expected targeted spell on stack");
    assert_eq!(
        event.target,
        Some(demonictutor::SpellTarget::Creature(
            world
                .tracked_blocker_id
                .clone()
                .expect("tracked creature should exist")
        ))
    );
}

#[then("Bob's creature is in exile")]
fn bobs_creature_is_in_exile(world: &mut GameplayWorld) {
    let card_id = world
        .tracked_blocker_id
        .as_ref()
        .expect("tracked creature should exist");
    assert!(world.exile_contains("Bob", card_id));
}

#[then("the spell is on the stack targeting Bob's graveyard card")]
fn the_spell_is_on_the_stack_targeting_bobs_graveyard_card(world: &mut GameplayWorld) {
    let event = world
        .last_spell_put_on_stack
        .as_ref()
        .expect("expected targeted spell on stack");
    assert_eq!(
        event.target,
        Some(demonictutor::SpellTarget::GraveyardCard(
            world
                .tracked_blocker_id
                .clone()
                .expect("tracked graveyard card should exist")
        ))
    );
}

#[then("Bob's graveyard card is in exile")]
fn bobs_graveyard_card_is_in_exile(world: &mut GameplayWorld) {
    let card_id = world
        .tracked_blocker_id
        .as_ref()
        .expect("tracked graveyard card should exist");
    assert!(world.exile_contains("Bob", card_id));
}

#[then("Alice's creature gets +2/+2 until end of turn")]
fn alices_creature_gets_plus_2_plus_2_until_end_of_turn(world: &mut GameplayWorld) {
    let creature_id = world
        .tracked_blocker_id
        .as_ref()
        .expect("tracked creature should exist");
    assert_eq!(
        world
            .battlefield_card("Alice", creature_id)
            .creature_stats(),
        Some((4, 4))
    );
}

#[then("Bob loses 2 life")]
fn bob_loses_2_life(world: &mut GameplayWorld) {
    let event = world
        .last_life_changed
        .as_ref()
        .expect("expected life changed event");
    assert_eq!(event.player_id, GameplayWorld::player_id("Bob"));
    assert_eq!(event.from_life, 20);
    assert_eq!(event.to_life, 18);
}

#[then("Alice loses 2 life")]
fn alice_loses_2_life(world: &mut GameplayWorld) {
    let event = world
        .last_life_changed
        .as_ref()
        .expect("expected life changed event");
    assert_eq!(event.player_id, GameplayWorld::player_id("Alice"));
    assert_eq!(event.from_life, 20);
    assert_eq!(event.to_life, 18);
}

#[then("the game ends with Bob losing")]
fn the_game_ends_with_bob_losing(world: &mut GameplayWorld) {
    let event = world
        .last_game_ended
        .as_ref()
        .expect("expected game end event");
    assert_eq!(event.loser_id, GameplayWorld::player_id("Bob"));
}

#[then("casting fails because the spell target is missing")]
fn casting_fails_because_the_spell_target_is_missing(world: &mut GameplayWorld) {
    assert!(world
        .last_error
        .as_ref()
        .is_some_and(|error| error.contains("requires an explicit target")));
}

#[then("casting fails because the target player does not exist")]
fn casting_fails_because_the_target_player_does_not_exist(world: &mut GameplayWorld) {
    assert!(world
        .last_error
        .as_ref()
        .is_some_and(|error| error.contains("missing-player")));
}

#[then("casting fails because the spell requires an opponent target")]
fn casting_fails_because_the_spell_requires_an_opponent_target(world: &mut GameplayWorld) {
    assert!(world
        .last_error
        .as_ref()
        .is_some_and(|error| error.contains("cannot use the provided target")));
}

#[then("casting fails because the spell requires a controlled creature target")]
fn casting_fails_because_the_spell_requires_a_controlled_creature_target(
    world: &mut GameplayWorld,
) {
    assert!(world
        .last_error
        .as_ref()
        .is_some_and(|error| error.contains("cannot use the provided target")));
}

#[then("casting fails because the spell requires an opponents-creature target")]
fn casting_fails_because_the_spell_requires_an_opponents_creature_target(
    world: &mut GameplayWorld,
) {
    assert!(world
        .last_error
        .as_ref()
        .is_some_and(|error| error.contains("cannot use the provided target")));
}

#[then("Bob's creature dies")]
fn bobs_creature_dies(world: &mut GameplayWorld) {
    assert_eq!(world.last_creature_died.len(), 1);
    let creature_id = world
        .tracked_blocker_id
        .as_ref()
        .expect("tracked creature target should exist");
    assert_eq!(world.last_creature_died[0].card_id, *creature_id);
}

#[then("Alice's creature dies")]
fn alices_creature_dies(world: &mut GameplayWorld) {
    assert_eq!(world.last_creature_died.len(), 1);
    let creature_id = world
        .tracked_blocker_id
        .as_ref()
        .expect("tracked creature target should exist");
    assert_eq!(world.last_creature_died[0].card_id, *creature_id);
}
