#![allow(clippy::unwrap_used)]

use demonictutor::{
    domain::play::game::{Player, TerminalState},
    AdjustPlayerLifeEffectCommand, CardDefinitionId, CardInstance, CardInstanceId, DeckId,
    DomainError, Game, GameEndReason, GameError, GameId, GameService, InMemoryEventBus,
    InMemoryEventStore, PlayerDeck, PlayerId, StartGameCommand,
};

fn player_deck(player: &str, deck: &str) -> PlayerDeck {
    PlayerDeck::new(PlayerId::new(player), DeckId::new(deck))
}

fn create_service() -> GameService<InMemoryEventStore, InMemoryEventBus> {
    GameService::new(InMemoryEventStore::new(), InMemoryEventBus::new())
}

#[test]
fn players_start_with_20_life() {
    let service = create_service();
    let (game, _) = service
        .start_game(StartGameCommand::new(
            GameId::new("game-1"),
            vec![
                player_deck("player-1", "deck-1"),
                player_deck("player-2", "deck-2"),
            ],
        ))
        .unwrap();

    assert_eq!(game.players()[0].life(), 20);
    assert_eq!(game.players()[1].life(), 20);
}

#[test]
fn adjust_life_deltas_player_life() {
    let service = create_service();
    let (mut game, _) = service
        .start_game(StartGameCommand::new(
            GameId::new("game-1"),
            vec![
                player_deck("player-1", "deck-1"),
                player_deck("player-2", "deck-2"),
            ],
        ))
        .unwrap();

    let cmd = AdjustPlayerLifeEffectCommand::new(
        PlayerId::new("player-1"),
        PlayerId::new("player-1"),
        -5,
    );
    let result = service.adjust_player_life_effect(&mut game, cmd);

    assert!(result.is_ok());
    let outcome = result.unwrap();
    let event = outcome.life_changed;
    assert_eq!(event.player_id.as_str(), "player-1");
    assert_eq!(event.from_life, 20);
    assert_eq!(event.to_life, 15);
    assert!(outcome.game_ended.is_none());
    assert_eq!(game.players()[0].life(), 15);
    assert_eq!(game.players()[1].life(), 20);
}

#[test]
fn adjust_life_gains_life() {
    let service = create_service();
    let (mut game, _) = service
        .start_game(StartGameCommand::new(
            GameId::new("game-1"),
            vec![
                player_deck("player-1", "deck-1"),
                player_deck("player-2", "deck-2"),
            ],
        ))
        .unwrap();

    let cmd =
        AdjustPlayerLifeEffectCommand::new(PlayerId::new("player-1"), PlayerId::new("player-1"), 3);
    let result = service.adjust_player_life_effect(&mut game, cmd);

    assert!(result.is_ok());
    let outcome = result.unwrap();
    let event = outcome.life_changed;
    assert_eq!(event.from_life, 20);
    assert_eq!(event.to_life, 23);
    assert!(outcome.game_ended.is_none());
    assert_eq!(game.players()[0].life(), 23);
}

#[test]
fn adjust_life_cannot_go_below_zero_and_ends_the_game() {
    let service = create_service();
    let (mut game, _) = service
        .start_game(StartGameCommand::new(
            GameId::new("game-1"),
            vec![
                player_deck("player-1", "deck-1"),
                player_deck("player-2", "deck-2"),
            ],
        ))
        .unwrap();

    let cmd = AdjustPlayerLifeEffectCommand::new(
        PlayerId::new("player-1"),
        PlayerId::new("player-1"),
        -30,
    );
    let result = service.adjust_player_life_effect(&mut game, cmd);

    assert!(result.is_ok());
    let outcome = result.unwrap();
    let event = outcome.life_changed;
    assert_eq!(event.from_life, 20);
    assert_eq!(event.to_life, 0);
    assert!(outcome.game_ended.is_some());
    let Some(game_ended) = outcome.game_ended else {
        return;
    };
    assert_eq!(game_ended.loser_id, PlayerId::new("player-1"));
    assert_eq!(game_ended.winner_id, PlayerId::new("player-2"));
    assert_eq!(game_ended.reason, GameEndReason::ZeroLife);
    assert_eq!(game.players()[0].life(), 0);
    assert!(game.is_over());
}

#[test]
fn gameplay_actions_fail_after_zero_life_game_end() {
    let service = create_service();
    let (mut game, _) = service
        .start_game(StartGameCommand::new(
            GameId::new("game-1"),
            vec![
                player_deck("player-1", "deck-1"),
                player_deck("player-2", "deck-2"),
            ],
        ))
        .unwrap();

    let outcome = service
        .adjust_player_life_effect(
            &mut game,
            AdjustPlayerLifeEffectCommand::new(
                PlayerId::new("player-1"),
                PlayerId::new("player-1"),
                -20,
            ),
        )
        .unwrap();
    assert!(outcome.game_ended.is_some());

    let result = service.adjust_player_life_effect(
        &mut game,
        AdjustPlayerLifeEffectCommand::new(PlayerId::new("player-2"), PlayerId::new("player-2"), 1),
    );

    assert_eq!(
        result.unwrap_err(),
        DomainError::Game(GameError::GameAlreadyEnded)
    );
}

#[test]
fn adjust_life_fails_for_unknown_player() {
    let service = create_service();
    let (mut game, _) = service
        .start_game(StartGameCommand::new(
            GameId::new("game-1"),
            vec![
                player_deck("player-1", "deck-1"),
                player_deck("player-2", "deck-2"),
            ],
        ))
        .unwrap();

    let cmd = AdjustPlayerLifeEffectCommand::new(
        PlayerId::new("unknown-player"),
        PlayerId::new("player-1"),
        10,
    );
    let result = service.adjust_player_life_effect(&mut game, cmd);

    assert!(result.is_err());
    assert!(matches!(
        result.unwrap_err(),
        DomainError::Game(GameError::PlayerNotFound { .. })
    ));
}

#[test]
fn adjust_life_fails_for_unknown_target_player() {
    let service = create_service();
    let (mut game, _) = service
        .start_game(StartGameCommand::new(
            GameId::new("game-1"),
            vec![
                player_deck("player-1", "deck-1"),
                player_deck("player-2", "deck-2"),
            ],
        ))
        .unwrap();

    let cmd = AdjustPlayerLifeEffectCommand::new(
        PlayerId::new("player-1"),
        PlayerId::new("unknown-player"),
        10,
    );
    let result = service.adjust_player_life_effect(&mut game, cmd);

    assert!(matches!(
        result.unwrap_err(),
        DomainError::Game(GameError::PlayerNotFound { .. })
    ));
}

#[test]
fn adjust_life_reviews_pending_state_based_actions_for_existing_lethal_damage() {
    let service = create_service();

    let mut alice = Player::new(PlayerId::new("player-1"), DeckId::new("deck-1"));
    let mut doomed_creature = CardInstance::new_creature(
        CardInstanceId::new("doomed-creature"),
        CardDefinitionId::new("doomed-creature"),
        0,
        2,
        2,
    );
    doomed_creature.add_damage(2);
    alice.battlefield_mut().add(doomed_creature);

    let bob = Player::new(PlayerId::new("player-2"), DeckId::new("deck-2"));
    let mut game = Game::new(
        GameId::new("game-sba-life"),
        PlayerId::new("player-1"),
        demonictutor::Phase::FirstMain,
        1,
        vec![alice, bob],
        TerminalState::active(),
    );

    let outcome = service
        .adjust_player_life_effect(
            &mut game,
            AdjustPlayerLifeEffectCommand::new(
                PlayerId::new("player-1"),
                PlayerId::new("player-1"),
                1,
            ),
        )
        .unwrap();

    assert_eq!(outcome.life_changed.from_life, 20);
    assert_eq!(outcome.life_changed.to_life, 21);
    assert_eq!(outcome.creatures_died.len(), 1);
    assert_eq!(
        outcome.creatures_died[0].card_id,
        CardInstanceId::new("doomed-creature")
    );
    assert!(outcome.game_ended.is_none());
    assert_eq!(game.players()[0].battlefield().cards().len(), 0);
    assert_eq!(game.players()[0].graveyard().cards().len(), 1);
}

#[test]
fn adjust_player_life_effect_can_target_another_player() {
    let service = create_service();
    let (mut game, _) = service
        .start_game(StartGameCommand::new(
            GameId::new("game-1"),
            vec![
                player_deck("player-1", "deck-1"),
                player_deck("player-2", "deck-2"),
            ],
        ))
        .unwrap();

    let outcome = service
        .adjust_player_life_effect(
            &mut game,
            AdjustPlayerLifeEffectCommand::new(
                PlayerId::new("player-1"),
                PlayerId::new("player-2"),
                -4,
            ),
        )
        .unwrap();

    assert_eq!(outcome.life_changed.player_id, PlayerId::new("player-2"));
    assert_eq!(outcome.life_changed.from_life, 20);
    assert_eq!(outcome.life_changed.to_life, 16);
    assert_eq!(game.players()[1].life(), 16);
}

#[test]
fn adjust_player_life_effect_can_gain_life_for_another_player() {
    let service = create_service();
    let (mut game, _) = service
        .start_game(StartGameCommand::new(
            GameId::new("game-1"),
            vec![
                player_deck("player-1", "deck-1"),
                player_deck("player-2", "deck-2"),
            ],
        ))
        .unwrap();

    let outcome = service
        .adjust_player_life_effect(
            &mut game,
            AdjustPlayerLifeEffectCommand::new(
                PlayerId::new("player-1"),
                PlayerId::new("player-2"),
                3,
            ),
        )
        .unwrap();

    assert_eq!(outcome.life_changed.player_id, PlayerId::new("player-2"));
    assert_eq!(outcome.life_changed.from_life, 20);
    assert_eq!(outcome.life_changed.to_life, 23);
    assert_eq!(game.players()[1].life(), 23);
}
