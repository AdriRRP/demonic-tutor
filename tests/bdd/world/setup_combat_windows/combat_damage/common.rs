use super::super::super::{support, GameplayWorld};
use demonictutor::{CardDefinitionId, CastSpellCommand, LibraryCard, ResolveCombatDamageCommand};

pub(super) fn prepare_priority_after_combat_damage(
    world: &mut GameplayWorld,
    game_id: &str,
    alice_cards: Vec<LibraryCard>,
    bob_cards: Vec<LibraryCard>,
) -> demonictutor::CardInstanceId {
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
    let attacker_id = world.hand_card_by_definition("Alice", "bdd-attacker-unblocked");
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
    support::advance_turn_raw(&service, world.game_mut());
    service
        .resolve_combat_damage(
            world.game_mut(),
            ResolveCombatDamageCommand::new(GameplayWorld::player_id("Alice")),
        )
        .expect("combat damage should resolve");

    world.tracked_attacker_id = Some(attacker_id.clone());

    attacker_id
}

pub(super) fn attacker_card() -> LibraryCard {
    LibraryCard::creature(CardDefinitionId::new("bdd-attacker-unblocked"), 0, 3, 3)
}
