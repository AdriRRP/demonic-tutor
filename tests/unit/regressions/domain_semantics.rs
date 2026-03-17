#![allow(clippy::unwrap_used)]

use demonictutor::{
    AdvanceTurnCommand, CardDefinitionId, CardError, CardInstanceId, CastSpellCommand,
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

        service
            .advance_turn(game, AdvanceTurnCommand::new())
            .unwrap();
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

    service
        .cast_spell(
            &mut game,
            CastSpellCommand::new(
                PlayerId::new("player-1"),
                CardInstanceId::new("game-1-player-1-0"),
            ),
        )
        .unwrap();

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
    service
        .cast_spell(
            &mut game,
            CastSpellCommand::new(PlayerId::new("player-1"), creature_id),
        )
        .unwrap();

    assert!(game.players()[0].battlefield().cards()[0].has_summoning_sickness());

    advance_until(&service, &mut game, "player-2", Phase::Untap);

    assert!(game.players()[0].battlefield().cards()[0].has_summoning_sickness());

    advance_until(&service, &mut game, "player-1", Phase::Untap);

    assert!(!game.players()[0].battlefield().cards()[0].has_summoning_sickness());
}

#[test]
fn combat_damage_marks_damage_on_the_creatures_that_received_it() {
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
    service
        .cast_spell(
            &mut game,
            CastSpellCommand::new(PlayerId::new("player-1"), attacker_id.clone()),
        )
        .unwrap();

    advance_until(&service, &mut game, "player-2", Phase::FirstMain);
    service
        .cast_spell(
            &mut game,
            CastSpellCommand::new(PlayerId::new("player-2"), blocker_id.clone()),
        )
        .unwrap();

    advance_until(&service, &mut game, "player-1", Phase::Combat);

    service
        .declare_attackers(
            &mut game,
            DeclareAttackersCommand::new(PlayerId::new("player-1"), vec![attacker_id.clone()]),
        )
        .unwrap();

    let assignments = vec![(blocker_id, attacker_id)];

    service
        .declare_blockers(
            &mut game,
            DeclareBlockersCommand::new(PlayerId::new("player-2"), assignments.clone()),
        )
        .unwrap();

    service
        .resolve_combat_damage(
            &mut game,
            ResolveCombatDamageCommand::new(PlayerId::new("player-1"), assignments),
        )
        .unwrap();

    assert_eq!(game.players()[0].battlefield().cards()[0].damage(), 2);
    assert_eq!(game.players()[1].battlefield().cards()[0].damage(), 3);
    assert!(!game.players()[0].battlefield().cards()[0].is_attacking());
    assert!(!game.players()[1].battlefield().cards()[0].is_blocking());
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
