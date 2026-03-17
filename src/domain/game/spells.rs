use super::player::Player;
use super::Phase;
use crate::domain::{
    commands::CastSpellCommand,
    errors::{CardError, DomainError, GameError},
    events::SpellCast,
    ids::{GameId, PlayerId},
};

/// Casts a non-creature spell from hand to battlefield.
///
/// # Errors
/// Returns an error if:
/// - The player is not the active player
/// - The phase is not `FirstMain` or `SecondMain`
/// - The card is not in the player's hand
/// - The card is a land (cannot be cast)
/// - The player has insufficient mana
pub fn cast_spell(
    game_id: &GameId,
    players: &mut [Player],
    active_player: &PlayerId,
    phase: &Phase,
    cmd: CastSpellCommand,
) -> Result<SpellCast, DomainError> {
    if *active_player != cmd.player_id {
        return Err(DomainError::Game(GameError::NotYourTurn {
            current: active_player.clone(),
            requested: cmd.player_id,
        }));
    }

    if !matches!(phase, Phase::FirstMain | Phase::SecondMain) {
        return Err(DomainError::Phase(
            super::PhaseError::InvalidForPlayingCard { phase: *phase },
        ));
    }

    let player_idx = players
        .iter()
        .position(|p| p.id() == &cmd.player_id)
        .ok_or_else(|| DomainError::Game(GameError::PlayerNotFound(cmd.player_id.clone())))?;

    let player = &mut players[player_idx];

    let card_id = cmd.card_id.clone();

    // Validate card type before removing from hand to avoid losing the card
    // if the type check fails.
    let card_type = player
        .hand()
        .cards()
        .iter()
        .find(|c| c.id() == &card_id)
        .map(|c| c.card_type().clone())
        .ok_or_else(|| {
            DomainError::Card(CardError::NotInHand {
                player: cmd.player_id.clone(),
                card: card_id.clone(),
            })
        })?;

    if card_type.is_land() {
        return Err(DomainError::Card(CardError::CannotCastLand(card_id)));
    }

    let card = player.hand_mut().remove(&card_id).ok_or_else(|| {
        DomainError::Card(CardError::NotInHand {
            player: cmd.player_id.clone(),
            card: card_id.clone(),
        })
    })?;

    let mana_cost = card.mana_cost();
    if !player.spend_mana(mana_cost) {
        return Err(DomainError::Game(GameError::InsufficientMana {
            player: cmd.player_id,
            required: mana_cost,
            available: player.mana(),
        }));
    }

    player.battlefield_mut().add(card);

    Ok(SpellCast::new(game_id.clone(), cmd.player_id, card_id))
}
