//! Supports stack priority resolution destination.

use crate::domain::play::{
    cards::{CardType, SpellPayload},
    errors::{DomainError, GameError},
    events::SpellCastOutcome,
    game::Player,
};

pub(super) fn move_resolved_spell_to_its_destination(
    players: &mut [Player],
    controller_index: usize,
    card_type: CardType,
    payload: SpellPayload,
) -> Result<SpellCastOutcome, DomainError> {
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
            Ok(SpellCastOutcome::EnteredBattlefield)
        }
        CardType::Instant | CardType::Sorcery => {
            if payload.exile_on_resolution() {
                player.receive_exile_card(payload.into_card_instance());
                return Ok(SpellCastOutcome::ResolvedToExile);
            }
            player.receive_graveyard_card(payload.into_card_instance());
            Ok(SpellCastOutcome::ResolvedToGraveyard)
        }
        CardType::Land => Err(DomainError::Game(GameError::InternalInvariantViolation(
            "land cards cannot resolve from the stack as spells".to_string(),
        ))),
    }
}
