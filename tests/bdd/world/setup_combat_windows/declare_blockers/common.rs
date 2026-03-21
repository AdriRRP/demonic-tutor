use super::super::super::{support, GameplayWorld};
use demonictutor::{CardDefinitionId, CastSpellCommand, LibraryCard};

pub(super) fn prepare_priority_after_blockers_declared(
    world: &mut GameplayWorld,
    game_id: &str,
    alice_cards: Vec<LibraryCard>,
    bob_cards: Vec<LibraryCard>,
) -> (demonictutor::CardInstanceId, demonictutor::CardInstanceId) {
    world.reset_game_with_libraries(
        game_id,
        support::filled_library(alice_cards, 10),
        support::filled_library(bob_cards, 10),
    );

    let service = support::create_service();
    support::advance_to_player_first_main_satisfying_cleanup(
        &service,
        world.game_mut(),
        "player-1",
    );
    let attacker_id = world.hand_card_by_definition("Alice", "bdd-attacker-priority");
    service
        .cast_spell(
            world.game_mut(),
            CastSpellCommand::new(GameplayWorld::player_id("Alice"), attacker_id.clone()),
        )
        .expect("attacker cast should succeed");
    support::resolve_top_stack_with_passes(&service, world.game_mut());

    support::advance_to_player_first_main_satisfying_cleanup(
        &service,
        world.game_mut(),
        "player-2",
    );
    let blocker_id = world.hand_card_by_definition("Bob", "bdd-blocker-priority");
    service
        .cast_spell(
            world.game_mut(),
            CastSpellCommand::new(GameplayWorld::player_id("Bob"), blocker_id.clone()),
        )
        .expect("blocker cast should succeed");
    support::resolve_top_stack_with_passes(&service, world.game_mut());

    support::advance_to_player_first_main_satisfying_cleanup(
        &service,
        world.game_mut(),
        "player-1",
    );
    support::advance_turn_raw(&service, world.game_mut());
    support::close_empty_priority_window(&service, world.game_mut());
    support::advance_turn_raw(&service, world.game_mut());
    service
        .declare_attackers(
            world.game_mut(),
            demonictutor::DeclareAttackersCommand::new(
                GameplayWorld::player_id("Alice"),
                vec![attacker_id.clone()],
            ),
        )
        .expect("declare attackers should succeed");
    support::close_empty_priority_window(&service, world.game_mut());
    service
        .declare_blockers(
            world.game_mut(),
            demonictutor::DeclareBlockersCommand::new(
                GameplayWorld::player_id("Bob"),
                vec![(blocker_id.clone(), attacker_id.clone())],
            ),
        )
        .expect("declare blockers should succeed");

    world.tracked_attacker_id = Some(attacker_id.clone());
    world.tracked_blocker_id = Some(blocker_id.clone());
    world.blocker_assignments = vec![(blocker_id.clone(), attacker_id.clone())];

    (attacker_id, blocker_id)
}

pub(super) fn attacker_card() -> LibraryCard {
    LibraryCard::creature(CardDefinitionId::new("bdd-attacker-priority"), 0, 2, 2)
}

pub(super) fn blocker_card() -> LibraryCard {
    LibraryCard::creature(CardDefinitionId::new("bdd-blocker-priority"), 0, 2, 2)
}
