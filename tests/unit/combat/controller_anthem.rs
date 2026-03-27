#![allow(clippy::expect_used)]

//! Unit coverage for unit combat controller anthem.

use {
    crate::support,
    demonictutor::{CardDefinitionId, CastSpellCommand, PassPriorityCommand, PlayerId},
};

fn cast_and_resolve(
    service: &support::TestService,
    game: &mut demonictutor::Game,
    player_id: &str,
    card_id: demonictutor::CardInstanceId,
) {
    service
        .cast_spell(
            game,
            CastSpellCommand::new(PlayerId::new(player_id), card_id),
        )
        .expect("setup cast should succeed");
    service
        .pass_priority(game, PassPriorityCommand::new(PlayerId::new(player_id)))
        .expect("controller should pass");
    let other = if player_id == "player-1" {
        PlayerId::new("player-2")
    } else {
        PlayerId::new("player-1")
    };
    service
        .pass_priority(game, PassPriorityCommand::new(other))
        .expect("opponent should pass");
}

#[test]
fn controller_anthem_boosts_existing_controlled_creatures() {
    let (service, mut game) = support::setup_two_player_game(
        "game-anthem-existing",
        support::filled_library(
            vec![
                support::creature_card("bear", 0, 2, 2),
                support::anthem_enchantment_card("battle-anthem", 0),
            ],
            10,
        ),
        support::filled_library(Vec::new(), 10),
    );
    support::advance_to_first_main_satisfying_cleanup(&service, &mut game);

    let creature_id = game.players()[0]
        .hand_card_by_definition(&CardDefinitionId::new("bear"))
        .expect("bear should be in hand")
        .id()
        .clone();
    cast_and_resolve(&service, &mut game, "player-1", creature_id.clone());

    let anthem_id = game.players()[0]
        .hand_card_by_definition(&CardDefinitionId::new("battle-anthem"))
        .expect("anthem should be in hand")
        .id()
        .clone();
    cast_and_resolve(&service, &mut game, "player-1", anthem_id);

    assert_eq!(
        game.players()[0]
            .battlefield_card(&creature_id)
            .and_then(demonictutor::CardInstance::creature_stats),
        Some((3, 3))
    );
}

#[test]
fn controller_anthem_boosts_creatures_that_enter_after_it() {
    let (service, mut game) = support::setup_two_player_game(
        "game-anthem-later-creature",
        support::filled_library(
            vec![
                support::anthem_enchantment_card("battle-anthem", 0),
                support::creature_card("bear", 0, 2, 2),
            ],
            10,
        ),
        support::filled_library(Vec::new(), 10),
    );
    support::advance_to_first_main_satisfying_cleanup(&service, &mut game);

    let anthem_id = game.players()[0]
        .hand_card_by_definition(&CardDefinitionId::new("battle-anthem"))
        .expect("anthem should be in hand")
        .id()
        .clone();
    cast_and_resolve(&service, &mut game, "player-1", anthem_id);

    let creature_id = game.players()[0]
        .hand_card_by_definition(&CardDefinitionId::new("bear"))
        .expect("bear should be in hand")
        .id()
        .clone();
    cast_and_resolve(&service, &mut game, "player-1", creature_id.clone());

    assert_eq!(
        game.players()[0]
            .battlefield_card(&creature_id)
            .and_then(demonictutor::CardInstance::creature_stats),
        Some((3, 3))
    );
}

#[test]
fn controller_anthem_bonus_is_removed_when_the_anthem_leaves_battlefield() {
    let (service, mut game) = support::setup_two_player_game(
        "game-anthem-removed",
        support::filled_library(
            vec![
                support::creature_card("bear", 0, 2, 2),
                support::anthem_enchantment_card("battle-anthem", 0),
            ],
            10,
        ),
        support::filled_library(
            vec![
                support::destroy_target_artifact_or_enchantment_instant_card("disenchant-lite", 0),
            ],
            10,
        ),
    );
    support::advance_to_first_main_satisfying_cleanup(&service, &mut game);

    let creature_id = game.players()[0]
        .hand_card_by_definition(&CardDefinitionId::new("bear"))
        .expect("bear should be in hand")
        .id()
        .clone();
    cast_and_resolve(&service, &mut game, "player-1", creature_id.clone());
    let anthem_id = game.players()[0]
        .hand_card_by_definition(&CardDefinitionId::new("battle-anthem"))
        .expect("anthem should be in hand")
        .id()
        .clone();
    cast_and_resolve(&service, &mut game, "player-1", anthem_id.clone());
    assert_eq!(
        game.players()[0]
            .battlefield_card(&creature_id)
            .and_then(demonictutor::CardInstance::creature_stats),
        Some((3, 3))
    );

    service
        .pass_priority(
            &mut game,
            PassPriorityCommand::new(PlayerId::new("player-1")),
        )
        .expect("player-1 should pass");
    let disenchant_id = game.players()[1]
        .hand_card_at(0)
        .expect("disenchant should be in hand")
        .id()
        .clone();
    service
        .cast_spell(
            &mut game,
            CastSpellCommand::new(PlayerId::new("player-2"), disenchant_id)
                .with_target(demonictutor::SpellTarget::Permanent(anthem_id.clone())),
        )
        .expect("removal should cast");
    service
        .pass_priority(
            &mut game,
            PassPriorityCommand::new(PlayerId::new("player-2")),
        )
        .expect("player-2 should pass");
    service
        .pass_priority(
            &mut game,
            PassPriorityCommand::new(PlayerId::new("player-1")),
        )
        .expect("player-1 should pass");

    assert!(game.players()[0].graveyard_card(&anthem_id).is_some());
    assert_eq!(
        game.players()[0]
            .battlefield_card(&creature_id)
            .and_then(demonictutor::CardInstance::creature_stats),
        Some((2, 2))
    );
}
