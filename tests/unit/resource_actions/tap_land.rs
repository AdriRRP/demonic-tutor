#![allow(clippy::unwrap_used)]
#![allow(clippy::panic)]

use crate::support::{
    advance_to_first_main_satisfying_cleanup, advance_to_player_phase_satisfying_cleanup,
    advance_turn_raw, create_service, filled_library, forest_card, instant_card, land_card,
    setup_two_player_game,
};
use demonictutor::{
    CardError, CardInstanceId, CastSpellCommand, DomainError, GameService, InMemoryEventBus,
    InMemoryEventStore, PassPriorityCommand, Phase, PlayLandCommand, PlayerId, TapLandCommand,
};

fn create_game_with_land_on_battlefield() -> (
    demonictutor::Game,
    GameService<InMemoryEventStore, InMemoryEventBus>,
) {
    let service = create_service();
    let mut game = crate::support::start_two_player_game(&service, "game-1");

    crate::support::deal_opening_hands(
        &service,
        &mut game,
        filled_library(vec![land_card("forest")], 10),
        filled_library(vec![land_card("mountain")], 10),
    );

    advance_to_first_main_satisfying_cleanup(&service, &mut game);
    service
        .play_land(
            &mut game,
            PlayLandCommand::new(
                PlayerId::new("player-1"),
                CardInstanceId::new("game-1-player-1-0"),
            ),
        )
        .unwrap();

    (game, service)
}

fn hand_card_id_by_definition(
    game: &demonictutor::Game,
    player_index: usize,
    definition_id: &str,
) -> CardInstanceId {
    game.players()[player_index]
        .hand_card_by_definition(&demonictutor::CardDefinitionId::new(definition_id))
        .unwrap_or_else(|| panic!("hand card should exist for definition {definition_id}"))
        .id()
        .clone()
}

#[test]
fn players_start_with_zero_mana() {
    let service = create_service();
    let game = crate::support::start_two_player_game(&service, "game-1");

    assert_eq!(game.players()[0].mana(), 0);
    assert_eq!(game.players()[1].mana(), 0);
}

#[test]
fn tap_land_adds_mana() {
    let (mut game, service) = create_game_with_land_on_battlefield();

    let result = service.tap_land(
        &mut game,
        TapLandCommand::new(
            PlayerId::new("player-1"),
            CardInstanceId::new("game-1-player-1-0"),
        ),
    );

    assert!(result.is_ok());
    let (_, mana_event) = result.unwrap();
    assert_eq!(mana_event.amount, 1);
    assert_eq!(mana_event.new_mana_total, 1);
    assert_eq!(game.players()[0].mana(), 1);
}

#[test]
fn tap_land_fails_for_untapped_land() {
    let (mut game, service) = create_game_with_land_on_battlefield();

    assert!(service
        .tap_land(
            &mut game,
            TapLandCommand::new(
                PlayerId::new("player-1"),
                CardInstanceId::new("game-1-player-1-0"),
            ),
        )
        .is_ok());

    let result = service.tap_land(
        &mut game,
        TapLandCommand::new(
            PlayerId::new("player-1"),
            CardInstanceId::new("game-1-player-1-0"),
        ),
    );

    assert!(matches!(
        result,
        Err(DomainError::Card(CardError::AlreadyTapped { .. }))
    ));
}

#[test]
fn tap_land_fails_for_non_land_card() {
    let (mut game, service) = create_game_with_land_on_battlefield();

    let result = service.tap_land(
        &mut game,
        TapLandCommand::new(
            PlayerId::new("player-1"),
            CardInstanceId::new("game-1-player-1-1"),
        ),
    );

    assert!(matches!(
        result,
        Err(DomainError::Card(CardError::NotOnBattlefield { .. }))
    ));
}

#[test]
fn tap_land_fails_for_unknown_card() {
    let (service, mut game) = crate::support::setup_two_player_game(
        "game-1",
        filled_library(vec![land_card("forest")], 10),
        filled_library(vec![land_card("mountain")], 10),
    );
    advance_to_first_main_satisfying_cleanup(&service, &mut game);

    let result = service.tap_land(
        &mut game,
        TapLandCommand::new(
            PlayerId::new("player-1"),
            CardInstanceId::new("nonexistent"),
        ),
    );

    assert!(matches!(
        result,
        Err(DomainError::Card(CardError::NotOnBattlefield { .. }))
    ));
}

#[test]
fn tap_land_fails_when_not_players_turn() {
    let (mut game, service) = create_game_with_land_on_battlefield();

    let result = service.tap_land(
        &mut game,
        TapLandCommand::new(
            PlayerId::new("player-2"),
            CardInstanceId::new("game-1-player-1-0"),
        ),
    );

    assert!(matches!(
        result,
        Err(DomainError::Game(
            demonictutor::GameError::NotYourTurn { .. }
                | demonictutor::GameError::NotPriorityHolder { .. }
        ))
    ));
}

#[test]
fn tap_land_fails_while_the_stack_is_not_empty() {
    let (service, mut game) = setup_two_player_game(
        "game-tap-land-stack-open",
        filled_library(vec![land_card("forest"), instant_card("shock", 0)], 10),
        filled_library(vec![land_card("mountain")], 10),
    );
    advance_to_first_main_satisfying_cleanup(&service, &mut game);
    service
        .play_land(
            &mut game,
            PlayLandCommand::new(
                PlayerId::new("player-1"),
                CardInstanceId::new("game-tap-land-stack-open-player-1-0"),
            ),
        )
        .unwrap();
    service
        .cast_spell(
            &mut game,
            CastSpellCommand::new(
                PlayerId::new("player-1"),
                CardInstanceId::new("game-tap-land-stack-open-player-1-1"),
            ),
        )
        .unwrap();

    let result = service.tap_land(
        &mut game,
        TapLandCommand::new(
            PlayerId::new("player-1"),
            CardInstanceId::new("game-tap-land-stack-open-player-1-0"),
        ),
    );

    assert!(result.is_ok());
}

#[test]
fn tap_land_succeeds_in_upkeep_when_the_active_player_holds_priority() {
    let (mut game, service) = create_game_with_land_on_battlefield();

    advance_to_player_phase_satisfying_cleanup(&service, &mut game, "player-1", Phase::Upkeep);
    assert!(game.has_open_priority_window());
    assert_eq!(
        game.priority().unwrap().current_holder(),
        &PlayerId::new("player-1")
    );

    let result = service.tap_land(
        &mut game,
        TapLandCommand::new(
            PlayerId::new("player-1"),
            CardInstanceId::new("game-1-player-1-0"),
        ),
    );

    assert!(result.is_ok());
    let (_, mana_event) = result.unwrap();
    assert_eq!(mana_event.amount, 1);
    assert_eq!(mana_event.new_mana_total, 1);
    assert_eq!(game.players()[0].mana(), 1);
}

#[test]
fn tap_land_succeeds_in_beginning_of_combat_when_the_active_player_holds_priority() {
    let (mut game, service) = create_game_with_land_on_battlefield();

    advance_to_player_phase_satisfying_cleanup(
        &service,
        &mut game,
        "player-1",
        Phase::BeginningOfCombat,
    );
    assert!(game.has_open_priority_window());
    assert_eq!(
        game.priority().unwrap().current_holder(),
        &PlayerId::new("player-1")
    );

    let result = service.tap_land(
        &mut game,
        TapLandCommand::new(
            PlayerId::new("player-1"),
            CardInstanceId::new("game-1-player-1-0"),
        ),
    );

    assert!(result.is_ok());
    let (_, mana_event) = result.unwrap();
    assert_eq!(mana_event.amount, 1);
    assert_eq!(mana_event.new_mana_total, 1);
    assert_eq!(game.players()[0].mana(), 1);
}

#[test]
fn tap_land_fails_in_upkeep_after_the_active_player_passes_priority() {
    let (mut game, service) = create_game_with_land_on_battlefield();

    advance_to_player_phase_satisfying_cleanup(&service, &mut game, "player-1", Phase::Upkeep);
    service
        .pass_priority(
            &mut game,
            PassPriorityCommand::new(PlayerId::new("player-1")),
        )
        .unwrap();
    assert_eq!(
        game.priority().unwrap().current_holder(),
        &PlayerId::new("player-2")
    );

    let result = service.tap_land(
        &mut game,
        TapLandCommand::new(
            PlayerId::new("player-1"),
            CardInstanceId::new("game-1-player-1-0"),
        ),
    );

    assert!(matches!(
        result,
        Err(DomainError::Game(
            demonictutor::GameError::NotPriorityHolder { .. }
        ))
    ));
}

#[test]
fn non_active_player_can_tap_a_land_while_holding_priority_on_an_open_stack() {
    let alice_cards = vec![instant_card("alice-shock", 0); 10];
    let bob_cards = vec![land_card("mountain"); 10];

    let (service, mut game) =
        setup_two_player_game("game-tap-land-response-window", alice_cards, bob_cards);
    advance_to_player_phase_satisfying_cleanup(&service, &mut game, "player-2", Phase::FirstMain);

    let bob_land = hand_card_id_by_definition(&game, 1, "mountain");
    service
        .play_land(
            &mut game,
            PlayLandCommand::new(PlayerId::new("player-2"), bob_land.clone()),
        )
        .unwrap();

    advance_to_player_phase_satisfying_cleanup(&service, &mut game, "player-1", Phase::FirstMain);
    let alice_spell = hand_card_id_by_definition(&game, 0, "alice-shock");

    service
        .cast_spell(
            &mut game,
            CastSpellCommand::new(PlayerId::new("player-1"), alice_spell),
        )
        .unwrap();
    service
        .pass_priority(
            &mut game,
            PassPriorityCommand::new(PlayerId::new("player-1")),
        )
        .unwrap();

    assert_eq!(
        game.priority().unwrap().current_holder(),
        &PlayerId::new("player-2")
    );

    let result = service.tap_land(
        &mut game,
        TapLandCommand::new(PlayerId::new("player-2"), bob_land),
    );

    assert!(result.is_ok());
    let (_, mana_event) = result.unwrap();
    assert_eq!(mana_event.amount, 1);
    assert_eq!(mana_event.new_mana_total, 1);
    assert_eq!(game.players()[1].mana(), 1);
}

#[test]
fn tapping_a_land_for_mana_does_not_use_the_stack_or_change_priority() {
    let alice_cards = vec![instant_card("alice-shock", 0); 10];
    let bob_cards = vec![land_card("mountain"); 10];

    let (service, mut game) =
        setup_two_player_game("game-tap-land-no-stack", alice_cards, bob_cards);
    advance_to_player_phase_satisfying_cleanup(&service, &mut game, "player-2", Phase::FirstMain);

    let bob_land = hand_card_id_by_definition(&game, 1, "mountain");
    service
        .play_land(
            &mut game,
            PlayLandCommand::new(PlayerId::new("player-2"), bob_land.clone()),
        )
        .unwrap();

    advance_to_player_phase_satisfying_cleanup(&service, &mut game, "player-1", Phase::FirstMain);
    let alice_spell = hand_card_id_by_definition(&game, 0, "alice-shock");

    service
        .cast_spell(
            &mut game,
            CastSpellCommand::new(PlayerId::new("player-1"), alice_spell.clone()),
        )
        .unwrap();
    service
        .pass_priority(
            &mut game,
            PassPriorityCommand::new(PlayerId::new("player-1")),
        )
        .unwrap();

    assert_eq!(game.stack().len(), 1);
    assert_eq!(game.stack().top().unwrap().source_card_id(), &alice_spell);
    assert_eq!(
        game.priority().unwrap().current_holder(),
        &PlayerId::new("player-2")
    );

    service
        .tap_land(
            &mut game,
            TapLandCommand::new(PlayerId::new("player-2"), bob_land),
        )
        .unwrap();

    assert_eq!(game.stack().len(), 1);
    assert_eq!(game.stack().top().unwrap().source_card_id(), &alice_spell);
    assert_eq!(
        game.priority().unwrap().current_holder(),
        &PlayerId::new("player-2")
    );
}

#[test]
fn advance_turn_clears_mana_pools() {
    let (mut game, service) = create_game_with_land_on_battlefield();

    service
        .tap_land(
            &mut game,
            TapLandCommand::new(
                PlayerId::new("player-1"),
                CardInstanceId::new("game-1-player-1-0"),
            ),
        )
        .unwrap();

    assert_eq!(game.players()[0].mana(), 1);

    advance_turn_raw(&service, &mut game);

    assert_eq!(game.players()[0].mana(), 0);
    assert_eq!(game.players()[1].mana(), 0);
}

#[test]
fn tapping_a_forest_adds_green_mana_to_the_pool() {
    let service = create_service();
    let mut game = crate::support::start_two_player_game(&service, "game-green-mana");

    crate::support::deal_opening_hands(
        &service,
        &mut game,
        filled_library(vec![forest_card("forest")], 10),
        filled_library(vec![land_card("mountain")], 10),
    );

    advance_to_first_main_satisfying_cleanup(&service, &mut game);
    service
        .play_land(
            &mut game,
            PlayLandCommand::new(
                PlayerId::new("player-1"),
                CardInstanceId::new("game-green-mana-player-1-0"),
            ),
        )
        .unwrap();

    let (_, mana_event) = service
        .tap_land(
            &mut game,
            TapLandCommand::new(
                PlayerId::new("player-1"),
                CardInstanceId::new("game-green-mana-player-1-0"),
            ),
        )
        .unwrap();

    assert_eq!(mana_event.color, Some(demonictutor::ManaColor::Green));
    assert_eq!(game.players()[0].mana_pool().green(), 1);
    assert_eq!(game.players()[0].mana(), 1);
}
