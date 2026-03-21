#![allow(clippy::unwrap_used)]

use crate::support::{
    advance_to_player_first_main_satisfying_cleanup, close_empty_priority_window, filled_library,
    own_turn_priority_artifact_card, resolve_top_stack_with_passes, setup_two_player_game,
};
use demonictutor::{
    CardDefinitionId, CardInstanceId, CastSpellCommand, DeclareAttackersCommand, LibraryCard,
    Phase, PlayerId,
};

#[test]
fn active_player_can_cast_and_resolve_an_own_turn_priority_artifact_after_attackers() {
    let (service, mut game) = setup_two_player_game(
        "game-own-turn-artifact-after-attackers",
        filled_library(
            vec![
                LibraryCard::creature(CardDefinitionId::new("attacker"), 0, 2, 2),
                own_turn_priority_artifact_card("swift-relic", 0),
            ],
            10,
        ),
        filled_library(Vec::new(), 10),
    );

    advance_to_player_first_main_satisfying_cleanup(&service, &mut game, "player-1");
    let attacker_id = CardInstanceId::new("game-own-turn-artifact-after-attackers-player-1-0");
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
            DeclareAttackersCommand::new(PlayerId::new("player-1"), vec![attacker_id]),
        )
        .unwrap();

    assert_eq!(game.phase(), &Phase::DeclareBlockers);
    assert_eq!(
        game.priority().unwrap().current_holder(),
        &PlayerId::new("player-1")
    );

    service
        .cast_spell(
            &mut game,
            CastSpellCommand::new(
                PlayerId::new("player-1"),
                CardInstanceId::new("game-own-turn-artifact-after-attackers-player-1-1"),
            ),
        )
        .unwrap();

    assert_eq!(game.stack().len(), 1);
    resolve_top_stack_with_passes(&service, &mut game);
    assert_eq!(game.players()[0].battlefield().cards().len(), 2);
    assert_eq!(
        game.players()[0].battlefield().cards()[1]
            .definition_id()
            .as_str(),
        "swift-relic"
    );
}
