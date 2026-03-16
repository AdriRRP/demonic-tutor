use super::player::Player;
use super::Phase;
use crate::domain::{
    cards::CardInstance,
    commands::AdvanceTurnCommand,
    errors::{DomainError, GameError},
    events::{CardDrawn, PhaseChanged, TurnAdvanced, TurnNumberChanged},
    ids::PlayerId,
};

/// Represents the result of determining the next phase transition.
struct PhaseTransition {
    to_phase: Phase,
    change_player: bool,
    auto_draw: bool,
}

impl Phase {
    const fn next(self) -> Self {
        match self {
            Self::Setup | Self::EndStep => Self::Untap,
            Self::Untap => Self::Upkeep,
            Self::Upkeep => Self::Draw,
            Self::Draw => Self::FirstMain,
            Self::FirstMain => Self::Combat,
            Self::Combat => Self::SecondMain,
            Self::SecondMain => Self::EndStep,
        }
    }

    const fn requires_player_change(self) -> bool {
        matches!(self, Self::EndStep)
    }

    const fn triggers_auto_draw(self) -> bool {
        matches!(self, Self::Draw)
    }
}

fn rotate_to_next_player(
    players: &[Player],
    active_player: &mut PlayerId,
    turn_number: &mut u32,
) -> Result<(), DomainError> {
    let current_idx = players
        .iter()
        .position(|p| p.id() == active_player)
        .ok_or_else(|| {
            DomainError::Game(GameError::InternalInvariantViolation(
                "active player should exist in player list".to_string(),
            ))
        })?;

    let next_idx = (current_idx + 1) % players.len();
    *active_player = players[next_idx].id().clone();
    *turn_number += 1;
    Ok(())
}

fn prepare_players_for_new_turn(players: &mut [Player]) {
    for player in players.iter_mut() {
        *player.lands_played_this_turn_mut() = 0;
        player
            .battlefield_mut()
            .cards_mut()
            .iter_mut()
            .for_each(|card| {
                card.untap();
                card.remove_summoning_sickness();
            });
    }
}

fn cleanup_damage(players: &mut [Player]) {
    for player in players.iter_mut() {
        player
            .battlefield_mut()
            .cards_mut()
            .iter_mut()
            .for_each(CardInstance::clear_damage);
    }
}

fn auto_draw_card(
    players: &mut [Player],
    active_player: &PlayerId,
) -> Result<Option<CardDrawn>, DomainError> {
    let player_idx = players
        .iter()
        .position(|p| p.id() == active_player)
        .ok_or_else(|| DomainError::Game(GameError::PlayerNotFound(active_player.clone())))?;

    let player = &mut players[player_idx];
    let drawn_cards = player.library_mut().draw(1).ok_or_else(|| {
        DomainError::Game(GameError::NotEnoughCardsInLibrary {
            player: active_player.clone(),
            available: player.library().len(),
            requested: 1,
        })
    })?;

    let card = drawn_cards.into_iter().next().ok_or_else(|| {
        DomainError::Game(GameError::InternalInvariantViolation(
            "draw(1) should return exactly one card".to_string(),
        ))
    })?;

    let card_id = card.id().clone();
    player.hand_mut().receive(vec![card]);

    Ok(Some(CardDrawn::new(
        super::Game::id_from_player_id(active_player),
        active_player.clone(),
        card_id,
    )))
}

fn build_events(
    active_player: &PlayerId,
    from_phase: Phase,
    to_phase: Phase,
    from_turn: u32,
    turn_number: u32,
    card_drawn_event: Option<CardDrawn>,
) -> (
    TurnAdvanced,
    TurnNumberChanged,
    PhaseChanged,
    Option<CardDrawn>,
) {
    (
        TurnAdvanced::new(
            super::Game::id_from_player_id(active_player),
            active_player.clone(),
        ),
        TurnNumberChanged::new(
            super::Game::id_from_player_id(active_player),
            from_turn,
            turn_number,
        ),
        PhaseChanged::new(
            super::Game::id_from_player_id(active_player),
            from_phase,
            to_phase,
        ),
        card_drawn_event,
    )
}

/// Advances the turn to the next phase and player.
///
/// # Errors
/// Returns an error if auto-draw fails (empty library).
pub fn advance_turn(
    players: &mut [Player],
    active_player: &mut PlayerId,
    phase: &mut Phase,
    turn_number: &mut u32,
    _cmd: AdvanceTurnCommand,
) -> Result<
    (
        TurnAdvanced,
        TurnNumberChanged,
        PhaseChanged,
        Option<CardDrawn>,
    ),
    DomainError,
> {
    let from_phase = *phase;
    let from_turn = *turn_number;
    let to_phase = from_phase.next();

    let transition = PhaseTransition {
        to_phase,
        change_player: from_phase.requires_player_change(),
        auto_draw: from_phase.triggers_auto_draw(),
    };

    // Cleanup damage at end of EndStep
    if matches!(from_phase, Phase::EndStep) {
        cleanup_damage(players);
    }

    if transition.change_player {
        rotate_to_next_player(players, active_player, turn_number)?;
        prepare_players_for_new_turn(players);
        *phase = transition.to_phase;
        return Ok(build_events(
            active_player,
            from_phase,
            transition.to_phase,
            from_turn,
            *turn_number,
            None,
        ));
    }

    let card_drawn_event = if transition.auto_draw {
        auto_draw_card(players, active_player)?
    } else {
        None
    };

    *phase = transition.to_phase;
    Ok(build_events(
        active_player,
        from_phase,
        transition.to_phase,
        from_turn,
        *turn_number,
        card_drawn_event,
    ))
}
