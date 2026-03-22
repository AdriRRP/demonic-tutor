#![allow(clippy::unwrap_used)]

use crate::support::{
    advance_to_player_first_main_satisfying_cleanup, close_empty_priority_window, filled_library,
    resolve_top_stack_with_passes, setup_two_player_game,
    targeted_controlled_attacking_creature_damage_instant_card,
};
use demonictutor::{
    CardDefinitionId, CardInstanceId, CastSpellCommand, DeclareAttackersCommand, LibraryCard,
    Phase, PlayerId, SpellTarget,
};

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
fn controlled_attacking_creature_spell_marks_nonlethal_damage_and_leaves_the_attacker_in_combat() {
    let (service, mut game) = setup_two_player_game(
        "game-target-controlled-attacking-nonlethal",
        filled_library(
            vec![
                LibraryCard::creature(CardDefinitionId::new("attacker"), 0, 2, 3),
                targeted_controlled_attacking_creature_damage_instant_card("rally-shot", 0, 1),
            ],
            10,
        ),
        filled_library(Vec::new(), 10),
    );

    let spell_id = hand_card_id_by_definition(&game, 0, "rally-shot");

    advance_to_player_first_main_satisfying_cleanup(&service, &mut game, "player-1");
    let attacker_id = CardInstanceId::new("game-target-controlled-attacking-nonlethal-player-1-0");
    service
        .cast_spell(
            &mut game,
            CastSpellCommand::new(PlayerId::new("player-1"), attacker_id.clone()),
        )
        .unwrap();
    resolve_top_stack_with_passes(&service, &mut game);

    advance_to_player_first_main_satisfying_cleanup(&service, &mut game, "player-2");
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

    assert_eq!(game.phase(), &Phase::DeclareBlockers);

    service
        .cast_spell(
            &mut game,
            CastSpellCommand::new(PlayerId::new("player-1"), spell_id)
                .with_target(SpellTarget::Creature(attacker_id.clone())),
        )
        .unwrap();

    resolve_top_stack_with_passes(&service, &mut game);

    let attacker = game.players()[0]
        .battlefield_cards()
        .find(|card| card.id() == &attacker_id)
        .unwrap();
    assert_eq!(attacker.damage(), 1);
    assert!(attacker.is_attacking());
}
