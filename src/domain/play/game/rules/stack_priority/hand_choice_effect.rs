//! Supports rules stack priority pending hand-choice effects.

use crate::domain::play::{
    cards::CardType,
    commands::ResolvePendingHandChoiceCommand,
    errors::{DomainError, GameError},
    events::{
        CardDiscarded, DiscardKind, GameEnded, SpellCast, SpellCastOutcome, StackTopResolved,
    },
    game::{
        model::{PlayerCardZone, StackObjectKind},
        PendingHandChoiceEffect, PendingHandChoiceKind, PriorityState,
    },
    ids::PlayerCardHandle,
};

use super::{ResolvePendingHandChoiceOutcome, StackPriorityContext};

const fn restore_pending_hand_choice(
    pending_hand_choice_effect: &mut Option<PendingHandChoiceEffect>,
    controller_index: usize,
    stack_object_number: u32,
    kind: PendingHandChoiceKind,
) {
    *pending_hand_choice_effect = Some(PendingHandChoiceEffect::new(
        controller_index,
        stack_object_number,
        kind,
    ));
}

fn build_spell_resolution_events(
    game_id: &crate::domain::play::ids::GameId,
    controller_id: &crate::domain::play::ids::PlayerId,
    stack_object_number: u32,
    source_card_id: &crate::domain::play::ids::CardInstanceId,
    card_type: CardType,
    mana_cost_paid: u32,
    outcome: SpellCastOutcome,
) -> (StackTopResolved, SpellCast) {
    let stack_object_id =
        crate::domain::play::ids::StackObjectId::for_stack_object(game_id, stack_object_number);
    (
        StackTopResolved::new(
            game_id.clone(),
            controller_id.clone(),
            stack_object_id,
            source_card_id.clone(),
        ),
        SpellCast::new(
            game_id.clone(),
            controller_id.clone(),
            source_card_id.clone(),
            card_type,
            mana_cost_paid,
            outcome,
        ),
    )
}

fn draw_cards_for_pending_effect(
    game_id: &crate::domain::play::ids::GameId,
    players: &mut [crate::domain::play::game::Player],
    terminal_state: &mut crate::domain::play::game::TerminalState,
    controller_index: usize,
    draw_count: u32,
) -> Result<Option<GameEnded>, DomainError> {
    for _ in 0..draw_count {
        if players[controller_index].draw_one_into_hand().is_none() {
            let game_ended =
                crate::domain::play::game::rules::game_effects::end_game_for_empty_library_draw(
                    game_id,
                    players,
                    terminal_state,
                    controller_index,
                )?;
            return Ok(Some(game_ended));
        }
    }

    Ok(None)
}

fn move_spell_to_resolution_destination(
    players: &mut [crate::domain::play::game::Player],
    controller_index: usize,
    payload: crate::domain::play::cards::SpellPayload,
    card_type: CardType,
) -> Result<
    (
        SpellCastOutcome,
        Vec<crate::domain::play::ids::CardInstanceId>,
    ),
    DomainError,
> {
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
            let card_id = payload.id().clone();
            player.receive_battlefield_card(payload.into_card_instance());
            Ok((SpellCastOutcome::EnteredBattlefield, vec![card_id]))
        }
        CardType::Instant | CardType::Sorcery => {
            player.receive_graveyard_card(payload.into_card_instance());
            Ok((SpellCastOutcome::ResolvedToGraveyard, Vec::new()))
        }
        CardType::Land => Err(DomainError::Game(GameError::InternalInvariantViolation(
            "land cards cannot resolve from the stack as spells".to_string(),
        ))),
    }
}

fn validate_pending_hand_choice(
    players: &[crate::domain::play::game::Player],
    pending_hand_choice_effect: &mut Option<PendingHandChoiceEffect>,
    controller_index: usize,
    stack_object_number: u32,
    kind: PendingHandChoiceKind,
    player_id: &crate::domain::play::ids::PlayerId,
    chosen_card_id: &crate::domain::play::ids::CardInstanceId,
) -> Result<PlayerCardHandle, DomainError> {
    let current_controller = players[controller_index].id().clone();
    if current_controller != *player_id {
        restore_pending_hand_choice(
            pending_hand_choice_effect,
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
            pending_hand_choice_effect,
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
            pending_hand_choice_effect,
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

/// Finishes a pending hand-choice spell effect.
///
/// # Errors
/// Returns an error if no hand-choice effect is pending, if the caller is not
/// its controller, or if the selected card is not currently in that player's hand.
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
        pending_optional_effect: _,
        pending_hand_choice_effect,
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

    let PendingHandChoiceEffect {
        controller_index,
        stack_object_number,
        kind,
    } = pending_hand_choice_effect
        .take()
        .ok_or(DomainError::Game(GameError::NoPendingHandChoice))?;

    let handle = validate_pending_hand_choice(
        players,
        pending_hand_choice_effect,
        controller_index,
        stack_object_number,
        kind,
        &player_id,
        &chosen_card_id,
    )?;

    let stack_object = stack.remove_by_number(stack_object_number).ok_or_else(|| {
        DomainError::Game(GameError::InternalInvariantViolation(
            "pending hand-choice spell must still exist on the stack".to_string(),
        ))
    })?;
    let controller_id = players[controller_index].id().clone();
    let StackObjectKind::Spell(spell) = stack_object.into_kind() else {
        return Err(DomainError::Game(GameError::InternalInvariantViolation(
            "pending hand choice requires a spell stack object".to_string(),
        )));
    };

    let source_card_id = spell.source_card_id().clone();
    let card_type = *spell.card_type();
    let mana_cost_paid = spell.mana_cost_paid();
    let payload = spell.into_payload();

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

    let game_ended = match kind {
        PendingHandChoiceKind::Loot { .. } => None,
        PendingHandChoiceKind::Rummage { draw_count } => draw_cards_for_pending_effect(
            game_id,
            players,
            terminal_state,
            controller_index,
            draw_count,
        )?,
    };

    let (spell_outcome, moved_cards) =
        move_spell_to_resolution_destination(players, controller_index, payload, card_type)?;
    let (stack_top_resolved, spell_cast) = build_spell_resolution_events(
        game_id,
        &controller_id,
        stack_object_number,
        &source_card_id,
        card_type,
        mana_cost_paid,
        spell_outcome,
    );

    if terminal_state.is_over() {
        *priority = None;
    } else {
        *priority = Some(PriorityState::opened(active_player.clone()));
    }

    Ok(ResolvePendingHandChoiceOutcome {
        stack_top_resolved: Some(stack_top_resolved),
        spell_cast: Some(spell_cast),
        card_discarded,
        moved_cards,
        game_ended,
        priority_still_open: priority.is_some(),
    })
}
