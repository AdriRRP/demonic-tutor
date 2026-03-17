use cucumber::{given, then, when};
use demonictutor::{CardInstance, CardType, Phase, SpellCastOutcome};

use crate::world::GameplayWorld;

#[given(expr = "{word} is the active player in {word}")]
fn player_is_active_in_phase(world: &mut GameplayWorld, player: String, phase: String) {
    let expected_phase = GameplayWorld::phase_from_name(&phase);
    if !world.is_initialized() {
        world.setup_turn_state_satisfying_cleanup(expected_phase, &player, 1);
    }
    assert_eq!(player, "Alice");
    assert_eq!(
        world.game().active_player(),
        &GameplayWorld::player_id(&player)
    );
    assert_eq!(world.game().phase(), &expected_phase);
}

#[given("Alice has a creature card in hand with valid power and toughness")]
fn alice_has_a_creature_card_in_hand_with_valid_power_and_toughness(world: &mut GameplayWorld) {
    world.setup_cast_creature_spell();

    let card_id = world
        .tracked_card_id
        .as_ref()
        .expect("tracked creature card should exist");
    let card = world
        .player("Alice")
        .hand()
        .cards()
        .iter()
        .find(|card: &&CardInstance| card.id() == card_id)
        .expect("creature card should be in hand");

    assert_eq!(card.card_type(), &CardType::Creature);
    assert_eq!(card.creature_stats(), Some((2, 2)));
}

#[given("Alice has cast a creature spell and still holds priority with Bob's instant in hand")]
fn alice_has_cast_a_creature_spell_and_still_holds_priority_with_bobs_instant_in_hand(
    world: &mut GameplayWorld,
) {
    world.setup_spell_response_stack();
    world.ensure_tracked_land_provides_mana();
    world.cast_tracked_spell("Alice");
    alice_has_priority(world);
}

#[given(
    "Alice has cast an instant spell and still holds priority with Bob's creature card in hand"
)]
fn alice_has_cast_an_instant_spell_and_still_holds_priority_with_bobs_creature_card_in_hand(
    world: &mut GameplayWorld,
) {
    world.setup_invalid_noninstant_response();
    world.cast_tracked_spell("Alice");
    alice_has_priority(world);
}

#[given("Alice has enough mana to pay its cost")]
fn alice_has_enough_mana_to_pay_its_cost(world: &mut GameplayWorld) {
    world.ensure_tracked_land_provides_mana();
    assert_eq!(world.player("Alice").mana(), 1);
}

#[when("Alice casts the creature spell")]
fn alice_casts_the_creature_spell(world: &mut GameplayWorld) {
    world.cast_tracked_spell("Alice");
}

#[when("Bob casts the instant response spell")]
fn bob_casts_the_instant_response_spell(world: &mut GameplayWorld) {
    world.cast_tracked_response_spell("Bob");
}

#[when("Bob tries to cast the creature response spell")]
fn bob_tries_to_cast_the_creature_response_spell(world: &mut GameplayWorld) {
    world.cast_tracked_response_spell("Bob");
}

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

#[then("the card leaves Alice's hand")]
fn the_card_leaves_alices_hand(world: &mut GameplayWorld) {
    let card_id = world
        .tracked_card_id
        .as_ref()
        .expect("tracked card should exist");
    assert!(!world.hand_contains("Alice", card_id));
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

#[then("Bob's instant is on top of the stack under Bob's control")]
fn bobs_instant_is_on_top_of_the_stack_under_bobs_control(world: &mut GameplayWorld) {
    let top = world
        .game()
        .stack()
        .top()
        .expect("stack should contain a top spell");
    assert_eq!(top.controller_id(), &GameplayWorld::player_id("Bob"));
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

#[given("Bob has priority")]
#[then("Bob has priority")]
fn bob_has_priority(world: &mut GameplayWorld) {
    let priority = world
        .game()
        .priority()
        .expect("priority window should be open after casting");
    assert_eq!(priority.current_holder(), &GameplayWorld::player_id("Bob"));
}

#[then("the game emits SpellPutOnStack")]
fn the_game_emits_spell_put_on_stack(world: &mut GameplayWorld) {
    assert!(world.last_spell_put_on_stack.is_some());
}

#[then("the card enters Alice's battlefield")]
fn the_card_enters_alices_battlefield(world: &mut GameplayWorld) {
    let card_id = world
        .tracked_card_id
        .as_ref()
        .expect("tracked card should exist");
    assert!(world.battlefield_contains("Alice", card_id));
}

#[then("the card has summoning sickness")]
fn the_card_has_summoning_sickness(world: &mut GameplayWorld) {
    assert!(world.tracked_card("Alice").has_summoning_sickness());
}

#[then("the game emits StackTopResolved")]
fn the_game_emits_stack_top_resolved(world: &mut GameplayWorld) {
    assert!(world.last_stack_top_resolved.is_some());
}

#[then(expr = "the game emits SpellCast with outcome {word}")]
fn the_game_emits_spell_cast_with_outcome(world: &mut GameplayWorld, outcome: String) {
    let expected = match outcome.as_str() {
        "EnteredBattlefield" => SpellCastOutcome::EnteredBattlefield,
        "ResolvedToGraveyard" => SpellCastOutcome::ResolvedToGraveyard,
        other => panic!("unsupported spell outcome in BDD suite: {other}"),
    };

    let event = world
        .last_spell_cast
        .as_ref()
        .expect("expected a SpellCast event");
    assert!(matches!(
        (&event.outcome, expected),
        (
            SpellCastOutcome::EnteredBattlefield,
            SpellCastOutcome::EnteredBattlefield
        ) | (
            SpellCastOutcome::ResolvedToGraveyard,
            SpellCastOutcome::ResolvedToGraveyard
        )
    ));
}

#[then("the action is rejected because only instant responses are currently supported")]
fn the_action_is_rejected_because_only_instant_responses_are_currently_supported(
    world: &mut GameplayWorld,
) {
    let error = world
        .last_error
        .as_ref()
        .expect("response cast should be rejected");
    assert!(
        error.contains("only supports instant response spells"),
        "unexpected error: {error}"
    );
}

#[given("Alice has a land card in hand")]
fn alice_has_a_land_card_in_hand(world: &mut GameplayWorld) {
    world.setup_cast_land_as_spell();
    let card_id = world
        .tracked_card_id
        .as_ref()
        .expect("tracked land card should exist");
    let card = world
        .player("Alice")
        .hand()
        .cards()
        .iter()
        .find(|card: &&CardInstance| card.id() == card_id)
        .expect("land card should be in hand");

    assert_eq!(card.card_type(), &CardType::Land);
}

#[given("Alice is the active player in Upkeep with an instant card in hand and priority")]
fn alice_is_the_active_player_in_upkeep_with_an_instant_card_in_hand_and_priority(
    world: &mut GameplayWorld,
) {
    world.setup_active_priority_window_with_instant("bdd-upkeep-instant-window", Phase::Upkeep);
}

#[given("Alice is the active player in Draw with an instant card in hand and priority")]
fn alice_is_the_active_player_in_draw_with_an_instant_card_in_hand_and_priority(
    world: &mut GameplayWorld,
) {
    world.setup_active_priority_window_with_instant("bdd-draw-instant-window", Phase::Draw);
}

#[given("Alice is the active player in SecondMain with an instant card in hand and priority")]
fn alice_is_the_active_player_in_second_main_with_an_instant_card_in_hand_and_priority(
    world: &mut GameplayWorld,
) {
    world.setup_active_priority_window_with_instant(
        "bdd-second-main-instant-window",
        Phase::SecondMain,
    );
}

#[given("Alice is the active player in EndStep with an instant card in hand and priority")]
fn alice_is_the_active_player_in_end_step_with_an_instant_card_in_hand_and_priority(
    world: &mut GameplayWorld,
) {
    world.setup_active_priority_window_with_instant("bdd-end-step-instant-window", Phase::EndStep);
}

#[given("Alice is the active player in Upkeep with two instant cards in hand and priority")]
fn alice_is_the_active_player_in_upkeep_with_two_instant_cards_in_hand_and_priority(
    world: &mut GameplayWorld,
) {
    world.setup_active_priority_window_with_two_instants("bdd-upkeep-two-instants", Phase::Upkeep);
}

#[given("Alice is the active player in Draw with two instant cards in hand and priority")]
fn alice_is_the_active_player_in_draw_with_two_instant_cards_in_hand_and_priority(
    world: &mut GameplayWorld,
) {
    world.setup_active_priority_window_with_two_instants("bdd-draw-two-instants", Phase::Draw);
}

#[given("Alice is the active player in SecondMain with two instant cards in hand and priority")]
fn alice_is_the_active_player_in_second_main_with_two_instant_cards_in_hand_and_priority(
    world: &mut GameplayWorld,
) {
    world.setup_active_priority_window_with_two_instants(
        "bdd-second-main-two-instants",
        Phase::SecondMain,
    );
}

#[given("Alice is the active player in EndStep with two instant cards in hand and priority")]
fn alice_is_the_active_player_in_end_step_with_two_instant_cards_in_hand_and_priority(
    world: &mut GameplayWorld,
) {
    world.setup_active_priority_window_with_two_instants(
        "bdd-end-step-two-instants",
        Phase::EndStep,
    );
}

#[given("Alice is at the beginning of Combat with two instant cards in hand and priority")]
fn alice_is_at_the_beginning_of_combat_with_two_instant_cards_in_hand_and_priority(
    world: &mut GameplayWorld,
) {
    world.setup_priority_when_entering_combat_with_two_instants();
}

#[given("Alice is at the beginning of Combat with an instant card in hand and priority")]
fn alice_is_at_the_beginning_of_combat_with_an_instant_card_in_hand_and_priority(
    world: &mut GameplayWorld,
) {
    world.setup_priority_when_entering_combat_with_instant();
}

#[given("Alice has declared attackers and still has an instant card in hand with priority")]
fn alice_has_declared_attackers_and_still_has_an_instant_card_in_hand_with_priority(
    world: &mut GameplayWorld,
) {
    world.setup_priority_after_attackers_declared_with_instant();
}

#[given("Bob has declared blockers and Alice still has an instant card in hand with priority")]
fn bob_has_declared_blockers_and_alice_still_has_an_instant_card_in_hand_with_priority(
    world: &mut GameplayWorld,
) {
    world.setup_priority_after_blockers_declared_with_instant();
}

#[given("combat damage has resolved and Alice still has an instant card in hand with priority")]
fn combat_damage_has_resolved_and_alice_still_has_an_instant_card_in_hand_with_priority(
    world: &mut GameplayWorld,
) {
    world.setup_priority_after_combat_damage_with_instant();
}

#[when("Alice casts the instant spell")]
fn alice_casts_the_instant_spell(world: &mut GameplayWorld) {
    world.cast_tracked_spell("Alice");
}

#[given("Bob has priority in Upkeep with an instant card in hand")]
fn bob_has_priority_in_upkeep_with_an_instant_card_in_hand(world: &mut GameplayWorld) {
    world
        .setup_non_active_priority_window_with_instant("bdd-upkeep-response-window", Phase::Upkeep);
}

#[given("Bob has priority in Draw with an instant card in hand")]
fn bob_has_priority_in_draw_with_an_instant_card_in_hand(world: &mut GameplayWorld) {
    world.setup_non_active_priority_window_with_instant("bdd-draw-response-window", Phase::Draw);
}

#[given("Bob has priority in FirstMain with an instant card in hand")]
fn bob_has_priority_in_first_main_with_an_instant_card_in_hand(world: &mut GameplayWorld) {
    world.setup_non_active_priority_window_with_instant(
        "bdd-first-main-response-window",
        Phase::FirstMain,
    );
}

#[given("Bob has priority in SecondMain with an instant card in hand")]
fn bob_has_priority_in_second_main_with_an_instant_card_in_hand(world: &mut GameplayWorld) {
    world.setup_non_active_priority_window_with_instant(
        "bdd-second-main-response-window",
        Phase::SecondMain,
    );
}

#[given("Bob has priority in EndStep with an instant card in hand")]
fn bob_has_priority_in_end_step_with_an_instant_card_in_hand(world: &mut GameplayWorld) {
    world.setup_non_active_priority_window_with_instant(
        "bdd-end-step-response-window",
        Phase::EndStep,
    );
}

#[given("Bob has priority at the beginning of Combat with an instant card in hand")]
fn bob_has_priority_at_the_beginning_of_combat_with_an_instant_card_in_hand(
    world: &mut GameplayWorld,
) {
    world.setup_non_active_priority_when_entering_combat_with_instant();
}

#[given("Bob has priority after attackers are declared with an instant card in hand")]
fn bob_has_priority_after_attackers_are_declared_with_an_instant_card_in_hand(
    world: &mut GameplayWorld,
) {
    world.setup_non_active_priority_after_attackers_declared_with_instant();
}

#[given("Bob has priority after blockers are declared with an instant card in hand")]
fn bob_has_priority_after_blockers_are_declared_with_an_instant_card_in_hand(
    world: &mut GameplayWorld,
) {
    world.setup_non_active_priority_after_blockers_declared_with_instant();
}

#[given("Bob has priority after combat damage with an instant card in hand")]
fn bob_has_priority_after_combat_damage_with_an_instant_card_in_hand(world: &mut GameplayWorld) {
    world.setup_non_active_priority_after_combat_damage_with_instant();
}

#[when("Bob casts the instant spell")]
fn bob_casts_the_instant_spell(world: &mut GameplayWorld) {
    world.cast_tracked_response_spell("Bob");
}

#[when("Alice casts the second instant spell")]
fn alice_casts_the_second_instant_spell(world: &mut GameplayWorld) {
    world.cast_tracked_response_spell("Alice");
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

#[then("the stack contains 2 spells controlled by Alice")]
fn the_stack_contains_two_spells_controlled_by_alice(world: &mut GameplayWorld) {
    assert_eq!(world.game().stack().len(), 2);
    for object in world.game().stack().objects() {
        assert_eq!(object.controller_id(), &GameplayWorld::player_id("Alice"));
    }
}

#[when("Alice tries to cast the card as a spell")]
fn alice_tries_to_cast_the_card_as_a_spell(world: &mut GameplayWorld) {
    world.cast_tracked_spell("Alice");
}

#[then("the action is rejected")]
fn the_action_is_rejected(world: &mut GameplayWorld) {
    assert!(world.last_error.is_some());
}

#[then("the land remains in Alice's hand")]
fn the_land_remains_in_alices_hand(world: &mut GameplayWorld) {
    let card_id = world
        .tracked_card_id
        .as_ref()
        .expect("tracked land should exist");
    assert!(world.hand_contains("Alice", card_id));
}
