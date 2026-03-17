use super::super::{invariants, model::Player, TerminalState};
use crate::domain::play::{
    cards::CardType,
    commands::{AdjustLifeCommand, CastSpellCommand, PlayLandCommand, TapLandCommand},
    errors::{CardError, DomainError, GameError, PhaseError},
    events::{
        GameEndReason, GameEnded, LandPlayed, LandTapped, LifeChanged, ManaAdded, SpellCast,
        SpellCastOutcome,
    },
    ids::{GameId, PlayerId},
    phase::Phase,
};

#[derive(Debug, Clone)]
pub struct AdjustLifeOutcome {
    pub life_changed: LifeChanged,
    pub game_ended: Option<GameEnded>,
}

impl AdjustLifeOutcome {
    #[must_use]
    pub const fn new(life_changed: LifeChanged, game_ended: Option<GameEnded>) -> Self {
        Self {
            life_changed,
            game_ended,
        }
    }
}

/// Plays a land card from hand to battlefield.
///
/// # Errors
/// Returns an error if the action is invalid.
pub fn play_land(
    game_id: &GameId,
    players: &mut [Player],
    active_player: &PlayerId,
    phase: &Phase,
    cmd: PlayLandCommand,
) -> Result<LandPlayed, DomainError> {
    invariants::require_active_player(active_player, &cmd.player_id)?;

    if !matches!(phase, Phase::FirstMain | Phase::SecondMain) {
        return Err(DomainError::Phase(PhaseError::InvalidForLand));
    }

    let player = invariants::find_player_mut(players, &cmd.player_id)?;

    if player.lands_played_this_turn() > 0 {
        return Err(DomainError::Phase(PhaseError::AlreadyPlayedLandThisTurn(
            cmd.player_id.clone(),
        )));
    }

    let card_id = cmd.card_id.clone();
    let card_type = invariants::hand_card_type(player, &cmd.player_id, &card_id)?;

    if !card_type.is_land() {
        return Err(DomainError::Card(CardError::NotALand(card_id)));
    }

    let card = invariants::remove_card_from_hand(player, &cmd.player_id, &card_id)?;

    player.battlefield_mut().add(card);
    player.record_land_played();

    Ok(LandPlayed::new(game_id.clone(), cmd.player_id, card_id))
}

/// Taps a land to produce mana.
///
/// # Errors
/// Returns an error if the action is invalid.
pub fn tap_land(
    game_id: &GameId,
    players: &mut [Player],
    active_player: &PlayerId,
    phase: &Phase,
    cmd: TapLandCommand,
) -> Result<(LandTapped, ManaAdded), DomainError> {
    invariants::require_active_player(active_player, &cmd.player_id)?;

    if !matches!(phase, Phase::FirstMain | Phase::SecondMain) {
        return Err(DomainError::Phase(PhaseError::InvalidForPlayingCard {
            phase: *phase,
        }));
    }

    let player = invariants::find_player_mut(players, &cmd.player_id)?;

    let card = player
        .battlefield_mut()
        .card_mut(&cmd.card_id)
        .ok_or_else(|| {
            DomainError::Card(CardError::NotOnBattlefield {
                player: cmd.player_id.clone(),
                card: cmd.card_id.clone(),
            })
        })?;

    if card.is_tapped() {
        return Err(DomainError::Card(CardError::AlreadyTapped {
            player: cmd.player_id.clone(),
            card: cmd.card_id.clone(),
        }));
    }

    if !matches!(card.card_type(), CardType::Land) {
        return Err(DomainError::Card(CardError::NotALand(cmd.card_id.clone())));
    }

    card.tap();

    player.add_mana(1);
    let new_mana = player.mana();

    Ok((
        LandTapped::new(game_id.clone(), cmd.player_id.clone(), cmd.card_id.clone()),
        ManaAdded::new(game_id.clone(), cmd.player_id, 1, new_mana),
    ))
}

/// Adjusts a player's life total by a signed delta.
///
/// # Errors
/// Returns an error if the player is not found.
pub fn adjust_life(
    game_id: &GameId,
    players: &mut [Player],
    terminal_state: &mut TerminalState,
    cmd: AdjustLifeCommand,
) -> Result<AdjustLifeOutcome, DomainError> {
    let player = invariants::find_player_mut(players, &cmd.player_id)?;
    let old_life = player.life();
    player.adjust_life(cmd.life_delta);
    let new_life = player.life();

    let life_changed = LifeChanged::new(game_id.clone(), cmd.player_id.clone(), old_life, new_life);

    let game_ended = if new_life == 0 {
        let winner = invariants::opposing_player_id(players, &cmd.player_id)?;
        terminal_state.end(
            winner.clone(),
            cmd.player_id.clone(),
            GameEndReason::ZeroLife,
        );
        Some(GameEnded::new(
            game_id.clone(),
            winner,
            cmd.player_id,
            GameEndReason::ZeroLife,
        ))
    } else {
        None
    };

    Ok(AdjustLifeOutcome::new(life_changed, game_ended))
}

/// Casts a spell from hand.
///
/// # Errors
/// Returns an error if the action is invalid.
pub fn cast_spell(
    game_id: &GameId,
    players: &mut [Player],
    active_player: &PlayerId,
    phase: &Phase,
    cmd: CastSpellCommand,
) -> Result<SpellCast, DomainError> {
    invariants::require_active_player(active_player, &cmd.player_id)?;

    if !matches!(phase, Phase::FirstMain | Phase::SecondMain) {
        return Err(DomainError::Phase(PhaseError::InvalidForPlayingCard {
            phase: *phase,
        }));
    }

    let player = invariants::find_player_mut(players, &cmd.player_id)?;
    let card_id = cmd.card_id.clone();
    let card_type = invariants::hand_card_type(player, &cmd.player_id, &card_id)?;

    if card_type.is_land() {
        return Err(DomainError::Card(CardError::CannotCastLand(card_id)));
    }

    let hand_card = invariants::hand_card(player, &cmd.player_id, &card_id)?;
    let mana_cost = hand_card.mana_cost();
    if player.mana() < mana_cost {
        return Err(DomainError::Game(GameError::InsufficientMana {
            player: cmd.player_id.clone(),
            required: mana_cost,
            available: player.mana(),
        }));
    }
    let card = invariants::remove_card_from_hand(player, &cmd.player_id, &card_id)?;
    let spent = player.spend_mana(mana_cost);
    debug_assert!(spent, "mana was checked before removing the card from hand");

    let outcome = match card_type {
        CardType::Creature
        | CardType::Enchantment
        | CardType::Artifact
        | CardType::Planeswalker => {
            if matches!(card_type, CardType::Creature) && card.creature_stats().is_none() {
                return Err(DomainError::Game(GameError::InternalInvariantViolation(
                    format!("creature card {} must have power and toughness", card.id()),
                )));
            }
            player.battlefield_mut().add(card);
            SpellCastOutcome::EnteredBattlefield
        }
        CardType::Instant | CardType::Sorcery => {
            player.graveyard_mut().add(card);
            SpellCastOutcome::ResolvedToGraveyard
        }
        CardType::Land => {
            return Err(DomainError::Game(GameError::InternalInvariantViolation(
                "land cards cannot be cast as spells".to_string(),
            )))
        }
    };

    Ok(SpellCast::new(
        game_id.clone(),
        cmd.player_id,
        card_id,
        card_type,
        mana_cost,
        outcome,
    ))
}
