#![allow(clippy::unwrap_used)]

use crate::support::{
    advance_to_first_main_satisfying_cleanup, advance_to_player_first_main_satisfying_cleanup,
    artifact_card, filled_library, instant_card, land_card, setup_two_player_game,
    vanilla_creature,
};
use demonictutor::{
    domain::play::game::{Player, TerminalState},
    CardDefinitionId, CardError, CardInstance, CardInstanceId, CardType, CastSpellCommand, DeckId,
    DomainError, Game, GameId, LibraryCard, Phase, PlayLandCommand, PlayerId, SpellCastOutcome,
    TapLandCommand,
};

#[test]
fn cast_instant_moves_card_from_hand_to_graveyard() {
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

    assert_eq!(outcome.spell_cast.card_id, card_id);
    assert!(matches!(outcome.spell_cast.card_type, CardType::Instant));
    assert_eq!(outcome.spell_cast.mana_cost_paid, 0);
    assert!(matches!(
        outcome.spell_cast.outcome,
        SpellCastOutcome::ResolvedToGraveyard
    ));
    assert!(outcome.creatures_died.is_empty());
    assert_eq!(game.players()[0].hand().cards().len(), 7);
    assert_eq!(game.players()[0].battlefield().cards().len(), 0);
    assert_eq!(game.players()[0].graveyard().cards().len(), 1);
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
        Err(DomainError::Card(CardError::CannotCastLand { .. }))
    ));
}

#[test]
fn cast_creature_spell_moves_card_to_battlefield() {
    let (service, mut game) = setup_two_player_game(
        "game-1",
        filled_library(vec![vanilla_creature("grizzly-bears")], 10),
        filled_library(vec![land_card("mountain")], 10),
    );

    advance_to_first_main_satisfying_cleanup(&service, &mut game);

    let outcome = service
        .cast_spell(
            &mut game,
            CastSpellCommand::new(
                PlayerId::new("player-1"),
                CardInstanceId::new("game-1-player-1-0"),
            ),
        )
        .unwrap();

    assert!(matches!(outcome.spell_cast.card_type, CardType::Creature));
    assert_eq!(outcome.spell_cast.mana_cost_paid, 0);
    assert!(matches!(
        outcome.spell_cast.outcome,
        SpellCastOutcome::EnteredBattlefield
    ));
    assert!(outcome.creatures_died.is_empty());
    assert_eq!(game.players()[0].battlefield().cards().len(), 1);
    assert_eq!(game.players()[0].graveyard().cards().len(), 0);
}

#[test]
fn cast_artifact_spell_moves_card_to_battlefield() {
    let (service, mut game) = setup_two_player_game(
        "game-1",
        filled_library(vec![artifact_card("howling-mine", 0)], 10),
        filled_library(vec![land_card("mountain")], 10),
    );

    advance_to_first_main_satisfying_cleanup(&service, &mut game);

    let outcome = service
        .cast_spell(
            &mut game,
            CastSpellCommand::new(
                PlayerId::new("player-1"),
                CardInstanceId::new("game-1-player-1-0"),
            ),
        )
        .unwrap();

    assert!(matches!(outcome.spell_cast.card_type, CardType::Artifact));
    assert_eq!(outcome.spell_cast.mana_cost_paid, 0);
    assert!(matches!(
        outcome.spell_cast.outcome,
        SpellCastOutcome::EnteredBattlefield
    ));
    assert!(outcome.creatures_died.is_empty());
    assert_eq!(game.players()[0].battlefield().cards().len(), 1);
    assert_eq!(game.players()[0].graveyard().cards().len(), 0);
}

#[test]
fn cast_zero_toughness_creature_dies_immediately_after_entering_battlefield() {
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

    let outcome = service
        .cast_spell(
            &mut game,
            CastSpellCommand::new(
                PlayerId::new("player-1"),
                CardInstanceId::new("game-1-player-1-0"),
            ),
        )
        .unwrap();

    assert!(matches!(outcome.spell_cast.card_type, CardType::Creature));
    assert!(matches!(
        outcome.spell_cast.outcome,
        SpellCastOutcome::EnteredBattlefield
    ));
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
fn cast_spell_reviews_pending_state_based_actions_for_existing_zero_toughness_creatures() {
    let service = crate::support::create_service();

    let mut alice = Player::new(PlayerId::new("player-1"), DeckId::new("deck-1"));
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

    let bob = Player::new(PlayerId::new("player-2"), DeckId::new("deck-2"));
    let mut game = Game::new(
        GameId::new("game-sba-review"),
        PlayerId::new("player-1"),
        Phase::FirstMain,
        1,
        vec![alice, bob],
        TerminalState::active(),
    );

    let outcome = service
        .cast_spell(
            &mut game,
            CastSpellCommand::new(
                PlayerId::new("player-1"),
                CardInstanceId::new("supporting-spell"),
            ),
        )
        .unwrap();

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
    assert_eq!(outcome.spell_cast.mana_cost_paid, 1);
    assert!(outcome.creatures_died.is_empty());
    assert_eq!(game.players()[1].mana(), 0);
}
