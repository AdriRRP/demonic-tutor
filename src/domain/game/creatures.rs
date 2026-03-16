use super::player::Player;
use super::Phase;
use crate::domain::{
    cards::CardType,
    commands::PlayCreatureCommand,
    errors::{CardError, DomainError, GameError},
    events::CreatureEnteredBattlefield,
    ids::PlayerId,
};

/// Plays a creature card from hand to battlefield.
///
/// # Errors
/// Returns an error if:
/// - The player is not the active player
/// - The phase is not Main
/// - The card is not in the player's hand
/// - The card is not a creature
/// - The player has insufficient mana
pub fn play_creature(
    players: &mut [Player],
    active_player: &PlayerId,
    phase: &Phase,
    cmd: PlayCreatureCommand,
) -> Result<CreatureEnteredBattlefield, DomainError> {
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

    let card = player.hand_mut().remove(&card_id).ok_or_else(|| {
        DomainError::Card(CardError::NotInHand {
            player: cmd.player_id.clone(),
            card: card_id.clone(),
        })
    })?;

    if !matches!(card.card_type(), CardType::Creature) {
        return Err(DomainError::Card(CardError::NotACreature(card_id)));
    }

    let power = card.power().unwrap_or(0);
    let toughness = card.toughness().unwrap_or(0);

    let mana_cost = card.mana_cost();
    if player.mana() < mana_cost {
        return Err(DomainError::Game(GameError::InsufficientMana {
            player: cmd.player_id,
            required: mana_cost,
            available: player.mana(),
        }));
    }

    *player.mana_mut() -= mana_cost;

    player.battlefield_mut().add(card);

    Ok(CreatureEnteredBattlefield::new(
        super::Game::id_from_player_id(&cmd.player_id),
        cmd.player_id,
        card_id,
        power,
        toughness,
    ))
}
