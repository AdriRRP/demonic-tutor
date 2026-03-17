#![allow(clippy::unwrap_used)]

use demonictutor::{
    AdjustLifeCommand, CardDefinitionId, CardError, CardInstanceId, CastSpellCommand, CreatureDied,
    DealOpeningHandsCommand, DeckId, DeclareAttackersCommand, DeclareBlockersCommand, DomainError,
    Game, GameId, GameService, InMemoryEventBus, InMemoryEventStore, LibraryCard,
    NonCreatureCardType, Phase, PlayerDeck, PlayerId, PlayerLibrary, ResolveCombatDamageCommand,
    StartGameCommand,
};

fn player_deck(player: &str, deck: &str) -> PlayerDeck {
    PlayerDeck::new(PlayerId::new(player), DeckId::new(deck))
}

fn player_deck_contents(player: &str, cards: Vec<LibraryCard>) -> PlayerLibrary {
    PlayerLibrary::new(PlayerId::new(player), cards)
}

fn create_service() -> GameService<InMemoryEventStore, InMemoryEventBus> {
    GameService::new(InMemoryEventStore::new(), InMemoryEventBus::new())
}

fn cast_and_resolve(
    service: &GameService<InMemoryEventStore, InMemoryEventBus>,
    game: &mut Game,
    player_id: &str,
    card_id: CardInstanceId,
) {
    crate::support::cast_spell_and_resolve(service, game, player_id, card_id);
}

fn close_empty_priority_window(
    service: &GameService<InMemoryEventStore, InMemoryEventBus>,
    game: &mut Game,
) {
    crate::support::close_empty_priority_window(service, game);
}

fn advance_until(
    service: &GameService<InMemoryEventStore, InMemoryEventBus>,
    game: &mut Game,
    active_player: &str,
    phase: Phase,
) {
    let target_player = PlayerId::new(active_player);

    for _ in 0..20 {
        if game.active_player() == &target_player && game.phase() == &phase {
            return;
        }

        crate::support::advance_turn_satisfying_cleanup(service, game);
    }

    assert_eq!(game.active_player(), &target_player);
    assert_eq!(game.phase(), &phase);
}

fn setup_game(
    player_one_cards: Vec<LibraryCard>,
    player_two_cards: Vec<LibraryCard>,
) -> (GameService<InMemoryEventStore, InMemoryEventBus>, Game) {
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

    service
        .deal_opening_hands(
            &mut game,
            &DealOpeningHandsCommand::new(vec![
                player_deck_contents("player-1", player_one_cards),
                player_deck_contents("player-2", player_two_cards),
            ]),
        )
        .unwrap();

    (service, game)
}

#[test]
fn instant_spells_resolve_to_graveyard_not_battlefield() {
    let (service, mut game) = setup_game(
        vec![
            LibraryCard::non_creature(
                CardDefinitionId::new("shock"),
                NonCreatureCardType::Instant,
                0,
            ),
            LibraryCard::creature(CardDefinitionId::new("card-2"), 0, 2, 2),
            LibraryCard::creature(CardDefinitionId::new("card-3"), 0, 2, 2),
            LibraryCard::creature(CardDefinitionId::new("card-4"), 0, 2, 2),
            LibraryCard::creature(CardDefinitionId::new("card-5"), 0, 2, 2),
            LibraryCard::creature(CardDefinitionId::new("card-6"), 0, 2, 2),
            LibraryCard::creature(CardDefinitionId::new("card-7"), 0, 2, 2),
            LibraryCard::creature(CardDefinitionId::new("card-8"), 0, 2, 2),
            LibraryCard::creature(CardDefinitionId::new("card-9"), 0, 2, 2),
            LibraryCard::creature(CardDefinitionId::new("card-10"), 0, 2, 2),
        ],
        vec![
            LibraryCard::non_creature(
                CardDefinitionId::new("forest"),
                NonCreatureCardType::Land,
                0,
            ),
            LibraryCard::creature(CardDefinitionId::new("card-2"), 0, 2, 2),
            LibraryCard::creature(CardDefinitionId::new("card-3"), 0, 2, 2),
            LibraryCard::creature(CardDefinitionId::new("card-4"), 0, 2, 2),
            LibraryCard::creature(CardDefinitionId::new("card-5"), 0, 2, 2),
            LibraryCard::creature(CardDefinitionId::new("card-6"), 0, 2, 2),
            LibraryCard::creature(CardDefinitionId::new("card-7"), 0, 2, 2),
            LibraryCard::creature(CardDefinitionId::new("card-8"), 0, 2, 2),
            LibraryCard::creature(CardDefinitionId::new("card-9"), 0, 2, 2),
            LibraryCard::creature(CardDefinitionId::new("card-10"), 0, 2, 2),
        ],
    );

    advance_until(&service, &mut game, "player-1", Phase::FirstMain);

    cast_and_resolve(
        &service,
        &mut game,
        "player-1",
        CardInstanceId::new("game-1-player-1-0"),
    );

    let player = &game.players()[0];
    assert_eq!(player.hand().cards().len(), 7);
    assert!(player.battlefield().cards().is_empty());
    assert_eq!(player.graveyard().cards().len(), 1);
}

#[test]
fn untap_only_updates_the_active_players_board_state() {
    let (service, mut game) = setup_game(
        vec![
            LibraryCard::creature(CardDefinitionId::new("bear"), 0, 2, 2),
            LibraryCard::creature(CardDefinitionId::new("card-2"), 0, 2, 2),
            LibraryCard::creature(CardDefinitionId::new("card-3"), 0, 2, 2),
            LibraryCard::creature(CardDefinitionId::new("card-4"), 0, 2, 2),
            LibraryCard::creature(CardDefinitionId::new("card-5"), 0, 2, 2),
            LibraryCard::creature(CardDefinitionId::new("card-6"), 0, 2, 2),
            LibraryCard::creature(CardDefinitionId::new("card-7"), 0, 2, 2),
            LibraryCard::creature(CardDefinitionId::new("card-8"), 0, 2, 2),
            LibraryCard::creature(CardDefinitionId::new("card-9"), 0, 2, 2),
            LibraryCard::creature(CardDefinitionId::new("card-10"), 0, 2, 2),
        ],
        vec![
            LibraryCard::non_creature(
                CardDefinitionId::new("forest"),
                NonCreatureCardType::Land,
                0,
            ),
            LibraryCard::creature(CardDefinitionId::new("card-2"), 0, 2, 2),
            LibraryCard::creature(CardDefinitionId::new("card-3"), 0, 2, 2),
            LibraryCard::creature(CardDefinitionId::new("card-4"), 0, 2, 2),
            LibraryCard::creature(CardDefinitionId::new("card-5"), 0, 2, 2),
            LibraryCard::creature(CardDefinitionId::new("card-6"), 0, 2, 2),
            LibraryCard::creature(CardDefinitionId::new("card-7"), 0, 2, 2),
            LibraryCard::creature(CardDefinitionId::new("card-8"), 0, 2, 2),
            LibraryCard::creature(CardDefinitionId::new("card-9"), 0, 2, 2),
            LibraryCard::creature(CardDefinitionId::new("card-10"), 0, 2, 2),
        ],
    );

    advance_until(&service, &mut game, "player-1", Phase::FirstMain);

    let creature_id = CardInstanceId::new("game-1-player-1-0");
    cast_and_resolve(&service, &mut game, "player-1", creature_id);

    assert!(game.players()[0].battlefield().cards()[0].has_summoning_sickness());

    advance_until(&service, &mut game, "player-2", Phase::Untap);

    assert!(game.players()[0].battlefield().cards()[0].has_summoning_sickness());

    advance_until(&service, &mut game, "player-1", Phase::Untap);

    assert!(!game.players()[0].battlefield().cards()[0].has_summoning_sickness());
}

#[test]
fn combat_damage_marks_surviving_creatures_and_destroys_lethally_damaged_ones() {
    let (service, mut game) = setup_game(
        vec![
            LibraryCard::creature(CardDefinitionId::new("ogre"), 0, 3, 3),
            LibraryCard::creature(CardDefinitionId::new("card-2"), 0, 2, 2),
            LibraryCard::creature(CardDefinitionId::new("card-3"), 0, 2, 2),
            LibraryCard::creature(CardDefinitionId::new("card-4"), 0, 2, 2),
            LibraryCard::creature(CardDefinitionId::new("card-5"), 0, 2, 2),
            LibraryCard::creature(CardDefinitionId::new("card-6"), 0, 2, 2),
            LibraryCard::creature(CardDefinitionId::new("card-7"), 0, 2, 2),
            LibraryCard::creature(CardDefinitionId::new("card-8"), 0, 2, 2),
            LibraryCard::creature(CardDefinitionId::new("card-9"), 0, 2, 2),
            LibraryCard::creature(CardDefinitionId::new("card-10"), 0, 2, 2),
        ],
        vec![
            LibraryCard::creature(CardDefinitionId::new("soldier"), 0, 2, 2),
            LibraryCard::creature(CardDefinitionId::new("card-2"), 0, 2, 2),
            LibraryCard::creature(CardDefinitionId::new("card-3"), 0, 2, 2),
            LibraryCard::creature(CardDefinitionId::new("card-4"), 0, 2, 2),
            LibraryCard::creature(CardDefinitionId::new("card-5"), 0, 2, 2),
            LibraryCard::creature(CardDefinitionId::new("card-6"), 0, 2, 2),
            LibraryCard::creature(CardDefinitionId::new("card-7"), 0, 2, 2),
            LibraryCard::creature(CardDefinitionId::new("card-8"), 0, 2, 2),
            LibraryCard::creature(CardDefinitionId::new("card-9"), 0, 2, 2),
            LibraryCard::creature(CardDefinitionId::new("card-10"), 0, 2, 2),
        ],
    );

    let attacker_id = CardInstanceId::new("game-1-player-1-0");
    let blocker_id = CardInstanceId::new("game-1-player-2-0");

    advance_until(&service, &mut game, "player-1", Phase::FirstMain);
    cast_and_resolve(&service, &mut game, "player-1", attacker_id.clone());

    advance_until(&service, &mut game, "player-2", Phase::FirstMain);
    cast_and_resolve(&service, &mut game, "player-2", blocker_id.clone());

    advance_until(&service, &mut game, "player-1", Phase::BeginningOfCombat);
    close_empty_priority_window(&service, &mut game);
    crate::support::advance_turn_raw(&service, &mut game);

    service
        .declare_attackers(
            &mut game,
            DeclareAttackersCommand::new(PlayerId::new("player-1"), vec![attacker_id.clone()]),
        )
        .unwrap();
    close_empty_priority_window(&service, &mut game);

    let assignments = vec![(blocker_id, attacker_id)];

    service
        .declare_blockers(
            &mut game,
            DeclareBlockersCommand::new(PlayerId::new("player-2"), assignments),
        )
        .unwrap();
    close_empty_priority_window(&service, &mut game);

    let outcome = service
        .resolve_combat_damage(
            &mut game,
            ResolveCombatDamageCommand::new(PlayerId::new("player-1")),
        )
        .unwrap();

    assert_eq!(game.players()[0].battlefield().cards()[0].damage(), 2);
    assert!(game.players()[1].battlefield().cards().is_empty());
    assert_eq!(game.players()[1].graveyard().cards().len(), 1);
    assert_eq!(outcome.creatures_died.len(), 1);
    assert_eq!(
        outcome.creatures_died[0].card_id,
        CardInstanceId::new("game-1-player-2-0")
    );
    assert!(!game.players()[0].battlefield().cards()[0].is_attacking());
}

#[test]
fn creature_destruction_emits_one_event_per_destroyed_creature() {
    let (service, mut game) = setup_game(
        vec![
            LibraryCard::creature(CardDefinitionId::new("rhino-a"), 0, 4, 5),
            LibraryCard::creature(CardDefinitionId::new("rhino-b"), 0, 4, 5),
            LibraryCard::creature(CardDefinitionId::new("card-3"), 0, 2, 2),
            LibraryCard::creature(CardDefinitionId::new("card-4"), 0, 2, 2),
            LibraryCard::creature(CardDefinitionId::new("card-5"), 0, 2, 2),
            LibraryCard::creature(CardDefinitionId::new("card-6"), 0, 2, 2),
            LibraryCard::creature(CardDefinitionId::new("card-7"), 0, 2, 2),
            LibraryCard::creature(CardDefinitionId::new("card-8"), 0, 2, 2),
            LibraryCard::creature(CardDefinitionId::new("card-9"), 0, 2, 2),
            LibraryCard::creature(CardDefinitionId::new("card-10"), 0, 2, 2),
        ],
        vec![
            LibraryCard::creature(CardDefinitionId::new("guard-a"), 0, 2, 2),
            LibraryCard::creature(CardDefinitionId::new("guard-b"), 0, 2, 2),
            LibraryCard::creature(CardDefinitionId::new("card-3"), 0, 2, 2),
            LibraryCard::creature(CardDefinitionId::new("card-4"), 0, 2, 2),
            LibraryCard::creature(CardDefinitionId::new("card-5"), 0, 2, 2),
            LibraryCard::creature(CardDefinitionId::new("card-6"), 0, 2, 2),
            LibraryCard::creature(CardDefinitionId::new("card-7"), 0, 2, 2),
            LibraryCard::creature(CardDefinitionId::new("card-8"), 0, 2, 2),
            LibraryCard::creature(CardDefinitionId::new("card-9"), 0, 2, 2),
            LibraryCard::creature(CardDefinitionId::new("card-10"), 0, 2, 2),
        ],
    );

    let left_attacker_id = CardInstanceId::new("game-1-player-1-0");
    let right_attacker_id = CardInstanceId::new("game-1-player-1-1");
    let left_blocker_id = CardInstanceId::new("game-1-player-2-0");
    let right_blocker_id = CardInstanceId::new("game-1-player-2-1");

    advance_until(&service, &mut game, "player-1", Phase::FirstMain);
    cast_and_resolve(&service, &mut game, "player-1", left_attacker_id.clone());
    cast_and_resolve(&service, &mut game, "player-1", right_attacker_id.clone());

    advance_until(&service, &mut game, "player-2", Phase::FirstMain);
    cast_and_resolve(&service, &mut game, "player-2", left_blocker_id.clone());
    cast_and_resolve(&service, &mut game, "player-2", right_blocker_id.clone());

    advance_until(&service, &mut game, "player-1", Phase::BeginningOfCombat);
    close_empty_priority_window(&service, &mut game);
    crate::support::advance_turn_raw(&service, &mut game);

    service
        .declare_attackers(
            &mut game,
            DeclareAttackersCommand::new(
                PlayerId::new("player-1"),
                vec![left_attacker_id.clone(), right_attacker_id.clone()],
            ),
        )
        .unwrap();
    close_empty_priority_window(&service, &mut game);

    let blocker_assignments = vec![
        (left_blocker_id.clone(), left_attacker_id),
        (right_blocker_id.clone(), right_attacker_id),
    ];

    service
        .declare_blockers(
            &mut game,
            DeclareBlockersCommand::new(PlayerId::new("player-2"), blocker_assignments),
        )
        .unwrap();
    close_empty_priority_window(&service, &mut game);

    let outcome = service
        .resolve_combat_damage(
            &mut game,
            ResolveCombatDamageCommand::new(PlayerId::new("player-1")),
        )
        .unwrap();

    assert_eq!(outcome.creatures_died.len(), 2);
    assert!(outcome.creatures_died.iter().all(|event: &CreatureDied| {
        event.player_id == PlayerId::new("player-2")
            && (event.card_id == left_blocker_id || event.card_id == right_blocker_id)
    }));
    assert_eq!(game.players()[1].battlefield().cards().len(), 0);
    assert_eq!(game.players()[1].graveyard().cards().len(), 2);
}

#[test]
fn unblocked_combat_damage_ends_the_game_when_it_reduces_a_player_to_zero_life() {
    let (service, mut game) = setup_game(
        vec![
            LibraryCard::creature(CardDefinitionId::new("ogre"), 0, 3, 3),
            LibraryCard::creature(CardDefinitionId::new("card-2"), 0, 2, 2),
            LibraryCard::creature(CardDefinitionId::new("card-3"), 0, 2, 2),
            LibraryCard::creature(CardDefinitionId::new("card-4"), 0, 2, 2),
            LibraryCard::creature(CardDefinitionId::new("card-5"), 0, 2, 2),
            LibraryCard::creature(CardDefinitionId::new("card-6"), 0, 2, 2),
            LibraryCard::creature(CardDefinitionId::new("card-7"), 0, 2, 2),
            LibraryCard::creature(CardDefinitionId::new("card-8"), 0, 2, 2),
            LibraryCard::creature(CardDefinitionId::new("card-9"), 0, 2, 2),
            LibraryCard::creature(CardDefinitionId::new("card-10"), 0, 2, 2),
        ],
        vec![
            LibraryCard::creature(CardDefinitionId::new("card-1"), 0, 2, 2),
            LibraryCard::creature(CardDefinitionId::new("card-2"), 0, 2, 2),
            LibraryCard::creature(CardDefinitionId::new("card-3"), 0, 2, 2),
            LibraryCard::creature(CardDefinitionId::new("card-4"), 0, 2, 2),
            LibraryCard::creature(CardDefinitionId::new("card-5"), 0, 2, 2),
            LibraryCard::creature(CardDefinitionId::new("card-6"), 0, 2, 2),
            LibraryCard::creature(CardDefinitionId::new("card-7"), 0, 2, 2),
            LibraryCard::creature(CardDefinitionId::new("card-8"), 0, 2, 2),
            LibraryCard::creature(CardDefinitionId::new("card-9"), 0, 2, 2),
            LibraryCard::creature(CardDefinitionId::new("card-10"), 0, 2, 2),
        ],
    );

    let attacker_id = CardInstanceId::new("game-1-player-1-0");

    advance_until(&service, &mut game, "player-1", Phase::FirstMain);
    cast_and_resolve(&service, &mut game, "player-1", attacker_id.clone());

    service
        .adjust_life(
            &mut game,
            AdjustLifeCommand::new(PlayerId::new("player-2"), -17),
        )
        .unwrap();

    advance_until(&service, &mut game, "player-2", Phase::FirstMain);
    advance_until(&service, &mut game, "player-1", Phase::BeginningOfCombat);
    close_empty_priority_window(&service, &mut game);
    crate::support::advance_turn_raw(&service, &mut game);

    service
        .declare_attackers(
            &mut game,
            DeclareAttackersCommand::new(PlayerId::new("player-1"), vec![attacker_id]),
        )
        .unwrap();
    close_empty_priority_window(&service, &mut game);
    crate::support::advance_turn_raw(&service, &mut game);

    let outcome = service
        .resolve_combat_damage(
            &mut game,
            ResolveCombatDamageCommand::new(PlayerId::new("player-1")),
        )
        .unwrap();

    assert_eq!(
        outcome.life_changed.as_ref().map(|event| event.to_life),
        Some(0)
    );
    assert_eq!(
        outcome
            .game_ended
            .as_ref()
            .map(|event| event.loser_id.clone()),
        Some(PlayerId::new("player-2"))
    );
    assert_eq!(
        outcome
            .game_ended
            .as_ref()
            .map(|event| event.winner_id.clone()),
        Some(PlayerId::new("player-1"))
    );
    assert!(game.is_over());
}

#[test]
fn cast_land_keeps_the_card_in_hand() {
    let (service, mut game) = setup_game(
        vec![
            LibraryCard::non_creature(
                CardDefinitionId::new("forest"),
                NonCreatureCardType::Land,
                0,
            ),
            LibraryCard::creature(CardDefinitionId::new("card-2"), 0, 2, 2),
            LibraryCard::creature(CardDefinitionId::new("card-3"), 0, 2, 2),
            LibraryCard::creature(CardDefinitionId::new("card-4"), 0, 2, 2),
            LibraryCard::creature(CardDefinitionId::new("card-5"), 0, 2, 2),
            LibraryCard::creature(CardDefinitionId::new("card-6"), 0, 2, 2),
            LibraryCard::creature(CardDefinitionId::new("card-7"), 0, 2, 2),
            LibraryCard::creature(CardDefinitionId::new("card-8"), 0, 2, 2),
            LibraryCard::creature(CardDefinitionId::new("card-9"), 0, 2, 2),
            LibraryCard::creature(CardDefinitionId::new("card-10"), 0, 2, 2),
        ],
        vec![
            LibraryCard::non_creature(
                CardDefinitionId::new("mountain"),
                NonCreatureCardType::Land,
                0,
            ),
            LibraryCard::creature(CardDefinitionId::new("card-2"), 0, 2, 2),
            LibraryCard::creature(CardDefinitionId::new("card-3"), 0, 2, 2),
            LibraryCard::creature(CardDefinitionId::new("card-4"), 0, 2, 2),
            LibraryCard::creature(CardDefinitionId::new("card-5"), 0, 2, 2),
            LibraryCard::creature(CardDefinitionId::new("card-6"), 0, 2, 2),
            LibraryCard::creature(CardDefinitionId::new("card-7"), 0, 2, 2),
            LibraryCard::creature(CardDefinitionId::new("card-8"), 0, 2, 2),
            LibraryCard::creature(CardDefinitionId::new("card-9"), 0, 2, 2),
            LibraryCard::creature(CardDefinitionId::new("card-10"), 0, 2, 2),
        ],
    );

    advance_until(&service, &mut game, "player-1", Phase::FirstMain);

    let hand_before = game.players()[0].hand().cards().len();
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
    assert_eq!(game.players()[0].hand().cards().len(), hand_before);
}
