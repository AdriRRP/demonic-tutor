#![allow(clippy::unwrap_used)]

use crate::support::{
    advance_to_first_main_satisfying_cleanup, advance_to_player_first_main_satisfying_cleanup,
    creature_card, filled_library, land_card, setup_two_player_game, targeted_damage_instant_card,
    targeted_player_damage_instant_card,
};
use demonictutor::{
    CardDefinitionId, CardInstanceId, CastSpellCommand, DomainError, GameEndReason, GameError,
    PlayerId, SpellCastOutcome, SpellTarget,
};

fn resolve_current_stack(
    service: &demonictutor::GameService<
        demonictutor::InMemoryEventStore,
        demonictutor::InMemoryEventBus,
    >,
    game: &mut demonictutor::Game,
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

fn hand_card_id_by_definition(
    game: &demonictutor::Game,
    player_index: usize,
    definition_id: &str,
) -> CardInstanceId {
    game.players()[player_index]
        .hand()
        .cards()
        .iter()
        .find(|card| card.definition_id() == &CardDefinitionId::new(definition_id))
        .unwrap()
        .id()
        .clone()
}

#[test]
fn targeted_instant_requires_a_target_when_cast() {
    let (service, mut game) = setup_two_player_game(
        "game-target-missing",
        filled_library(vec![targeted_damage_instant_card("shock", 0, 2)], 10),
        filled_library(vec![land_card("mountain")], 10),
    );

    advance_to_first_main_satisfying_cleanup(&service, &mut game);

    let result = service.cast_spell(
        &mut game,
        CastSpellCommand::new(
            PlayerId::new("player-1"),
            CardInstanceId::new("game-target-missing-player-1-0"),
        ),
    );

    assert!(matches!(
        result,
        Err(DomainError::Game(GameError::MissingSpellTarget(card_id)))
            if card_id == CardInstanceId::new("game-target-missing-player-1-0")
    ));
}

#[test]
fn targeted_instant_rejects_unknown_player_target_when_cast() {
    let (service, mut game) = setup_two_player_game(
        "game-target-player-invalid",
        filled_library(vec![targeted_damage_instant_card("shock", 0, 2)], 10),
        filled_library(vec![land_card("mountain")], 10),
    );

    advance_to_first_main_satisfying_cleanup(&service, &mut game);

    let result = service.cast_spell(
        &mut game,
        CastSpellCommand::new(
            PlayerId::new("player-1"),
            CardInstanceId::new("game-target-player-invalid-player-1-0"),
        )
        .with_target(SpellTarget::Player(PlayerId::new("missing-player"))),
    );

    assert!(matches!(
        result,
        Err(DomainError::Game(GameError::PlayerNotFound(player_id)))
            if player_id == PlayerId::new("missing-player")
    ));
}

#[test]
fn targeted_instant_deals_damage_to_target_player_when_it_resolves() {
    let (service, mut game) = setup_two_player_game(
        "game-target-player-resolve",
        filled_library(vec![targeted_damage_instant_card("shock", 0, 2)], 10),
        filled_library(vec![land_card("mountain")], 10),
    );

    advance_to_first_main_satisfying_cleanup(&service, &mut game);

    let shock_id = hand_card_id_by_definition(&game, 0, "shock");
    let outcome = service
        .cast_spell(
            &mut game,
            CastSpellCommand::new(PlayerId::new("player-1"), shock_id)
                .with_target(SpellTarget::Player(PlayerId::new("player-2"))),
        )
        .unwrap();

    assert_eq!(
        outcome.spell_put_on_stack.target,
        Some(SpellTarget::Player(PlayerId::new("player-2")))
    );

    let resolution = resolve_current_stack(&service, &mut game);
    assert_eq!(
        resolution.life_changed.as_ref().unwrap().player_id,
        PlayerId::new("player-2")
    );
    assert_eq!(resolution.life_changed.as_ref().unwrap().from_life, 20);
    assert_eq!(resolution.life_changed.as_ref().unwrap().to_life, 18);
    assert!(matches!(
        resolution.spell_cast.as_ref().unwrap().outcome,
        SpellCastOutcome::ResolvedToGraveyard
    ));
    assert_eq!(game.players()[1].life(), 18);
}

#[test]
fn targeted_player_damage_can_end_the_game() {
    let (service, mut game) = setup_two_player_game(
        "game-target-player-lethal",
        filled_library(vec![targeted_damage_instant_card("shock", 0, 2)], 10),
        filled_library(vec![land_card("mountain")], 10),
    );

    advance_to_first_main_satisfying_cleanup(&service, &mut game);
    service
        .adjust_player_life_effect(
            &mut game,
            demonictutor::AdjustPlayerLifeEffectCommand::new(
                PlayerId::new("player-1"),
                PlayerId::new("player-2"),
                -18,
            ),
        )
        .unwrap();

    let shock_id = hand_card_id_by_definition(&game, 0, "shock");
    service
        .cast_spell(
            &mut game,
            CastSpellCommand::new(PlayerId::new("player-1"), shock_id)
                .with_target(SpellTarget::Player(PlayerId::new("player-2"))),
        )
        .unwrap();

    let resolution = resolve_current_stack(&service, &mut game);
    assert_eq!(resolution.life_changed.as_ref().unwrap().to_life, 0);
    assert_eq!(
        resolution.game_ended.as_ref().unwrap().reason,
        GameEndReason::ZeroLife
    );
}

#[test]
fn targeted_instant_rejects_invalid_creature_target_when_cast() {
    let (service, mut game) = setup_two_player_game(
        "game-target-creature-invalid",
        filled_library(vec![targeted_damage_instant_card("shock", 0, 2)], 10),
        filled_library(vec![land_card("mountain")], 10),
    );

    advance_to_first_main_satisfying_cleanup(&service, &mut game);

    let result = service.cast_spell(
        &mut game,
        CastSpellCommand::new(
            PlayerId::new("player-1"),
            CardInstanceId::new("game-target-creature-invalid-player-1-0"),
        )
        .with_target(SpellTarget::Creature(CardInstanceId::new(
            "missing-creature",
        ))),
    );

    assert!(matches!(
        result,
        Err(DomainError::Game(GameError::InvalidCreatureTarget(card_id)))
            if card_id == CardInstanceId::new("missing-creature")
    ));
}

#[test]
fn targeted_player_spell_rejects_a_creature_target_when_cast() {
    let (service, mut game) = setup_two_player_game(
        "game-target-illegal-kind",
        filled_library(
            vec![
                land_card("alice-setup-land"),
                targeted_player_damage_instant_card("bolt", 0, 2),
            ],
            10,
        ),
        filled_library(vec![creature_card("bob-bear", 0, 2, 2)], 10),
    );
    let bolt_id = hand_card_id_by_definition(&game, 0, "bolt");

    advance_to_player_first_main_satisfying_cleanup(&service, &mut game, "player-2");
    let creature_id = CardInstanceId::new("game-target-illegal-kind-player-2-0");
    service
        .cast_spell(
            &mut game,
            CastSpellCommand::new(PlayerId::new("player-2"), creature_id.clone()),
        )
        .unwrap();
    let _ = resolve_current_stack(&service, &mut game);
    advance_to_player_first_main_satisfying_cleanup(&service, &mut game, "player-1");

    let result = service.cast_spell(
        &mut game,
        CastSpellCommand::new(PlayerId::new("player-1"), bolt_id.clone())
            .with_target(SpellTarget::Creature(creature_id)),
    );

    assert!(
        matches!(
            result,
            Err(DomainError::Game(GameError::IllegalSpellTarget(ref card_id)))
                if card_id == &bolt_id
        ),
        "unexpected result: {result:?}"
    );
}

#[test]
fn targeted_instant_deals_damage_to_target_creature_and_state_based_actions_destroy_it() {
    let (service, mut game) = setup_two_player_game(
        "game-target-creature-resolve",
        filled_library(
            vec![
                land_card("mountain"),
                targeted_damage_instant_card("shock", 0, 2),
            ],
            10,
        ),
        filled_library(vec![creature_card("bob-bear", 0, 2, 2)], 10),
    );

    advance_to_player_first_main_satisfying_cleanup(&service, &mut game, "player-2");
    let creature_id = CardInstanceId::new("game-target-creature-resolve-player-2-0");
    service
        .cast_spell(
            &mut game,
            CastSpellCommand::new(PlayerId::new("player-2"), creature_id.clone()),
        )
        .unwrap();
    let _ = resolve_current_stack(&service, &mut game);
    advance_to_player_first_main_satisfying_cleanup(&service, &mut game, "player-1");

    let shock_id = hand_card_id_by_definition(&game, 0, "shock");
    service
        .cast_spell(
            &mut game,
            CastSpellCommand::new(PlayerId::new("player-1"), shock_id)
                .with_target(SpellTarget::Creature(creature_id.clone())),
        )
        .unwrap();

    let resolution = resolve_current_stack(&service, &mut game);
    assert!(resolution.life_changed.is_none());
    assert_eq!(resolution.creatures_died.len(), 1);
    assert_eq!(resolution.creatures_died[0].card_id, creature_id);
    assert!(game.players()[1]
        .battlefield()
        .cards()
        .iter()
        .all(|card| card.definition_id() != &CardDefinitionId::new("bob-bear")));
    assert_eq!(game.players()[1].graveyard().cards().len(), 1);
}

#[test]
fn targeted_instant_does_not_apply_if_its_only_creature_target_is_gone_on_resolution() {
    let (service, mut game) = setup_two_player_game(
        "game-target-creature-gone",
        filled_library(
            vec![
                land_card("mountain"),
                targeted_damage_instant_card("shock-a", 0, 2),
                targeted_damage_instant_card("shock-b", 0, 2),
            ],
            10,
        ),
        filled_library(vec![creature_card("bob-bear", 0, 2, 2)], 10),
    );

    advance_to_player_first_main_satisfying_cleanup(&service, &mut game, "player-2");
    let creature_id = CardInstanceId::new("game-target-creature-gone-player-2-0");
    service
        .cast_spell(
            &mut game,
            CastSpellCommand::new(PlayerId::new("player-2"), creature_id.clone()),
        )
        .unwrap();
    let _ = resolve_current_stack(&service, &mut game);
    advance_to_player_first_main_satisfying_cleanup(&service, &mut game, "player-1");

    let first_shock_id = hand_card_id_by_definition(&game, 0, "shock-a");
    service
        .cast_spell(
            &mut game,
            CastSpellCommand::new(PlayerId::new("player-1"), first_shock_id)
                .with_target(SpellTarget::Creature(creature_id.clone())),
        )
        .unwrap();

    let second_shock_id = hand_card_id_by_definition(&game, 0, "shock-b");
    service
        .cast_spell(
            &mut game,
            CastSpellCommand::new(PlayerId::new("player-1"), second_shock_id)
                .with_target(SpellTarget::Creature(creature_id.clone())),
        )
        .unwrap();

    let first_resolution = resolve_current_stack(&service, &mut game);
    assert_eq!(first_resolution.creatures_died.len(), 1);
    assert_eq!(first_resolution.creatures_died[0].card_id, creature_id);

    let second_resolution = resolve_current_stack(&service, &mut game);
    assert!(second_resolution.life_changed.is_none());
    assert!(second_resolution.creatures_died.is_empty());
    assert!(game.players()[1].battlefield().cards().is_empty());
    assert_eq!(game.players()[1].graveyard().cards().len(), 1);
}
