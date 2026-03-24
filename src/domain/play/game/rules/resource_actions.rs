use super::{
    super::{helpers, invariants, model::Player, TerminalState},
    game_effects,
    state_based_actions::{self, StateBasedActionsResult},
};
use crate::domain::play::{
    cards::CardType,
    commands::{AdjustPlayerLifeEffectCommand, PlayLandCommand, TapLandCommand},
    errors::{CardError, DomainError, PhaseError},
    events::{CreatureDied, GameEnded, LandPlayed, LandTapped, LifeChanged, ManaAdded},
    ids::{GameId, PlayerId},
    phase::Phase,
};

#[derive(Debug, Clone)]
pub struct AdjustPlayerLifeEffectOutcome {
    pub life_changed: LifeChanged,
    pub creatures_died: Vec<CreatureDied>,
    pub game_ended: Option<GameEnded>,
}

impl AdjustPlayerLifeEffectOutcome {
    #[must_use]
    pub const fn new(
        life_changed: LifeChanged,
        creatures_died: Vec<CreatureDied>,
        game_ended: Option<GameEnded>,
    ) -> Self {
        Self {
            life_changed,
            creatures_died,
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

    let player = helpers::find_player_mut(players, &cmd.player_id)?;

    if player.lands_played_this_turn() > 0 {
        return Err(DomainError::Phase(PhaseError::AlreadyPlayedLandThisTurn(
            cmd.player_id.clone(),
        )));
    }

    let card_id = cmd.card_id.clone();
    let card_type = helpers::hand_card_type(player, &cmd.player_id, &card_id)?;

    if !card_type.is_land() {
        return Err(DomainError::Card(CardError::NotALand(card_id)));
    }

    let card = helpers::remove_card_from_hand(player, &cmd.player_id, &card_id)?;
    player.receive_battlefield_card(card);
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
    priority: Option<&crate::domain::play::game::PriorityState>,
    cmd: TapLandCommand,
) -> Result<(LandTapped, ManaAdded), DomainError> {
    if priority.is_none() {
        invariants::require_active_player(active_player, &cmd.player_id)?;
    }

    if priority.is_none() && !matches!(phase, Phase::FirstMain | Phase::SecondMain) {
        return Err(DomainError::Phase(PhaseError::InvalidForPlayingCard {
            phase: *phase,
        }));
    }

    let player = helpers::find_player_mut(players, &cmd.player_id)?;

    let card = player.battlefield_card_mut(&cmd.card_id).ok_or_else(|| {
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

    let mana_ability = card
        .activated_mana_ability()
        .ok_or_else(|| DomainError::Card(CardError::NotALand(cmd.card_id.clone())))?;

    if let Some(color) = mana_ability.color() {
        player.add_colored_mana(color, mana_ability.amount());
    } else {
        player.add_mana(mana_ability.amount());
    }
    let new_mana = player.mana();

    Ok((
        LandTapped::new(game_id.clone(), cmd.player_id.clone(), cmd.card_id.clone()),
        ManaAdded::new(
            game_id.clone(),
            cmd.player_id,
            mana_ability.amount(),
            mana_ability.color(),
            new_mana,
        ),
    ))
}

/// Adjusts a player's life total by a signed delta.
///
/// # Errors
/// Returns an error if the player is not found.
pub fn adjust_player_life_effect(
    game_id: &GameId,
    players: &mut [Player],
    terminal_state: &mut TerminalState,
    cmd: AdjustPlayerLifeEffectCommand,
) -> Result<AdjustPlayerLifeEffectOutcome, DomainError> {
    let AdjustPlayerLifeEffectCommand {
        caster_id,
        target_player_id,
        life_delta,
    } = cmd;

    helpers::find_player_index(players, &caster_id)?;
    let life_changed =
        game_effects::adjust_player_life(game_id, players, &target_player_id, life_delta)?;
    let StateBasedActionsResult {
        creatures_died,
        game_ended,
    } = state_based_actions::check_state_based_actions(game_id, players, terminal_state)?;

    Ok(AdjustPlayerLifeEffectOutcome::new(
        life_changed,
        creatures_died,
        game_ended,
    ))
}
