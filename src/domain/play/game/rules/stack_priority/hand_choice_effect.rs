//! Supports rules stack priority pending hand-choice effects.

use crate::domain::play::{
    commands::ResolvePendingHandChoiceCommand,
    errors::{DomainError, GameError},
    events::{
        CardDiscarded, CardDrawn, CardMovedZone, DiscardKind, DrawKind, GameEnded,
        SpellCastOutcome, ZoneType,
    },
    game::{model::PlayerCardZone, PendingDecision, PendingHandChoiceKind, PriorityState},
    ids::PlayerCardHandle,
};

use super::{
    deferred_resolution::{remove_pending_spell, resolve_pending_spell_to_default_destination},
    ResolvePendingHandChoiceOutcome, StackPriorityContext,
};

const fn restore_pending_hand_choice(
    pending_decision: &mut Option<PendingDecision>,
    controller_index: usize,
    stack_object_number: u32,
    kind: PendingHandChoiceKind,
) {
    *pending_decision = Some(PendingDecision::hand_choice(
        controller_index,
        stack_object_number,
        kind,
    ));
}

pub(super) fn draw_cards_for_pending_effect(
    game_id: &crate::domain::play::ids::GameId,
    players: &mut [crate::domain::play::game::Player],
    terminal_state: &mut crate::domain::play::game::TerminalState,
    controller_index: usize,
    draw_count: u32,
) -> Result<(Vec<CardDrawn>, Option<GameEnded>), DomainError> {
    let controller_id = players[controller_index].id().clone();
    let mut cards_drawn = Vec::new();

    for _ in 0..draw_count {
        let Some(card_id) = players[controller_index].draw_one_into_hand() else {
            let game_ended =
                crate::domain::play::game::rules::game_effects::end_game_for_empty_library_draw(
                    game_id,
                    players,
                    terminal_state,
                    controller_index,
                )?;
            return Ok((cards_drawn, Some(game_ended)));
        };

        cards_drawn.push(CardDrawn::new(
            game_id.clone(),
            controller_id.clone(),
            card_id,
            DrawKind::ExplicitEffect,
        ));
    }

    Ok((cards_drawn, None))
}

fn validate_pending_hand_choice(
    players: &[crate::domain::play::game::Player],
    pending_decision: &mut Option<PendingDecision>,
    controller_index: usize,
    stack_object_number: u32,
    kind: PendingHandChoiceKind,
    player_id: &crate::domain::play::ids::PlayerId,
    chosen_card_id: &crate::domain::play::ids::CardInstanceId,
) -> Result<PlayerCardHandle, DomainError> {
    let current_controller = players[controller_index].id().clone();
    if current_controller != *player_id {
        restore_pending_hand_choice(
            pending_decision,
            controller_index,
            stack_object_number,
            kind,
        );
        return Err(DomainError::Game(
            GameError::NotPendingHandChoiceController {
                current: current_controller,
                requested: player_id.clone(),
            },
        ));
    }

    let Some(handle) = players[controller_index].resolve_public_card_handle(chosen_card_id) else {
        restore_pending_hand_choice(
            pending_decision,
            controller_index,
            stack_object_number,
            kind,
        );
        return Err(DomainError::Game(GameError::InvalidHandCardChoice(
            chosen_card_id.clone(),
        )));
    };

    if players[controller_index].card_zone(chosen_card_id) != Some(PlayerCardZone::Hand) {
        restore_pending_hand_choice(
            pending_decision,
            controller_index,
            stack_object_number,
            kind,
        );
        return Err(DomainError::Game(GameError::InvalidHandCardChoice(
            chosen_card_id.clone(),
        )));
    }

    Ok(handle)
}

fn zone_change_for_drawn_card(event: &CardDrawn) -> CardMovedZone {
    CardMovedZone::new(
        event.game_id.clone(),
        event.player_id.clone(),
        event.card_id.clone(),
        ZoneType::Library,
        ZoneType::Hand,
    )
}

fn zone_change_for_spell_cast(
    spell_cast: &crate::domain::play::events::SpellCast,
) -> CardMovedZone {
    let destination_zone = match spell_cast.outcome {
        SpellCastOutcome::EnteredBattlefield => ZoneType::Battlefield,
        SpellCastOutcome::ResolvedToGraveyard => ZoneType::Graveyard,
        SpellCastOutcome::ResolvedToExile => ZoneType::Exile,
    };
    CardMovedZone::new(
        spell_cast.game_id.clone(),
        spell_cast.player_id.clone(),
        spell_cast.card_id.clone(),
        ZoneType::Stack,
        destination_zone,
    )
}

/// Finishes a pending hand-choice spell effect.
///
/// # Errors
/// Returns an error if no hand-choice effect is pending, if the caller is not
/// its controller, or if the selected card is not currently in that player's hand.
#[allow(clippy::too_many_lines)]
pub fn resolve_pending_hand_choice(
    ctx: StackPriorityContext<'_>,
    cmd: ResolvePendingHandChoiceCommand,
) -> Result<ResolvePendingHandChoiceOutcome, DomainError> {
    let StackPriorityContext {
        game_id,
        players,
        active_player,
        stack,
        priority,
        pending_decision,
        terminal_state,
        ..
    } = ctx;

    let ResolvePendingHandChoiceCommand {
        player_id,
        chosen_card_id,
    } = cmd;

    if priority.is_some() {
        return Err(DomainError::Game(GameError::InternalInvariantViolation(
            "pending hand-choice resolution requires the priority window to be closed".to_string(),
        )));
    }

    let (controller_index, stack_object_number, kind) = match pending_decision
        .take()
        .ok_or(DomainError::Game(GameError::NoPendingHandChoice))?
    {
        PendingDecision::HandChoice {
            controller_index,
            stack_object_number,
            kind,
        } => (controller_index, stack_object_number, kind),
        other => {
            *pending_decision = Some(other);
            return Err(DomainError::Game(GameError::NoPendingHandChoice));
        }
    };

    let handle = validate_pending_hand_choice(
        players,
        pending_decision,
        controller_index,
        stack_object_number,
        kind,
        &player_id,
        &chosen_card_id,
    )?;

    let pending_spell = remove_pending_spell(
        players,
        stack,
        controller_index,
        stack_object_number,
        "pending hand-choice spell must still exist on the stack",
        "pending hand choice requires a spell stack object",
    )?;
    let controller_id = pending_spell.controller_id().clone();

    players[controller_index]
        .move_hand_handle_to_graveyard(handle)
        .ok_or_else(|| {
            DomainError::Game(GameError::InvalidHandCardChoice(chosen_card_id.clone()))
        })?;
    let card_discarded = Some(CardDiscarded::new(
        game_id.clone(),
        controller_id.clone(),
        chosen_card_id,
        DiscardKind::SpellEffect,
    ));
    let mut zone_changes = card_discarded
        .as_ref()
        .map(|discarded| {
            vec![CardMovedZone::new(
                game_id.clone(),
                controller_id.clone(),
                discarded.card_id.clone(),
                ZoneType::Hand,
                ZoneType::Graveyard,
            )]
        })
        .unwrap_or_default();

    let (card_drawn, game_ended) = match kind {
        PendingHandChoiceKind::Loot { .. } => (Vec::new(), None),
        PendingHandChoiceKind::Rummage { draw_count } => draw_cards_for_pending_effect(
            game_id,
            players,
            terminal_state,
            controller_index,
            draw_count,
        )?,
    };
    zone_changes.extend(card_drawn.iter().map(zone_change_for_drawn_card));

    let (stack_top_resolved, spell_cast, _moved_cards) =
        resolve_pending_spell_to_default_destination(
            game_id,
            players,
            controller_index,
            pending_spell,
        )?;
    zone_changes.push(zone_change_for_spell_cast(&spell_cast));

    if terminal_state.is_over() {
        *priority = None;
    } else {
        *priority = Some(PriorityState::opened(active_player.clone()));
    }

    Ok(ResolvePendingHandChoiceOutcome {
        stack_top_resolved: Some(stack_top_resolved),
        spell_cast: Some(spell_cast),
        card_drawn,
        card_discarded,
        zone_changes,
        game_ended,
        priority_still_open: priority.is_some(),
    })
}
