#![allow(clippy::unwrap_used)]

use crate::support::{
    advance_to_first_main_satisfying_cleanup, advance_to_player_first_main_satisfying_cleanup,
    creature_card, filled_library, land_card, setup_two_player_game,
    targeted_attacking_creature_damage_instant_card,
    targeted_controlled_creature_damage_instant_card, targeted_damage_instant_card,
    targeted_destroy_creature_instant_card, targeted_exile_creature_instant_card,
    targeted_opponent_damage_instant_card, targeted_opponents_creature_damage_instant_card,
    targeted_player_damage_instant_card,
};
use demonictutor::{
    CardDefinitionId, CardInstanceId, CastSpellCommand, DeclareAttackersCommand, DomainError,
    GameEndReason, GameError, LibraryCard, PlayerId, SpellCastOutcome, SpellTarget,
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
        .hand_card_by_definition(&CardDefinitionId::new(definition_id))
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
        Err(DomainError::Game(GameError::InvalidPlayerTarget(player_id)))
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
fn targeted_attacking_creature_spell_rejects_a_player_target_when_cast() {
    let (service, mut game) = setup_two_player_game(
        "game-target-attacking-illegal-kind",
        filled_library(
            vec![targeted_attacking_creature_damage_instant_card(
                "marked-for-battle",
                0,
                2,
            )],
            10,
        ),
        filled_library(Vec::new(), 10),
    );

    advance_to_first_main_satisfying_cleanup(&service, &mut game);

    let spell_id = hand_card_id_by_definition(&game, 0, "marked-for-battle");
    let result = service.cast_spell(
        &mut game,
        CastSpellCommand::new(PlayerId::new("player-1"), spell_id.clone())
            .with_target(SpellTarget::Player(PlayerId::new("player-2"))),
    );

    assert!(matches!(
        result,
        Err(DomainError::Game(GameError::IllegalSpellTarget(card_id))) if card_id == spell_id
    ));
}

#[test]
fn targeted_attacking_creature_spell_rejects_a_non_attacking_creature_when_cast() {
    let (service, mut game) = setup_two_player_game(
        "game-target-attacking-nonattacker",
        filled_library(
            vec![
                land_card("alice-setup-land"),
                targeted_attacking_creature_damage_instant_card("marked-for-battle", 0, 2),
            ],
            10,
        ),
        filled_library(vec![creature_card("bob-bear", 0, 2, 2)], 10),
    );
    let spell_id = hand_card_id_by_definition(&game, 0, "marked-for-battle");

    advance_to_player_first_main_satisfying_cleanup(&service, &mut game, "player-2");
    let creature_id = CardInstanceId::new("game-target-attacking-nonattacker-player-2-0");
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
        CastSpellCommand::new(PlayerId::new("player-1"), spell_id.clone())
            .with_target(SpellTarget::Creature(creature_id)),
    );

    assert!(matches!(
        result,
        Err(DomainError::Game(GameError::IllegalSpellTarget(card_id))) if card_id == spell_id
    ));
}

#[test]
fn targeted_attacking_creature_spell_can_destroy_an_attacker_after_attackers() {
    let (service, mut game) = setup_two_player_game(
        "game-target-attacking-lethal",
        filled_library(
            vec![
                LibraryCard::creature(CardDefinitionId::new("attacker"), 0, 2, 2),
                targeted_attacking_creature_damage_instant_card("marked-for-battle", 0, 2),
            ],
            10,
        ),
        filled_library(Vec::new(), 10),
    );

    advance_to_player_first_main_satisfying_cleanup(&service, &mut game, "player-1");
    let attacker_id = CardInstanceId::new("game-target-attacking-lethal-player-1-0");
    service
        .cast_spell(
            &mut game,
            CastSpellCommand::new(PlayerId::new("player-1"), attacker_id.clone()),
        )
        .unwrap();
    let _ = resolve_current_stack(&service, &mut game);

    advance_to_player_first_main_satisfying_cleanup(&service, &mut game, "player-2");
    advance_to_player_first_main_satisfying_cleanup(&service, &mut game, "player-1");
    crate::support::advance_turn_raw(&service, &mut game);
    crate::support::close_empty_priority_window(&service, &mut game);
    crate::support::advance_turn_raw(&service, &mut game);
    service
        .declare_attackers(
            &mut game,
            DeclareAttackersCommand::new(PlayerId::new("player-1"), vec![attacker_id.clone()]),
        )
        .unwrap();

    let spell_id = hand_card_id_by_definition(&game, 0, "marked-for-battle");
    service
        .cast_spell(
            &mut game,
            CastSpellCommand::new(PlayerId::new("player-1"), spell_id)
                .with_target(SpellTarget::Creature(attacker_id.clone())),
        )
        .unwrap();

    let resolution = resolve_current_stack(&service, &mut game);
    assert_eq!(resolution.creatures_died.len(), 1);
    assert_eq!(resolution.creatures_died[0].card_id, attacker_id);
    assert!(game.players()[0]
        .battlefield_cards()
        .all(|card| card.id() != &resolution.creatures_died[0].card_id));
}

#[test]
fn targeted_attacking_creature_spell_marks_nonlethal_damage_and_leaves_the_attacker_in_combat() {
    let (service, mut game) = setup_two_player_game(
        "game-target-attacking-nonlethal",
        filled_library(
            vec![
                LibraryCard::creature(CardDefinitionId::new("attacker"), 0, 3, 3),
                targeted_attacking_creature_damage_instant_card("marked-for-battle", 0, 1),
            ],
            10,
        ),
        filled_library(Vec::new(), 10),
    );

    advance_to_player_first_main_satisfying_cleanup(&service, &mut game, "player-1");
    let attacker_id = CardInstanceId::new("game-target-attacking-nonlethal-player-1-0");
    service
        .cast_spell(
            &mut game,
            CastSpellCommand::new(PlayerId::new("player-1"), attacker_id.clone()),
        )
        .unwrap();
    let _ = resolve_current_stack(&service, &mut game);

    advance_to_player_first_main_satisfying_cleanup(&service, &mut game, "player-2");
    advance_to_player_first_main_satisfying_cleanup(&service, &mut game, "player-1");
    crate::support::advance_turn_raw(&service, &mut game);
    crate::support::close_empty_priority_window(&service, &mut game);
    crate::support::advance_turn_raw(&service, &mut game);
    service
        .declare_attackers(
            &mut game,
            DeclareAttackersCommand::new(PlayerId::new("player-1"), vec![attacker_id.clone()]),
        )
        .unwrap();

    let spell_id = hand_card_id_by_definition(&game, 0, "marked-for-battle");
    service
        .cast_spell(
            &mut game,
            CastSpellCommand::new(PlayerId::new("player-1"), spell_id)
                .with_target(SpellTarget::Creature(attacker_id.clone())),
        )
        .unwrap();

    let resolution = resolve_current_stack(&service, &mut game);
    assert!(resolution.creatures_died.is_empty());

    let attacker = game.players()[0]
        .battlefield_cards()
        .find(|card| card.id() == &attacker_id)
        .unwrap();
    assert_eq!(attacker.damage(), 1);
    assert!(attacker.is_attacking());
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
        .battlefield_cards()
        .all(|card| card.definition_id() != &CardDefinitionId::new("bob-bear")));
    assert_eq!(game.players()[1].graveyard_size(), 1);
}

#[test]
fn targeted_opponent_spell_rejects_the_caster_as_target_when_cast() {
    let (service, mut game) = setup_two_player_game(
        "game-target-opponent-only",
        filled_library(
            vec![targeted_opponent_damage_instant_card("lava-spike", 0, 2)],
            10,
        ),
        filled_library(vec![land_card("mountain")], 10),
    );

    advance_to_first_main_satisfying_cleanup(&service, &mut game);

    let spike_id = hand_card_id_by_definition(&game, 0, "lava-spike");
    let result = service.cast_spell(
        &mut game,
        CastSpellCommand::new(PlayerId::new("player-1"), spike_id.clone())
            .with_target(SpellTarget::Player(PlayerId::new("player-1"))),
    );

    assert!(matches!(
        result,
        Err(DomainError::Game(GameError::IllegalSpellTarget(card_id))) if card_id == spike_id
    ));
}

#[test]
fn targeted_opponent_spell_can_target_the_opponent_when_it_resolves() {
    let (service, mut game) = setup_two_player_game(
        "game-target-opponent-resolve",
        filled_library(
            vec![targeted_opponent_damage_instant_card("lava-spike", 0, 2)],
            10,
        ),
        filled_library(vec![land_card("mountain")], 10),
    );

    advance_to_first_main_satisfying_cleanup(&service, &mut game);

    let spike_id = hand_card_id_by_definition(&game, 0, "lava-spike");
    service
        .cast_spell(
            &mut game,
            CastSpellCommand::new(PlayerId::new("player-1"), spike_id)
                .with_target(SpellTarget::Player(PlayerId::new("player-2"))),
        )
        .unwrap();

    let resolution = resolve_current_stack(&service, &mut game);
    assert_eq!(
        resolution.life_changed.as_ref().unwrap().player_id,
        PlayerId::new("player-2")
    );
    assert_eq!(resolution.life_changed.as_ref().unwrap().to_life, 18);
}

#[test]
fn targeted_any_player_spell_can_target_the_caster_when_it_resolves() {
    let (service, mut game) = setup_two_player_game(
        "game-target-any-player-self",
        filled_library(vec![targeted_damage_instant_card("shock", 0, 2)], 10),
        filled_library(vec![land_card("mountain")], 10),
    );

    advance_to_first_main_satisfying_cleanup(&service, &mut game);

    let shock_id = hand_card_id_by_definition(&game, 0, "shock");
    service
        .cast_spell(
            &mut game,
            CastSpellCommand::new(PlayerId::new("player-1"), shock_id)
                .with_target(SpellTarget::Player(PlayerId::new("player-1"))),
        )
        .unwrap();

    let resolution = resolve_current_stack(&service, &mut game);
    assert_eq!(
        resolution.life_changed.as_ref().unwrap().player_id,
        PlayerId::new("player-1")
    );
    assert_eq!(resolution.life_changed.as_ref().unwrap().to_life, 18);
}

#[test]
fn targeted_controlled_creature_spell_rejects_an_opponents_creature_when_cast() {
    let (service, mut game) = setup_two_player_game(
        "game-target-controlled-creature-invalid",
        filled_library(
            vec![
                land_card("alice-setup-land"),
                targeted_controlled_creature_damage_instant_card("reckless-surge", 0, 2),
            ],
            10,
        ),
        filled_library(vec![creature_card("bob-bear", 0, 2, 2)], 10),
    );
    let spell_id = hand_card_id_by_definition(&game, 0, "reckless-surge");

    advance_to_player_first_main_satisfying_cleanup(&service, &mut game, "player-2");
    let creature_id = CardInstanceId::new("game-target-controlled-creature-invalid-player-2-0");
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
        CastSpellCommand::new(PlayerId::new("player-1"), spell_id.clone())
            .with_target(SpellTarget::Creature(creature_id)),
    );

    assert!(matches!(
        result,
        Err(DomainError::Game(GameError::IllegalSpellTarget(card_id))) if card_id == spell_id
    ));
}

#[test]
fn targeted_opponents_creature_spell_can_target_an_opponents_creature_when_cast() {
    let (service, mut game) = setup_two_player_game(
        "game-target-opponents-creature-cast",
        filled_library(
            vec![
                land_card("alice-setup-land"),
                targeted_opponents_creature_damage_instant_card("hostile-bolt", 0, 2),
            ],
            10,
        ),
        filled_library(vec![creature_card("bob-bear", 0, 2, 2)], 10),
    );
    let spell_id = hand_card_id_by_definition(&game, 0, "hostile-bolt");

    advance_to_player_first_main_satisfying_cleanup(&service, &mut game, "player-2");
    let creature_id = CardInstanceId::new("game-target-opponents-creature-cast-player-2-0");
    service
        .cast_spell(
            &mut game,
            CastSpellCommand::new(PlayerId::new("player-2"), creature_id.clone()),
        )
        .unwrap();
    let _ = resolve_current_stack(&service, &mut game);
    advance_to_player_first_main_satisfying_cleanup(&service, &mut game, "player-1");

    let outcome = service
        .cast_spell(
            &mut game,
            CastSpellCommand::new(PlayerId::new("player-1"), spell_id)
                .with_target(SpellTarget::Creature(creature_id.clone())),
        )
        .unwrap();

    assert_eq!(
        outcome.spell_put_on_stack.target,
        Some(SpellTarget::Creature(creature_id))
    );
}

#[test]
fn targeted_opponents_creature_spell_rejects_a_controlled_creature_when_cast() {
    let (service, mut game) = setup_two_player_game(
        "game-target-opponents-creature-invalid",
        filled_library(
            vec![
                creature_card("alice-bear", 0, 2, 2),
                targeted_opponents_creature_damage_instant_card("hostile-bolt", 0, 2),
            ],
            10,
        ),
        filled_library(Vec::new(), 10),
    );

    advance_to_player_first_main_satisfying_cleanup(&service, &mut game, "player-1");
    let creature_id = hand_card_id_by_definition(&game, 0, "alice-bear");
    service
        .cast_spell(
            &mut game,
            CastSpellCommand::new(PlayerId::new("player-1"), creature_id.clone()),
        )
        .unwrap();
    let _ = resolve_current_stack(&service, &mut game);
    advance_to_player_first_main_satisfying_cleanup(&service, &mut game, "player-1");
    let spell_id = hand_card_id_by_definition(&game, 0, "hostile-bolt");

    let result = service.cast_spell(
        &mut game,
        CastSpellCommand::new(PlayerId::new("player-1"), spell_id.clone())
            .with_target(SpellTarget::Creature(creature_id)),
    );

    assert!(matches!(
        result,
        Err(DomainError::Game(GameError::IllegalSpellTarget(card_id))) if card_id == spell_id
    ));
}

#[test]
fn targeted_opponents_creature_spell_can_target_the_opponents_creature_when_it_resolves() {
    let (service, mut game) = setup_two_player_game(
        "game-target-opponents-creature-resolve",
        filled_library(
            vec![
                land_card("alice-setup-land"),
                targeted_opponents_creature_damage_instant_card("hostile-bolt", 0, 2),
            ],
            10,
        ),
        filled_library(vec![creature_card("bob-bear", 0, 2, 2)], 10),
    );

    advance_to_player_first_main_satisfying_cleanup(&service, &mut game, "player-2");
    let creature_id = CardInstanceId::new("game-target-opponents-creature-resolve-player-2-0");
    service
        .cast_spell(
            &mut game,
            CastSpellCommand::new(PlayerId::new("player-2"), creature_id.clone()),
        )
        .unwrap();
    let _ = resolve_current_stack(&service, &mut game);
    advance_to_player_first_main_satisfying_cleanup(&service, &mut game, "player-1");
    let spell_id = hand_card_id_by_definition(&game, 0, "hostile-bolt");

    service
        .cast_spell(
            &mut game,
            CastSpellCommand::new(PlayerId::new("player-1"), spell_id)
                .with_target(SpellTarget::Creature(creature_id.clone())),
        )
        .unwrap();

    let resolution = resolve_current_stack(&service, &mut game);
    assert!(resolution.life_changed.is_none());
    assert_eq!(resolution.creatures_died.len(), 1);
    assert_eq!(resolution.creatures_died[0].card_id, creature_id);
    assert!(game.players()[1]
        .battlefield_cards()
        .all(|card| card.definition_id() != &CardDefinitionId::new("bob-bear")));
    assert_eq!(game.players()[1].graveyard_size(), 1);
}

#[test]
fn destroy_target_creature_spell_can_destroy_a_creature_when_it_resolves() {
    let (service, mut game) = setup_two_player_game(
        "game-destroy-target-creature-resolve",
        filled_library(
            vec![
                land_card("alice-setup-land"),
                targeted_destroy_creature_instant_card("murder-lite", 0),
            ],
            10,
        ),
        filled_library(vec![creature_card("bob-bear", 0, 2, 2)], 10),
    );

    advance_to_player_first_main_satisfying_cleanup(&service, &mut game, "player-2");
    let creature_id = CardInstanceId::new("game-destroy-target-creature-resolve-player-2-0");
    service
        .cast_spell(
            &mut game,
            CastSpellCommand::new(PlayerId::new("player-2"), creature_id.clone()),
        )
        .unwrap();
    let _ = resolve_current_stack(&service, &mut game);
    advance_to_player_first_main_satisfying_cleanup(&service, &mut game, "player-1");
    let spell_id = hand_card_id_by_definition(&game, 0, "murder-lite");

    service
        .cast_spell(
            &mut game,
            CastSpellCommand::new(PlayerId::new("player-1"), spell_id)
                .with_target(SpellTarget::Creature(creature_id.clone())),
        )
        .unwrap();

    let resolution = resolve_current_stack(&service, &mut game);
    assert!(resolution.life_changed.is_none());
    assert_eq!(resolution.creatures_died.len(), 1);
    assert_eq!(resolution.creatures_died[0].card_id, creature_id);
    assert!(game.players()[1]
        .battlefield_cards()
        .all(|card| card.definition_id() != &CardDefinitionId::new("bob-bear")));
    assert_eq!(game.players()[1].graveyard_size(), 1);
}

#[test]
fn destroy_target_creature_spell_does_not_apply_if_the_target_is_gone_on_resolution() {
    let (service, mut game) = setup_two_player_game(
        "game-destroy-target-creature-gone",
        filled_library(
            vec![
                land_card("mountain"),
                targeted_destroy_creature_instant_card("murder-lite", 0),
                targeted_damage_instant_card("shock", 0, 2),
            ],
            10,
        ),
        filled_library(vec![creature_card("bob-bear", 0, 2, 2)], 10),
    );

    advance_to_player_first_main_satisfying_cleanup(&service, &mut game, "player-2");
    let creature_id = CardInstanceId::new("game-destroy-target-creature-gone-player-2-0");
    service
        .cast_spell(
            &mut game,
            CastSpellCommand::new(PlayerId::new("player-2"), creature_id.clone()),
        )
        .unwrap();
    let _ = resolve_current_stack(&service, &mut game);
    advance_to_player_first_main_satisfying_cleanup(&service, &mut game, "player-1");

    let destroy_id = hand_card_id_by_definition(&game, 0, "murder-lite");
    service
        .cast_spell(
            &mut game,
            CastSpellCommand::new(PlayerId::new("player-1"), destroy_id)
                .with_target(SpellTarget::Creature(creature_id.clone())),
        )
        .unwrap();

    let shock_id = hand_card_id_by_definition(&game, 0, "shock");
    service
        .cast_spell(
            &mut game,
            CastSpellCommand::new(PlayerId::new("player-1"), shock_id)
                .with_target(SpellTarget::Creature(creature_id.clone())),
        )
        .unwrap();

    let first_resolution = resolve_current_stack(&service, &mut game);
    assert_eq!(first_resolution.creatures_died.len(), 1);
    assert_eq!(first_resolution.creatures_died[0].card_id, creature_id);

    let second_resolution = resolve_current_stack(&service, &mut game);
    assert!(second_resolution.life_changed.is_none());
    assert!(second_resolution.creatures_died.is_empty());
    assert_eq!(game.players()[1].graveyard_size(), 1);
}

#[test]
fn exile_target_creature_spell_can_exile_a_creature_when_it_resolves() {
    let (service, mut game) = setup_two_player_game(
        "game-exile-target-creature-resolve",
        filled_library(
            vec![
                land_card("alice-setup-land"),
                targeted_exile_creature_instant_card("banish-lite", 0),
            ],
            10,
        ),
        filled_library(vec![creature_card("bob-bear", 0, 2, 2)], 10),
    );

    advance_to_player_first_main_satisfying_cleanup(&service, &mut game, "player-2");
    let creature_id = CardInstanceId::new("game-exile-target-creature-resolve-player-2-0");
    service
        .cast_spell(
            &mut game,
            CastSpellCommand::new(PlayerId::new("player-2"), creature_id.clone()),
        )
        .unwrap();
    let _ = resolve_current_stack(&service, &mut game);
    advance_to_player_first_main_satisfying_cleanup(&service, &mut game, "player-1");
    let spell_id = hand_card_id_by_definition(&game, 0, "banish-lite");

    service
        .cast_spell(
            &mut game,
            CastSpellCommand::new(PlayerId::new("player-1"), spell_id)
                .with_target(SpellTarget::Creature(creature_id.clone())),
        )
        .unwrap();

    let resolution = resolve_current_stack(&service, &mut game);
    assert!(resolution.life_changed.is_none());
    assert!(resolution.creatures_died.is_empty());
    assert_eq!(
        resolution.card_exiled.as_ref().unwrap().card_id,
        creature_id
    );
    assert!(game.players()[1]
        .battlefield_cards()
        .all(|card| card.definition_id() != &CardDefinitionId::new("bob-bear")));
    assert_eq!(game.players()[1].graveyard_size(), 0);
    assert_eq!(game.players()[1].exile_size(), 1);
}

#[test]
fn exile_target_creature_spell_does_not_apply_if_the_target_is_gone_on_resolution() {
    let (service, mut game) = setup_two_player_game(
        "game-exile-target-creature-gone",
        filled_library(
            vec![
                land_card("mountain"),
                targeted_exile_creature_instant_card("banish-lite", 0),
                targeted_damage_instant_card("shock", 0, 2),
            ],
            10,
        ),
        filled_library(vec![creature_card("bob-bear", 0, 2, 2)], 10),
    );

    advance_to_player_first_main_satisfying_cleanup(&service, &mut game, "player-2");
    let creature_id = CardInstanceId::new("game-exile-target-creature-gone-player-2-0");
    service
        .cast_spell(
            &mut game,
            CastSpellCommand::new(PlayerId::new("player-2"), creature_id.clone()),
        )
        .unwrap();
    let _ = resolve_current_stack(&service, &mut game);
    advance_to_player_first_main_satisfying_cleanup(&service, &mut game, "player-1");

    let exile_id = hand_card_id_by_definition(&game, 0, "banish-lite");
    service
        .cast_spell(
            &mut game,
            CastSpellCommand::new(PlayerId::new("player-1"), exile_id)
                .with_target(SpellTarget::Creature(creature_id.clone())),
        )
        .unwrap();

    let shock_id = hand_card_id_by_definition(&game, 0, "shock");
    service
        .cast_spell(
            &mut game,
            CastSpellCommand::new(PlayerId::new("player-1"), shock_id)
                .with_target(SpellTarget::Creature(creature_id.clone())),
        )
        .unwrap();

    let first_resolution = resolve_current_stack(&service, &mut game);
    assert_eq!(first_resolution.creatures_died.len(), 1);
    assert_eq!(first_resolution.creatures_died[0].card_id, creature_id);

    let second_resolution = resolve_current_stack(&service, &mut game);
    assert!(second_resolution.card_exiled.is_none());
    assert!(second_resolution.life_changed.is_none());
    assert!(second_resolution.creatures_died.is_empty());
    assert_eq!(game.players()[1].graveyard_size(), 1);
    assert_eq!(game.players()[1].exile_size(), 0);
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
    assert!(game.players()[1].battlefield_is_empty());
    assert_eq!(game.players()[1].graveyard_size(), 1);
}
