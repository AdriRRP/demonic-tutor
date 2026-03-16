use {
    super::{phase_behavior, player::Player, Phase},
    crate::domain::{
        commands::AdvanceTurnCommand,
        errors::{DomainError, GameError},
        events::{CardDrawn, PhaseChanged, TurnAdvanced, TurnNumberChanged},
        ids::{GameId, PlayerId},
    },
};

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

fn auto_draw_card(
    game_id: &GameId,
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
        game_id.clone(),
        active_player.clone(),
        card_id,
    )))
}

fn build_events(
    game_id: &GameId,
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
        TurnAdvanced::new(game_id.clone(), active_player.clone()),
        TurnNumberChanged::new(game_id.clone(), from_turn, turn_number),
        PhaseChanged::new(game_id.clone(), from_phase, to_phase),
        card_drawn_event,
    )
}

/// Advances the turn to the next phase and player.
///
/// # Errors
/// Returns an error if auto-draw fails (empty library).
pub fn advance_turn(
    game_id: &GameId,
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

    // Get behavior for current phase using State pattern
    let current_phase_behavior = phase_behavior::get_phase_behavior(&from_phase);

    // Execute phase exit logic
    current_phase_behavior.on_exit(players, active_player)?;

    // Determine next phase
    let to_phase = current_phase_behavior.next_phase();
    let to_phase_behavior = phase_behavior::get_phase_behavior(&to_phase);

    // Check if we need to change players
    if current_phase_behavior.requires_player_change() {
        rotate_to_next_player(players, active_player, turn_number)?;
        *phase = to_phase;

        // Execute phase entry logic for new phase
        to_phase_behavior.on_enter(players, active_player)?;

        return Ok(build_events(
            game_id,
            active_player,
            from_phase,
            to_phase,
            from_turn,
            *turn_number,
            None,
        ));
    }

    // Handle auto-draw if current phase triggers it
    let card_drawn_event = if current_phase_behavior.triggers_auto_draw() {
        auto_draw_card(game_id, players, active_player)?
    } else {
        None
    };

    // Update phase and execute entry logic
    *phase = to_phase;
    to_phase_behavior.on_enter(players, active_player)?;

    Ok(build_events(
        game_id,
        active_player,
        from_phase,
        to_phase,
        from_turn,
        *turn_number,
        card_drawn_event,
    ))
}
