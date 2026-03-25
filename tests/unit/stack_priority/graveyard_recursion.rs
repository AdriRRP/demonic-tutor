#![allow(clippy::panic)]
#![allow(clippy::unwrap_used)]

use {
    crate::support::{
        advance_to_first_main_satisfying_cleanup, advance_to_player_first_main_satisfying_cleanup,
        cast_spell_and_resolve, creature_card, etb_life_gain_creature_card, filled_library,
        graveyard_cast_instant_card, mill_self_sorcery_card, mill_target_player_sorcery_card,
        player, reanimate_target_creature_card_sorcery_card,
        return_target_creature_card_from_graveyard_to_hand_sorcery_card, setup_two_player_game,
        target_player_discards_chosen_card_sorcery_card, targeted_destroy_creature_instant_card,
    },
    demonictutor::{CastSpellCommand, PassPriorityCommand, PlayerId, SpellChoice, SpellTarget},
};

fn cast_with_target_and_resolve(
    service: &crate::support::TestService,
    game: &mut demonictutor::Game,
    player_id: &str,
    card_id: demonictutor::CardInstanceId,
    target: SpellTarget,
) {
    service
        .cast_spell(
            game,
            CastSpellCommand::new(PlayerId::new(player_id), card_id).with_target(target),
        )
        .unwrap();
    service
        .pass_priority(game, PassPriorityCommand::new(PlayerId::new(player_id)))
        .unwrap();
    let second_holder = game.priority().unwrap().current_holder().clone();
    service
        .pass_priority(game, PassPriorityCommand::new(second_holder))
        .unwrap();
}

#[test]
fn supported_spell_can_return_target_creature_card_from_graveyard_to_hand() {
    let (service, mut game) = setup_two_player_game(
        "game-return-creature-from-graveyard",
        filled_library(
            vec![
                creature_card("bear", 0, 2, 2),
                targeted_destroy_creature_instant_card("doom-pebble", 0),
                return_target_creature_card_from_graveyard_to_hand_sorcery_card("raise-remains", 0),
            ],
            10,
        ),
        filled_library(Vec::new(), 10),
    );
    advance_to_first_main_satisfying_cleanup(&service, &mut game);

    let bear_id = player(&game, "player-1")
        .hand_card_by_definition(&demonictutor::CardDefinitionId::new("bear"))
        .unwrap()
        .id()
        .clone();
    cast_spell_and_resolve(&service, &mut game, "player-1", bear_id.clone());

    let destroy_id = player(&game, "player-1")
        .hand_card_by_definition(&demonictutor::CardDefinitionId::new("doom-pebble"))
        .unwrap()
        .id()
        .clone();
    cast_with_target_and_resolve(
        &service,
        &mut game,
        "player-1",
        destroy_id,
        SpellTarget::Creature(bear_id.clone()),
    );

    let recursion_id = player(&game, "player-1")
        .hand_card_by_definition(&demonictutor::CardDefinitionId::new("raise-remains"))
        .unwrap()
        .id()
        .clone();
    cast_with_target_and_resolve(
        &service,
        &mut game,
        "player-1",
        recursion_id,
        SpellTarget::GraveyardCard(bear_id.clone()),
    );

    let owner = player(&game, "player-1");
    assert!(owner.hand_card(&bear_id).is_some());
    assert!(owner.graveyard_card(&bear_id).is_none());
}

#[test]
fn supported_spell_can_reanimate_target_creature_card_and_fire_etb() {
    let (service, mut game) = setup_two_player_game(
        "game-reanimate-creature",
        filled_library(
            vec![
                etb_life_gain_creature_card("healer-cub", 0, 2, 2, 2),
                targeted_destroy_creature_instant_card("doom-pebble", 0),
                reanimate_target_creature_card_sorcery_card("raise-cub", 0),
            ],
            10,
        ),
        filled_library(Vec::new(), 10),
    );
    advance_to_first_main_satisfying_cleanup(&service, &mut game);

    let cub_id = player(&game, "player-1")
        .hand_card_by_definition(&demonictutor::CardDefinitionId::new("healer-cub"))
        .unwrap()
        .id()
        .clone();
    service
        .cast_spell(
            &mut game,
            CastSpellCommand::new(PlayerId::new("player-1"), cub_id.clone()),
        )
        .unwrap();
    service
        .pass_priority(
            &mut game,
            PassPriorityCommand::new(PlayerId::new("player-1")),
        )
        .unwrap();
    service
        .pass_priority(
            &mut game,
            PassPriorityCommand::new(PlayerId::new("player-2")),
        )
        .unwrap();
    service
        .pass_priority(
            &mut game,
            PassPriorityCommand::new(PlayerId::new("player-1")),
        )
        .unwrap();
    service
        .pass_priority(
            &mut game,
            PassPriorityCommand::new(PlayerId::new("player-2")),
        )
        .unwrap();

    let destroy_id = player(&game, "player-1")
        .hand_card_by_definition(&demonictutor::CardDefinitionId::new("doom-pebble"))
        .unwrap()
        .id()
        .clone();
    cast_with_target_and_resolve(
        &service,
        &mut game,
        "player-1",
        destroy_id,
        SpellTarget::Creature(cub_id.clone()),
    );

    let reanimate_id = player(&game, "player-1")
        .hand_card_by_definition(&demonictutor::CardDefinitionId::new("raise-cub"))
        .unwrap()
        .id()
        .clone();
    cast_with_target_and_resolve(
        &service,
        &mut game,
        "player-1",
        reanimate_id,
        SpellTarget::GraveyardCard(cub_id.clone()),
    );
    service
        .pass_priority(
            &mut game,
            PassPriorityCommand::new(PlayerId::new("player-1")),
        )
        .unwrap();
    service
        .pass_priority(
            &mut game,
            PassPriorityCommand::new(PlayerId::new("player-2")),
        )
        .unwrap();

    let owner = player(&game, "player-1");
    assert!(owner.battlefield_card(&cub_id).is_some());
    assert_eq!(owner.life(), 24);
}

#[test]
fn supported_reanimation_puts_an_opponents_graveyard_creature_onto_the_casters_battlefield() {
    let (service, mut game) = setup_two_player_game(
        "game-reanimate-opponents-creature",
        filled_library(
            vec![
                target_player_discards_chosen_card_sorcery_card("coercion-lite", 0),
                reanimate_target_creature_card_sorcery_card("raise-rival", 0),
            ],
            10,
        ),
        filled_library(vec![creature_card("rival-bear", 0, 2, 2)], 10),
    );

    advance_to_first_main_satisfying_cleanup(&service, &mut game);

    let rival_bear_id = player(&game, "player-2")
        .hand_card_by_definition(&demonictutor::CardDefinitionId::new("rival-bear"))
        .unwrap()
        .id()
        .clone();

    let discard_id = player(&game, "player-1")
        .hand_card_by_definition(&demonictutor::CardDefinitionId::new("coercion-lite"))
        .unwrap()
        .id()
        .clone();
    service
        .cast_spell(
            &mut game,
            CastSpellCommand::new(PlayerId::new("player-1"), discard_id)
                .with_target(SpellTarget::Player(PlayerId::new("player-2")))
                .with_choice(SpellChoice::HandCard(rival_bear_id.clone())),
        )
        .unwrap();
    service
        .pass_priority(
            &mut game,
            PassPriorityCommand::new(PlayerId::new("player-1")),
        )
        .unwrap();
    service
        .pass_priority(
            &mut game,
            PassPriorityCommand::new(PlayerId::new("player-2")),
        )
        .unwrap();

    let reanimate_id = player(&game, "player-1")
        .hand_card_by_definition(&demonictutor::CardDefinitionId::new("raise-rival"))
        .unwrap()
        .id()
        .clone();
    cast_with_target_and_resolve(
        &service,
        &mut game,
        "player-1",
        reanimate_id,
        SpellTarget::GraveyardCard(rival_bear_id.clone()),
    );

    assert!(player(&game, "player-1")
        .battlefield_card(&rival_bear_id)
        .is_some());
    assert!(player(&game, "player-2")
        .battlefield_card(&rival_bear_id)
        .is_none());
    assert!(player(&game, "player-2")
        .graveyard_card(&rival_bear_id)
        .is_none());
}

#[test]
fn supported_mill_effects_move_cards_from_library_to_graveyard() {
    let (service, mut game) = setup_two_player_game(
        "game-mill-foundation",
        filled_library(vec![mill_self_sorcery_card("study-loss", 0, 2)], 12),
        filled_library(
            vec![mill_target_player_sorcery_card("memory-drain", 0, 2)],
            12,
        ),
    );
    advance_to_first_main_satisfying_cleanup(&service, &mut game);

    let self_mill_id = player(&game, "player-1")
        .hand_card_by_definition(&demonictutor::CardDefinitionId::new("study-loss"))
        .unwrap()
        .id()
        .clone();
    cast_spell_and_resolve(&service, &mut game, "player-1", self_mill_id);
    assert_eq!(player(&game, "player-1").graveyard_size(), 3);

    advance_to_player_first_main_satisfying_cleanup(&service, &mut game, "player-2");

    let target_mill_id = player(&game, "player-2")
        .hand_card_by_definition(&demonictutor::CardDefinitionId::new("memory-drain"))
        .unwrap()
        .id()
        .clone();
    let prior_graveyard_size = player(&game, "player-1").graveyard_size();
    cast_with_target_and_resolve(
        &service,
        &mut game,
        "player-2",
        target_mill_id,
        SpellTarget::Player(PlayerId::new("player-1")),
    );

    assert_eq!(
        player(&game, "player-1").graveyard_size(),
        prior_graveyard_size + 2
    );
}

#[test]
fn supported_mill_effects_move_as_many_cards_as_possible_from_a_short_library() {
    let (service, mut game) = setup_two_player_game(
        "game-mill-short-library",
        filled_library(vec![mill_self_sorcery_card("study-loss", 0, 3)], 10),
        filled_library(Vec::new(), 10),
    );
    advance_to_first_main_satisfying_cleanup(&service, &mut game);

    let self_mill_id = player(&game, "player-1")
        .hand_card_by_definition(&demonictutor::CardDefinitionId::new("study-loss"))
        .unwrap()
        .id()
        .clone();

    cast_spell_and_resolve(&service, &mut game, "player-1", self_mill_id);

    assert_eq!(player(&game, "player-1").graveyard_size(), 3);
    assert_eq!(player(&game, "player-1").library_size(), 0);
}

#[test]
fn explicit_profile_allows_casting_a_supported_spell_from_its_own_graveyard() {
    let (service, mut game) = setup_two_player_game(
        "game-cast-from-graveyard",
        filled_library(vec![graveyard_cast_instant_card("echo-bolt", 0, 2)], 10),
        filled_library(Vec::new(), 10),
    );
    advance_to_first_main_satisfying_cleanup(&service, &mut game);

    let bolt_id = player(&game, "player-1")
        .hand_card_by_definition(&demonictutor::CardDefinitionId::new("echo-bolt"))
        .unwrap()
        .id()
        .clone();
    cast_with_target_and_resolve(
        &service,
        &mut game,
        "player-1",
        bolt_id.clone(),
        SpellTarget::Player(PlayerId::new("player-2")),
    );

    cast_with_target_and_resolve(
        &service,
        &mut game,
        "player-1",
        bolt_id.clone(),
        SpellTarget::Player(PlayerId::new("player-2")),
    );

    assert_eq!(player(&game, "player-2").life(), 16);
    assert!(player(&game, "player-1").graveyard_card(&bolt_id).is_some());
}
