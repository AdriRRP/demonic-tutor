//! Supports stack priority resolution destination.

use crate::domain::play::{
    cards::{CardType, SpellPayload},
    errors::{DomainError, GameError},
    events::{CardExiled, SpellCastOutcome, ZoneType},
    game::Player,
};

pub(super) fn move_resolved_spell_to_its_destination(
    game_id: &crate::domain::play::ids::GameId,
    players: &mut [Player],
    controller_index: usize,
    card_type: CardType,
    payload: SpellPayload,
) -> Result<(SpellCastOutcome, Option<CardExiled>), DomainError> {
    let player = players.get_mut(controller_index).ok_or_else(|| {
        DomainError::Game(GameError::InternalInvariantViolation(format!(
            "missing spell controller at player index {controller_index}"
        )))
    })?;

    match card_type {
        CardType::Creature
        | CardType::Enchantment
        | CardType::Artifact
        | CardType::Planeswalker => {
            player
                .receive_battlefield_card(payload.into_card_instance())
                .ok_or_else(|| {
                    DomainError::Game(GameError::InternalInvariantViolation(
                        "failed to move resolved permanent spell to the battlefield".to_string(),
                    ))
                })?;
            Ok((SpellCastOutcome::EnteredBattlefield, None))
        }
        CardType::Instant | CardType::Sorcery => {
            let card_id = payload.id().clone();
            if payload.exile_on_resolution() {
                player.receive_exile_card(payload.into_card_instance());
                return Ok((
                    SpellCastOutcome::ResolvedToExile,
                    Some(CardExiled::new(
                        game_id.clone(),
                        player.id().clone(),
                        card_id,
                        ZoneType::Stack,
                    )),
                ));
            }
            player.receive_graveyard_card(payload.into_card_instance());
            Ok((SpellCastOutcome::ResolvedToGraveyard, None))
        }
        CardType::Land => Err(DomainError::Game(GameError::InternalInvariantViolation(
            "land cards cannot resolve from the stack as spells".to_string(),
        ))),
    }
}
