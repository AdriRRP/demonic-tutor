#![allow(clippy::expect_used)]

//! Unit coverage for unit combat aura restrictions.

use {
    crate::support,
    demonictutor::{
        CardDefinitionId, CardError, CardInstance, CastSpellCommand, DeclareAttackersCommand,
        DeclareBlockersCommand, DomainError, PlayerId, SpellTarget,
    },
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
    support::resolve_top_stack_with_passes(service, game);
}

#[test]
fn pacifism_style_aura_prevents_the_enchanted_creature_from_attacking() {
    let (service, mut game) = support::setup_two_player_game(
        "game-pacifism-attack",
        support::filled_library(
            vec![
                support::creature_card("attacker", 0, 2, 2),
                support::pacifism_creature_aura_enchantment_card("pacifism-lite", 0),
            ],
            10,
        ),
        support::filled_library(vec![support::land_card("mountain")], 10),
    );

    support::advance_to_player_first_main_satisfying_cleanup(&service, &mut game, "player-1");
    let attacker_id = game.players()[0]
        .hand_card_by_definition(&CardDefinitionId::new("attacker"))
        .expect("attacker should be in hand")
        .id()
        .clone();
    let aura_id = game.players()[0]
        .hand_card_by_definition(&CardDefinitionId::new("pacifism-lite"))
        .expect("aura should be in hand")
        .id()
        .clone();
    cast_and_resolve(&service, &mut game, "player-1", attacker_id.clone());

    service
        .cast_spell(
            &mut game,
            CastSpellCommand::new(PlayerId::new("player-1"), aura_id)
                .with_target(SpellTarget::Creature(attacker_id.clone())),
        )
        .expect("aura should cast");
    support::resolve_top_stack_with_passes(&service, &mut game);

    support::advance_to_player_first_main_satisfying_cleanup(&service, &mut game, "player-2");
    support::advance_to_player_first_main_satisfying_cleanup(&service, &mut game, "player-1");

    support::advance_to_phase_satisfying_cleanup(
        &service,
        &mut game,
        demonictutor::Phase::DeclareAttackers,
    );

    let result = service.declare_attackers(
        &mut game,
        DeclareAttackersCommand::new(PlayerId::new("player-1"), vec![attacker_id.clone()]),
    );

    assert!(matches!(
        result,
        Err(DomainError::Card(CardError::CannotAttack { card, .. })) if card == attacker_id
    ));
}

#[test]
fn pacifism_style_aura_prevents_the_enchanted_creature_from_blocking() {
    let (service, mut game) = support::setup_two_player_game(
        "game-pacifism-block",
        support::filled_library(vec![support::creature_card("attacker", 0, 2, 2)], 10),
        support::filled_library(
            vec![
                support::creature_card("blocker", 0, 2, 2),
                support::pacifism_creature_aura_enchantment_card("pacifism-lite", 0),
            ],
            10,
        ),
    );

    support::advance_to_player_first_main_satisfying_cleanup(&service, &mut game, "player-1");
    let attacker_id = game.players()[0]
        .hand_card_by_definition(&CardDefinitionId::new("attacker"))
        .expect("attacker should be in hand")
        .id()
        .clone();
    cast_and_resolve(&service, &mut game, "player-1", attacker_id.clone());

    support::advance_to_player_first_main_satisfying_cleanup(&service, &mut game, "player-2");
    let blocker_id = game.players()[1]
        .hand_card_by_definition(&CardDefinitionId::new("blocker"))
        .expect("blocker should be in hand")
        .id()
        .clone();
    let aura_id = game.players()[1]
        .hand_card_by_definition(&CardDefinitionId::new("pacifism-lite"))
        .expect("aura should be in hand")
        .id()
        .clone();
    cast_and_resolve(&service, &mut game, "player-2", blocker_id.clone());

    service
        .cast_spell(
            &mut game,
            CastSpellCommand::new(PlayerId::new("player-2"), aura_id)
                .with_target(SpellTarget::Creature(blocker_id.clone())),
        )
        .expect("aura should cast");
    support::resolve_top_stack_with_passes(&service, &mut game);

    support::advance_to_player_first_main_satisfying_cleanup(&service, &mut game, "player-1");
    support::advance_to_phase_satisfying_cleanup(
        &service,
        &mut game,
        demonictutor::Phase::DeclareAttackers,
    );
    service
        .declare_attackers(
            &mut game,
            DeclareAttackersCommand::new(PlayerId::new("player-1"), vec![attacker_id.clone()]),
        )
        .expect("attacker should be declared");
    support::close_empty_priority_window(&service, &mut game);

    let result = service.declare_blockers(
        &mut game,
        DeclareBlockersCommand::new(
            PlayerId::new("player-2"),
            vec![(blocker_id.clone(), attacker_id)],
        ),
    );

    assert!(matches!(
        result,
        Err(DomainError::Card(CardError::CannotBlock { card, .. })) if card == blocker_id
    ));
}

#[test]
fn pacifism_style_restriction_is_removed_when_the_aura_leaves_battlefield() {
    let (service, mut game) = support::setup_two_player_game(
        "game-pacifism-released",
        support::filled_library(
            vec![
                support::creature_card("attacker", 0, 2, 2),
                support::pacifism_creature_aura_enchantment_card("pacifism-lite", 0),
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

    support::advance_to_player_first_main_satisfying_cleanup(&service, &mut game, "player-1");
    let attacker_id = game.players()[0]
        .hand_card_by_definition(&CardDefinitionId::new("attacker"))
        .expect("attacker should be in hand")
        .id()
        .clone();
    let aura_id = game.players()[0]
        .hand_card_by_definition(&CardDefinitionId::new("pacifism-lite"))
        .expect("aura should be in hand")
        .id()
        .clone();
    cast_and_resolve(&service, &mut game, "player-1", attacker_id.clone());
    service
        .cast_spell(
            &mut game,
            CastSpellCommand::new(PlayerId::new("player-1"), aura_id.clone())
                .with_target(SpellTarget::Creature(attacker_id.clone())),
        )
        .expect("aura should cast");
    support::resolve_top_stack_with_passes(&service, &mut game);

    support::pass_priority(&service, &mut game, "player-1");
    let disenchant_id = game.players()[1]
        .hand_card_at(0)
        .expect("disenchant should be in hand")
        .id()
        .clone();
    service
        .cast_spell(
            &mut game,
            CastSpellCommand::new(PlayerId::new("player-2"), disenchant_id)
                .with_target(SpellTarget::Permanent(aura_id)),
        )
        .expect("disenchant should cast");
    support::resolve_top_stack_with_passes(&service, &mut game);

    support::advance_to_player_first_main_satisfying_cleanup(&service, &mut game, "player-2");
    support::advance_to_player_first_main_satisfying_cleanup(&service, &mut game, "player-1");

    support::advance_to_phase_satisfying_cleanup(
        &service,
        &mut game,
        demonictutor::Phase::DeclareAttackers,
    );
    let result = service.declare_attackers(
        &mut game,
        DeclareAttackersCommand::new(PlayerId::new("player-1"), vec![attacker_id.clone()]),
    );

    assert!(
        result.is_ok(),
        "attacker should attack after aura leaves: {result:?}"
    );
    let attacking = game.players()[0]
        .battlefield_card(&attacker_id)
        .map(CardInstance::is_attacking);
    assert_eq!(attacking, Some(true));
}
