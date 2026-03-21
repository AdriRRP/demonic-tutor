#![allow(clippy::unwrap_used)]

use crate::support::{
    advance_to_player_first_main_satisfying_cleanup, close_empty_priority_window, filled_library,
    resolve_top_stack_with_passes, setup_two_player_game,
    targeted_controlled_blocking_creature_damage_instant_card,
    targeted_opponents_attacking_creature_damage_instant_card,
};
use demonictutor::{
    CardDefinitionId, CardInstanceId, CastSpellCommand, DeclareAttackersCommand,
    DeclareBlockersCommand, DomainError, GameError, LibraryCard, PlayerId, SpellTarget,
};

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
fn controlled_blocking_creature_spell_rejects_an_opponents_attacker() {
    let (service, mut game) = setup_two_player_game(
        "game-target-controlled-blocking-illegal",
        filled_library(
            vec![LibraryCard::creature(
                CardDefinitionId::new("attacker"),
                0,
                2,
                2,
            )],
            10,
        ),
        filled_library(
            vec![
                LibraryCard::creature(CardDefinitionId::new("blocker"), 0, 2, 2),
                targeted_controlled_blocking_creature_damage_instant_card("shield-snap", 0, 2),
            ],
            10,
        ),
    );

    let spell_id = hand_card_id_by_definition(&game, 1, "shield-snap");

    advance_to_player_first_main_satisfying_cleanup(&service, &mut game, "player-1");
    let attacker_id = CardInstanceId::new("game-target-controlled-blocking-illegal-player-1-0");
    service
        .cast_spell(
            &mut game,
            CastSpellCommand::new(PlayerId::new("player-1"), attacker_id.clone()),
        )
        .unwrap();
    resolve_top_stack_with_passes(&service, &mut game);

    advance_to_player_first_main_satisfying_cleanup(&service, &mut game, "player-2");
    let blocker_id = CardInstanceId::new("game-target-controlled-blocking-illegal-player-2-0");
    service
        .cast_spell(
            &mut game,
            CastSpellCommand::new(PlayerId::new("player-2"), blocker_id.clone()),
        )
        .unwrap();
    resolve_top_stack_with_passes(&service, &mut game);

    advance_to_player_first_main_satisfying_cleanup(&service, &mut game, "player-1");
    crate::support::advance_turn_raw(&service, &mut game);
    close_empty_priority_window(&service, &mut game);
    crate::support::advance_turn_raw(&service, &mut game);
    service
        .declare_attackers(
            &mut game,
            DeclareAttackersCommand::new(PlayerId::new("player-1"), vec![attacker_id.clone()]),
        )
        .unwrap();
    close_empty_priority_window(&service, &mut game);
    service
        .declare_blockers(
            &mut game,
            DeclareBlockersCommand::new(
                PlayerId::new("player-2"),
                vec![(blocker_id, attacker_id.clone())],
            ),
        )
        .unwrap();
    service
        .pass_priority(
            &mut game,
            demonictutor::PassPriorityCommand::new(PlayerId::new("player-1")),
        )
        .unwrap();

    let result = service.cast_spell(
        &mut game,
        CastSpellCommand::new(PlayerId::new("player-2"), spell_id.clone())
            .with_target(SpellTarget::Creature(attacker_id)),
    );

    assert!(matches!(
        result,
        Err(DomainError::Game(GameError::IllegalSpellTarget(card_id)))
            if card_id == spell_id
    ));
}

#[test]
fn opponents_attacking_creature_spell_rejects_the_casters_own_blocker() {
    let (service, mut game) = setup_two_player_game(
        "game-target-opponent-attacker-illegal",
        filled_library(
            vec![LibraryCard::creature(
                CardDefinitionId::new("attacker"),
                0,
                2,
                2,
            )],
            10,
        ),
        filled_library(
            vec![
                LibraryCard::creature(CardDefinitionId::new("blocker"), 0, 2, 2),
                targeted_opponents_attacking_creature_damage_instant_card("punish-charge", 0, 2),
            ],
            10,
        ),
    );

    let spell_id = hand_card_id_by_definition(&game, 1, "punish-charge");

    advance_to_player_first_main_satisfying_cleanup(&service, &mut game, "player-1");
    let attacker_id = CardInstanceId::new("game-target-opponent-attacker-illegal-player-1-0");
    service
        .cast_spell(
            &mut game,
            CastSpellCommand::new(PlayerId::new("player-1"), attacker_id.clone()),
        )
        .unwrap();
    resolve_top_stack_with_passes(&service, &mut game);

    advance_to_player_first_main_satisfying_cleanup(&service, &mut game, "player-2");
    let blocker_id = CardInstanceId::new("game-target-opponent-attacker-illegal-player-2-0");
    service
        .cast_spell(
            &mut game,
            CastSpellCommand::new(PlayerId::new("player-2"), blocker_id.clone()),
        )
        .unwrap();
    resolve_top_stack_with_passes(&service, &mut game);

    advance_to_player_first_main_satisfying_cleanup(&service, &mut game, "player-1");
    crate::support::advance_turn_raw(&service, &mut game);
    close_empty_priority_window(&service, &mut game);
    crate::support::advance_turn_raw(&service, &mut game);
    service
        .declare_attackers(
            &mut game,
            DeclareAttackersCommand::new(PlayerId::new("player-1"), vec![attacker_id.clone()]),
        )
        .unwrap();
    close_empty_priority_window(&service, &mut game);
    service
        .declare_blockers(
            &mut game,
            DeclareBlockersCommand::new(
                PlayerId::new("player-2"),
                vec![(blocker_id.clone(), attacker_id)],
            ),
        )
        .unwrap();
    service
        .pass_priority(
            &mut game,
            demonictutor::PassPriorityCommand::new(PlayerId::new("player-1")),
        )
        .unwrap();

    let result = service.cast_spell(
        &mut game,
        CastSpellCommand::new(PlayerId::new("player-2"), spell_id.clone())
            .with_target(SpellTarget::Creature(blocker_id)),
    );

    assert!(matches!(
        result,
        Err(DomainError::Game(GameError::IllegalSpellTarget(card_id)))
            if card_id == spell_id
    ));
}
