#![allow(clippy::unwrap_used)]

use crate::support::{
    advance_to_first_main_satisfying_cleanup, advance_to_player_first_main_satisfying_cleanup,
    artifact_card, filled_library, instant_card, land_card, setup_two_player_game,
    vanilla_creature,
};
use demonictutor::{
    domain::play::game::{Player, TerminalState},
    CardDefinitionId, CardError, CardInstance, CardInstanceId, CardType, CastSpellCommand,
    DomainError, Game, GameError, GameId, LibraryCard, Phase, PlayLandCommand, PlayerId,
    SpellCastOutcome, TapLandCommand,
};

fn resolve_current_stack(
    service: &demonictutor::GameService<
        demonictutor::InMemoryEventStore,
        demonictutor::InMemoryEventBus,
    >,
    game: &mut Game,
) -> demonictutor::PassPriorityOutcome {
    let first_holder = game.priority().unwrap().current_holder().clone();
    service
        .pass_priority(game, demonictutor::PassPriorityCommand::new(first_holder))
        .unwrap();

    let second_holder = game.priority().unwrap().current_holder().clone();
    service
        .pass_priority(game, demonictutor::PassPriorityCommand::new(second_holder))
        .unwrap()
}

#[test]
fn cast_instant_puts_the_spell_on_the_stack_before_resolution() {
    let (service, mut game) = setup_two_player_game(
        "game-1",
        filled_library(vec![instant_card("giant-growth", 0)], 10),
        filled_library(vec![land_card("mountain")], 10),
    );

    advance_to_first_main_satisfying_cleanup(&service, &mut game);

    let card_id = CardInstanceId::new("game-1-player-1-0");
    let outcome = service
        .cast_spell(
            &mut game,
            CastSpellCommand::new(PlayerId::new("player-1"), card_id.clone()),
        )
        .unwrap();

    assert_eq!(outcome.spell_put_on_stack.card_id, card_id);
    assert!(matches!(
        outcome.spell_put_on_stack.card_type,
        CardType::Instant
    ));
    assert_eq!(outcome.spell_put_on_stack.mana_cost_paid, 0);
    assert_eq!(game.players()[0].hand().cards().len(), 7);
    assert_eq!(game.players()[0].graveyard().cards().len(), 0);
    assert_eq!(game.stack().len(), 1);
    assert_eq!(
        game.priority().unwrap().current_holder(),
        &PlayerId::new("player-1")
    );
}

#[test]
fn passing_priority_twice_resolves_an_instant_from_stack_to_graveyard() {
    let (service, mut game) = setup_two_player_game(
        "game-1",
        filled_library(vec![instant_card("giant-growth", 0)], 10),
        filled_library(vec![land_card("mountain")], 10),
    );

    advance_to_first_main_satisfying_cleanup(&service, &mut game);

    let card_id = CardInstanceId::new("game-1-player-1-0");
    service
        .cast_spell(
            &mut game,
            CastSpellCommand::new(PlayerId::new("player-1"), card_id.clone()),
        )
        .unwrap();

    let outcome = resolve_current_stack(&service, &mut game);

    let spell_cast = outcome.spell_cast.unwrap();
    assert_eq!(spell_cast.card_id, card_id);
    assert!(matches!(spell_cast.card_type, CardType::Instant));
    assert!(matches!(
        spell_cast.outcome,
        SpellCastOutcome::ResolvedToGraveyard
    ));
    assert!(outcome.creatures_died.is_empty());
    assert_eq!(game.stack().len(), 0);
    assert_eq!(game.players()[0].graveyard().cards().len(), 1);
    assert_eq!(
        game.priority().unwrap().current_holder(),
        &PlayerId::new("player-1")
    );
}

#[test]
fn casting_player_keeps_priority_after_casting_a_spell() {
    let (service, mut game) = setup_two_player_game(
        "game-priority-after-cast",
        filled_library(vec![instant_card("giant-growth", 0)], 10),
        filled_library(vec![land_card("mountain")], 10),
    );

    advance_to_first_main_satisfying_cleanup(&service, &mut game);

    service
        .cast_spell(
            &mut game,
            CastSpellCommand::new(
                PlayerId::new("player-1"),
                CardInstanceId::new("game-priority-after-cast-player-1-0"),
            ),
        )
        .unwrap();

    assert_eq!(
        game.priority().unwrap().current_holder(),
        &PlayerId::new("player-1")
    );
}

#[test]
fn opponent_can_cast_an_instant_response_after_the_caster_passes_priority() {
    let (service, mut game) = setup_two_player_game(
        "game-respond-instant",
        filled_library(
            vec![vanilla_creature("grizzly-bears"), land_card("forest")],
            10,
        ),
        filled_library(vec![instant_card("giant-growth", 0)], 10),
    );

    advance_to_first_main_satisfying_cleanup(&service, &mut game);

    let alice_land = CardInstanceId::new("game-respond-instant-player-1-1");
    service
        .play_land(
            &mut game,
            PlayLandCommand::new(PlayerId::new("player-1"), alice_land.clone()),
        )
        .unwrap();
    service
        .tap_land(
            &mut game,
            TapLandCommand::new(PlayerId::new("player-1"), alice_land),
        )
        .unwrap();

    let alice_spell = CardInstanceId::new("game-respond-instant-player-1-0");
    service
        .cast_spell(
            &mut game,
            CastSpellCommand::new(PlayerId::new("player-1"), alice_spell.clone()),
        )
        .unwrap();

    service
        .pass_priority(
            &mut game,
            demonictutor::PassPriorityCommand::new(PlayerId::new("player-1")),
        )
        .unwrap();

    let bob_spell = CardInstanceId::new("game-respond-instant-player-2-0");
    let response = service
        .cast_spell(
            &mut game,
            CastSpellCommand::new(PlayerId::new("player-2"), bob_spell.clone()),
        )
        .unwrap();

    assert_eq!(response.spell_put_on_stack.card_id, bob_spell);
    assert_eq!(game.stack().len(), 2);
    assert_eq!(
        game.stack().top().unwrap().controller_id(),
        &PlayerId::new("player-2")
    );
    assert_eq!(
        game.priority().unwrap().current_holder(),
        &PlayerId::new("player-2")
    );

    let response_resolution = resolve_current_stack(&service, &mut game);
    let response_spell_cast = response_resolution.spell_cast.unwrap();
    assert_eq!(
        response_spell_cast.card_id,
        CardInstanceId::new("game-respond-instant-player-2-0")
    );
    assert_eq!(game.stack().len(), 1);
    assert_eq!(game.stack().top().unwrap().source_card_id(), &alice_spell);
    assert_eq!(
        game.priority().unwrap().current_holder(),
        &PlayerId::new("player-1")
    );

    let original_resolution = resolve_current_stack(&service, &mut game);
    let original_spell_cast = original_resolution.spell_cast.unwrap();
    assert_eq!(original_spell_cast.card_id, alice_spell);
    assert_eq!(game.stack().len(), 0);
}

#[test]
fn opponent_cannot_cast_a_creature_as_a_response_after_the_caster_passes() {
    let (service, mut game) = setup_two_player_game(
        "game-respond-creature",
        filled_library(vec![instant_card("giant-growth", 0)], 10),
        filled_library(vec![vanilla_creature("grizzly-bears")], 10),
    );

    advance_to_first_main_satisfying_cleanup(&service, &mut game);

    service
        .cast_spell(
            &mut game,
            CastSpellCommand::new(
                PlayerId::new("player-1"),
                CardInstanceId::new("game-respond-creature-player-1-0"),
            ),
        )
        .unwrap();

    service
        .pass_priority(
            &mut game,
            demonictutor::PassPriorityCommand::new(PlayerId::new("player-1")),
        )
        .unwrap();

    let result = service.cast_spell(
        &mut game,
        CastSpellCommand::new(
            PlayerId::new("player-2"),
            CardInstanceId::new("game-respond-creature-player-2-0"),
        ),
    );

    assert!(matches!(
        result,
        Err(DomainError::Game(GameError::OnlyInstantSpellsSupportedAsResponses(card_id)))
            if card_id == CardInstanceId::new("game-respond-creature-player-2-0")
    ));
    assert_eq!(game.stack().len(), 1);
}

#[test]
fn cast_spell_rejected_land_card_stays_in_hand() {
    let (service, mut game) = setup_two_player_game(
        "game-1",
        filled_library(vec![land_card("forest")], 10),
        filled_library(vec![land_card("mountain")], 10),
    );

    advance_to_first_main_satisfying_cleanup(&service, &mut game);

    let hand_before = game.players()[0].hand().cards().len();
    let result = service.cast_spell(
        &mut game,
        CastSpellCommand::new(
            PlayerId::new("player-1"),
            CardInstanceId::new("game-1-player-1-0"),
        ),
    );

    assert!(result.is_err());
    assert_eq!(game.players()[0].hand().cards().len(), hand_before);
}

#[test]
fn cast_spell_fails_for_land_card() {
    let (service, mut game) = setup_two_player_game(
        "game-1",
        filled_library(vec![land_card("forest")], 10),
        filled_library(vec![land_card("mountain")], 10),
    );

    advance_to_first_main_satisfying_cleanup(&service, &mut game);

    let result = service.cast_spell(
        &mut game,
        CastSpellCommand::new(
            PlayerId::new("player-1"),
            CardInstanceId::new("game-1-player-1-0"),
        ),
    );

    assert!(matches!(
        result,
        Err(DomainError::Card(CardError::CannotCastLand(_)))
    ));
}

#[test]
fn resolving_a_creature_spell_moves_card_to_battlefield() {
    let (service, mut game) = setup_two_player_game(
        "game-1",
        filled_library(vec![vanilla_creature("grizzly-bears")], 10),
        filled_library(vec![land_card("mountain")], 10),
    );

    advance_to_first_main_satisfying_cleanup(&service, &mut game);

    let cast_outcome = service
        .cast_spell(
            &mut game,
            CastSpellCommand::new(
                PlayerId::new("player-1"),
                CardInstanceId::new("game-1-player-1-0"),
            ),
        )
        .unwrap();

    assert!(matches!(
        cast_outcome.spell_put_on_stack.card_type,
        CardType::Creature
    ));
    assert_eq!(game.stack().len(), 1);

    let outcome = resolve_current_stack(&service, &mut game);
    let spell_cast = outcome.spell_cast.unwrap();

    assert!(matches!(
        spell_cast.outcome,
        SpellCastOutcome::EnteredBattlefield
    ));
    assert!(outcome.creatures_died.is_empty());
    assert_eq!(game.stack().len(), 0);
    assert_eq!(game.players()[0].battlefield().cards().len(), 1);
    assert_eq!(game.players()[0].graveyard().cards().len(), 0);
}

#[test]
fn resolving_an_artifact_spell_moves_card_to_battlefield() {
    let (service, mut game) = setup_two_player_game(
        "game-1",
        filled_library(vec![artifact_card("howling-mine", 0)], 10),
        filled_library(vec![land_card("mountain")], 10),
    );

    advance_to_first_main_satisfying_cleanup(&service, &mut game);

    let cast_outcome = service
        .cast_spell(
            &mut game,
            CastSpellCommand::new(
                PlayerId::new("player-1"),
                CardInstanceId::new("game-1-player-1-0"),
            ),
        )
        .unwrap();

    assert!(matches!(
        cast_outcome.spell_put_on_stack.card_type,
        CardType::Artifact
    ));
    let outcome = resolve_current_stack(&service, &mut game);

    let spell_cast = outcome.spell_cast.unwrap();
    assert!(matches!(
        spell_cast.outcome,
        SpellCastOutcome::EnteredBattlefield
    ));
    assert!(outcome.creatures_died.is_empty());
    assert_eq!(game.players()[0].battlefield().cards().len(), 1);
    assert_eq!(game.players()[0].graveyard().cards().len(), 0);
}

#[test]
fn zero_toughness_creature_dies_after_its_spell_resolves() {
    let (service, mut game) = setup_two_player_game(
        "game-1",
        filled_library(
            vec![LibraryCard::creature(
                CardDefinitionId::new("zero-toughness-creature"),
                0,
                1,
                0,
            )],
            10,
        ),
        filled_library(vec![land_card("mountain")], 10),
    );

    advance_to_first_main_satisfying_cleanup(&service, &mut game);

    service
        .cast_spell(
            &mut game,
            CastSpellCommand::new(
                PlayerId::new("player-1"),
                CardInstanceId::new("game-1-player-1-0"),
            ),
        )
        .unwrap();

    let outcome = resolve_current_stack(&service, &mut game);

    assert_eq!(outcome.creatures_died.len(), 1);
    assert_eq!(
        outcome.creatures_died[0].player_id,
        PlayerId::new("player-1")
    );
    assert_eq!(
        outcome.creatures_died[0].card_id,
        CardInstanceId::new("game-1-player-1-0")
    );
    assert_eq!(game.players()[0].battlefield().cards().len(), 0);
    assert_eq!(game.players()[0].graveyard().cards().len(), 1);
}

#[test]
fn resolving_a_spell_reviews_pending_state_based_actions_for_existing_zero_toughness_creatures() {
    let service = crate::support::create_service();

    let mut alice = Player::new(PlayerId::new("player-1"));
    alice.battlefield_mut().add(CardInstance::new_creature(
        CardInstanceId::new("doomed-creature"),
        CardDefinitionId::new("doomed-creature"),
        0,
        1,
        0,
    ));
    alice.hand_mut().receive(vec![CardInstance::new(
        CardInstanceId::new("supporting-spell"),
        CardDefinitionId::new("supporting-spell"),
        CardType::Instant,
        0,
    )]);

    let bob = Player::new(PlayerId::new("player-2"));
    let mut game = Game::new(
        GameId::new("game-sba-review"),
        PlayerId::new("player-1"),
        Phase::FirstMain,
        1,
        vec![alice, bob],
        TerminalState::active(),
    );

    service
        .cast_spell(
            &mut game,
            CastSpellCommand::new(
                PlayerId::new("player-1"),
                CardInstanceId::new("supporting-spell"),
            ),
        )
        .unwrap();

    let outcome = resolve_current_stack(&service, &mut game);

    assert_eq!(outcome.creatures_died.len(), 1);
    assert_eq!(
        outcome.creatures_died[0].card_id,
        CardInstanceId::new("doomed-creature")
    );
    assert!(outcome.game_ended.is_none());
    assert_eq!(game.players()[0].battlefield().cards().len(), 0);
    assert_eq!(game.players()[0].graveyard().cards().len(), 2);
}

#[test]
fn cast_spell_fails_when_not_player_turn() {
    let (service, mut game) = setup_two_player_game(
        "game-1",
        filled_library(vec![instant_card("card-1", 0)], 7),
        filled_library(vec![instant_card("card-1", 0)], 7),
    );

    let result = service.cast_spell(
        &mut game,
        CastSpellCommand::new(
            PlayerId::new("player-2"),
            CardInstanceId::new("game-1-player-2-0"),
        ),
    );

    assert!(matches!(
        result,
        Err(DomainError::Game(
            demonictutor::GameError::NotYourTurn { .. }
        ))
    ));
}

#[test]
fn cast_spell_fails_when_card_not_in_hand() {
    let (service, mut game) = setup_two_player_game(
        "game-1",
        filled_library(vec![vanilla_creature("card-1")], 10),
        filled_library(vec![land_card("mountain")], 10),
    );

    advance_to_first_main_satisfying_cleanup(&service, &mut game);

    let result = service.cast_spell(
        &mut game,
        CastSpellCommand::new(
            PlayerId::new("player-1"),
            CardInstanceId::new("game-1-player-1-99"),
        ),
    );

    assert!(matches!(
        result,
        Err(DomainError::Card(CardError::NotInHand { .. }))
    ));
}

#[test]
fn cast_spell_fails_with_insufficient_mana() {
    let (service, mut game) = setup_two_player_game(
        "game-1",
        filled_library(vec![instant_card("expensive-spell", 3)], 10),
        filled_library(vec![land_card("mountain")], 10),
    );

    advance_to_first_main_satisfying_cleanup(&service, &mut game);

    let result = service.cast_spell(
        &mut game,
        CastSpellCommand::new(
            PlayerId::new("player-1"),
            CardInstanceId::new("game-1-player-1-0"),
        ),
    );

    assert!(matches!(
        result,
        Err(DomainError::Game(
            demonictutor::GameError::InsufficientMana { .. }
        ))
    ));
    assert_eq!(game.players()[0].hand().cards().len(), 8);
    assert_eq!(game.players()[0].graveyard().cards().len(), 0);
    assert_eq!(game.players()[0].battlefield().cards().len(), 0);
    assert_eq!(game.stack().len(), 0);
}

#[test]
fn cast_spell_succeeds_with_sufficient_mana() {
    let (service, mut game) = setup_two_player_game(
        "game-1",
        filled_library(vec![land_card("forest"), instant_card("card-2", 1)], 10),
        filled_library(vec![land_card("forest"), instant_card("card-2", 1)], 10),
    );

    advance_to_player_first_main_satisfying_cleanup(&service, &mut game, "player-2");
    assert_eq!(*game.phase(), Phase::FirstMain);

    service
        .play_land(
            &mut game,
            PlayLandCommand::new(
                PlayerId::new("player-2"),
                CardInstanceId::new("game-1-player-2-0"),
            ),
        )
        .unwrap();
    service
        .tap_land(
            &mut game,
            TapLandCommand::new(
                PlayerId::new("player-2"),
                CardInstanceId::new("game-1-player-2-0"),
            ),
        )
        .unwrap();

    assert_eq!(game.players()[1].mana(), 1);

    let result = service.cast_spell(
        &mut game,
        CastSpellCommand::new(
            PlayerId::new("player-2"),
            CardInstanceId::new("game-1-player-2-1"),
        ),
    );

    let outcome = result.unwrap();
    assert_eq!(outcome.spell_put_on_stack.mana_cost_paid, 1);
    assert_eq!(game.players()[1].mana(), 0);
    assert_eq!(game.stack().len(), 1);
}
