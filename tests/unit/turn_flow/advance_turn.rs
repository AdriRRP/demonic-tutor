#![allow(clippy::unwrap_used)]

use crate::support::{
    advance_n_satisfying_cleanup, advance_to_first_main_satisfying_cleanup,
    advance_to_player_first_main_satisfying_cleanup, advance_turn_satisfying_cleanup,
    filled_library, land_card, setup_two_player_game, vanilla_creature,
};
use demonictutor::{
    AdvanceTurnCommand, CardDefinitionId, CardInstanceId, CastSpellCommand,
    DeclareAttackersCommand, DeclareBlockersCommand, LibraryCard, Phase, PlayLandCommand, PlayerId,
    ResolveCombatDamageCommand,
};

fn create_game_with_land_in_hand() -> demonictutor::Game {
    let (.., game) = setup_two_player_game(
        "game-1",
        filled_library(vec![land_card("forest")], 10),
        filled_library(vec![land_card("mountain")], 10),
    );
    game
}

#[test]
fn advance_turn_changes_active_player() {
    let (service, mut game) = setup_two_player_game(
        "game-1",
        filled_library(vec![land_card("forest")], 10),
        filled_library(vec![land_card("mountain")], 10),
    );

    assert_eq!(game.active_player().as_str(), "player-1");
    assert_eq!(game.phase(), &Phase::Setup);

    let expected = [
        ("player-1", Phase::Untap),
        ("player-1", Phase::Upkeep),
        ("player-1", Phase::Draw),
        ("player-1", Phase::FirstMain),
        ("player-1", Phase::Combat),
        ("player-1", Phase::SecondMain),
        ("player-1", Phase::EndStep),
        ("player-2", Phase::Untap),
    ];

    for (player, phase) in expected {
        advance_turn_satisfying_cleanup(&service, &mut game);
        assert_eq!(game.active_player().as_str(), player);
        assert_eq!(game.phase(), &phase);
    }
}

#[test]
fn advance_turn_emits_event() {
    let (service, mut game) = setup_two_player_game(
        "game-1",
        filled_library(vec![land_card("forest")], 10),
        filled_library(vec![land_card("mountain")], 10),
    );

    advance_n_satisfying_cleanup(&service, &mut game, 2);
    let event = service
        .advance_turn(&mut game, AdvanceTurnCommand::new())
        .unwrap();

    assert_eq!(event.active_player.as_str(), "player-1");
}

#[test]
fn advance_turn_resets_lands_played() {
    let mut game = create_game_with_land_in_hand();
    let service = crate::support::create_service();

    advance_to_player_first_main_satisfying_cleanup(&service, &mut game, "player-2");
    service
        .play_land(
            &mut game,
            PlayLandCommand::new(
                PlayerId::new("player-2"),
                CardInstanceId::new("game-1-player-2-0"),
            ),
        )
        .unwrap();

    assert_eq!(game.players()[1].lands_played_this_turn(), 1);

    advance_to_player_first_main_satisfying_cleanup(&service, &mut game, "player-1");

    assert_eq!(game.players()[0].lands_played_this_turn(), 0);
    assert_eq!(game.players()[1].lands_played_this_turn(), 1);

    advance_n_satisfying_cleanup(&service, &mut game, 4);

    assert_eq!(game.players()[1].lands_played_this_turn(), 0);
}

#[test]
fn advance_turn_allows_playing_land_after_turn_change() {
    let mut game = create_game_with_land_in_hand();
    let service = crate::support::create_service();

    advance_to_first_main_satisfying_cleanup(&service, &mut game);
    assert!(service
        .play_land(
            &mut game,
            PlayLandCommand::new(
                PlayerId::new("player-1"),
                CardInstanceId::new("game-1-player-1-0"),
            ),
        )
        .is_ok());

    advance_to_player_first_main_satisfying_cleanup(&service, &mut game, "player-2");

    assert!(service
        .play_land(
            &mut game,
            PlayLandCommand::new(
                PlayerId::new("player-2"),
                CardInstanceId::new("game-1-player-2-0"),
            ),
        )
        .is_ok());
}

#[test]
fn advance_turn_clears_marked_damage_when_turn_ends() {
    let (service, mut game) = setup_two_player_game(
        "game-1",
        filled_library(
            vec![LibraryCard::creature(
                CardDefinitionId::new("ogre"),
                0,
                3,
                3,
            )],
            10,
        ),
        filled_library(vec![vanilla_creature("soldier")], 10),
    );

    let attacker_id = CardInstanceId::new("game-1-player-1-0");
    let blocker_id = CardInstanceId::new("game-1-player-2-0");

    advance_to_player_first_main_satisfying_cleanup(&service, &mut game, "player-1");
    service
        .cast_spell(
            &mut game,
            CastSpellCommand::new(PlayerId::new("player-1"), attacker_id.clone()),
        )
        .unwrap();

    advance_to_player_first_main_satisfying_cleanup(&service, &mut game, "player-2");
    service
        .cast_spell(
            &mut game,
            CastSpellCommand::new(PlayerId::new("player-2"), blocker_id.clone()),
        )
        .unwrap();

    advance_to_player_first_main_satisfying_cleanup(&service, &mut game, "player-1");
    advance_turn_satisfying_cleanup(&service, &mut game);
    assert_eq!(game.phase(), &Phase::Combat);
    assert_eq!(game.active_player(), &PlayerId::new("player-1"));

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

    advance_n_satisfying_cleanup(&service, &mut game, 3);

    assert_eq!(game.phase(), &Phase::Untap);
    assert_eq!(game.players()[0].battlefield().cards()[0].damage(), 0);
}
