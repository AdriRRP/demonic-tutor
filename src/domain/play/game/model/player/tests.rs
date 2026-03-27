//! Supports local regression tests for the player model.

use {
    super::{ManaCost, Player, PlayerCardZone, PrepareHandSpellCastError},
    crate::domain::play::cards::CardInstance,
    crate::domain::play::ids::{CardDefinitionId, CardInstanceId, PlayerId},
};

#[test]
fn prepare_hand_spell_cast_rolls_back_arena_state_when_hand_and_arena_are_desynchronized() {
    let player_id = PlayerId::new("player-a");
    let card_id = CardInstanceId::new("card-a");
    let mut player = Player::new(player_id);
    player.receive_hand_cards(vec![CardInstance::new(
        card_id.clone(),
        CardDefinitionId::new("definition-a"),
        crate::domain::play::cards::CardType::Instant,
        1,
    )]);
    player.add_mana(1);

    let handle = player.handle_in_zone(&card_id, PlayerCardZone::Hand);
    assert!(handle.is_some());
    let Some(handle) = handle else { return };
    let removed_handle = player.hand.remove(handle);
    assert_eq!(removed_handle, Some(handle));

    let result = player.prepare_hand_spell_cast(&card_id, 1, ManaCost::generic(1));

    assert_eq!(result, Err(PrepareHandSpellCastError::MissingCard));
    assert_eq!(player.mana(), 1);
    assert_eq!(player.card_zone(&card_id), Some(PlayerCardZone::Hand));
    assert!(player.hand_card(&card_id).is_some());
}

#[test]
fn move_handle_between_zones_rolls_back_visible_zone_when_arena_update_fails() {
    let player_id = PlayerId::new("player-a");
    let card_id = CardInstanceId::new("card-a");
    let mut player = Player::new(player_id);
    assert!(player
        .receive_battlefield_card(CardInstance::new(
            card_id.clone(),
            CardDefinitionId::new("definition-a"),
            crate::domain::play::cards::CardType::Creature,
            2,
        ))
        .is_some());

    let handle = player.handle_in_zone(&card_id, PlayerCardZone::Battlefield);
    assert!(handle.is_some());
    let Some(handle) = handle else { return };
    let removed = player.cards.begin_remove_by_handle(handle);
    assert!(removed.is_some());

    let moved = player.move_handle_between_zones(
        handle,
        PlayerCardZone::Battlefield,
        PlayerCardZone::Graveyard,
    );

    assert_eq!(moved, None);
    assert!(player.battlefield.contains(handle));
    assert!(!player.graveyard.contains(handle));
}
