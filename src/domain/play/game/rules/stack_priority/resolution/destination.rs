//! Supports stack priority resolution destination.

use crate::domain::play::{
    cards::{CardType, SpellPayload},
    errors::{DomainError, GameError},
    events::SpellCastOutcome,
    game::{helpers, Player},
    ids::PlayerId,
};

pub(super) fn move_resolved_spell_to_its_destination(
    players: &mut [Player],
    controller_id: &PlayerId,
    card_type: CardType,
    payload: SpellPayload,
) -> Result<SpellCastOutcome, DomainError> {
    let player = helpers::find_player_mut(players, controller_id)?;

    match card_type {
        CardType::Creature
        | CardType::Enchantment
        | CardType::Artifact
        | CardType::Planeswalker => {
            player.receive_battlefield_card(payload.into_card_instance());
            Ok(SpellCastOutcome::EnteredBattlefield)
        }
        CardType::Instant | CardType::Sorcery => {
            player.receive_graveyard_card(payload.into_card_instance());
            Ok(SpellCastOutcome::ResolvedToGraveyard)
        }
        CardType::Land => Err(DomainError::Game(GameError::InternalInvariantViolation(
            "land cards cannot resolve from the stack as spells".to_string(),
        ))),
    }
}
