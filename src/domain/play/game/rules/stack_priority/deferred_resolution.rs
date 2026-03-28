//! Supports shared helpers for deferred stack resolution corridors.

use crate::domain::play::{
    cards::{CardType, SpellPayload},
    errors::{DomainError, GameError},
    events::{SpellCast, SpellCastOutcome, StackTopResolved},
    game::{model::StackObjectKind, Player},
    ids::{CardInstanceId, GameId, PlayerId},
};

#[derive(Debug)]
pub(super) struct PendingSpellResolution {
    controller_id: PlayerId,
    stack_object_number: u32,
    source_card_id: CardInstanceId,
    card_type: CardType,
    mana_cost_paid: u32,
    payload: SpellPayload,
}

impl PendingSpellResolution {
    #[must_use]
    pub(super) const fn controller_id(&self) -> &PlayerId {
        &self.controller_id
    }

    #[must_use]
    pub(super) const fn source_card_id(&self) -> &CardInstanceId {
        &self.source_card_id
    }

    #[must_use]
    pub(super) const fn card_type(&self) -> CardType {
        self.card_type
    }

    #[must_use]
    pub(super) const fn mana_cost_paid(&self) -> u32 {
        self.mana_cost_paid
    }

    #[must_use]
    pub(super) const fn stack_object_number(&self) -> u32 {
        self.stack_object_number
    }

    pub(super) fn into_payload(self) -> SpellPayload {
        self.payload
    }
}

pub(super) fn remove_pending_spell(
    players: &[Player],
    stack: &mut crate::domain::play::game::StackZone,
    controller_index: usize,
    stack_object_number: u32,
    missing_message: &str,
    non_spell_message: &str,
) -> Result<PendingSpellResolution, DomainError> {
    let stack_object = stack.remove_by_number(stack_object_number).ok_or_else(|| {
        DomainError::Game(GameError::InternalInvariantViolation(
            missing_message.to_string(),
        ))
    })?;
    let controller_id = players[controller_index].id().clone();
    let StackObjectKind::Spell(spell) = stack_object.into_kind() else {
        return Err(DomainError::Game(GameError::InternalInvariantViolation(
            non_spell_message.to_string(),
        )));
    };

    Ok(PendingSpellResolution {
        controller_id,
        stack_object_number,
        source_card_id: spell.source_card_id().clone(),
        card_type: *spell.card_type(),
        mana_cost_paid: spell.mana_cost_paid(),
        payload: spell.into_payload(),
    })
}

pub(super) fn build_spell_resolution_events_from_parts(
    game_id: &GameId,
    controller_id: &PlayerId,
    stack_object_number: u32,
    source_card_id: &CardInstanceId,
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

pub(super) fn move_spell_to_resolution_destination(
    players: &mut [Player],
    controller_index: usize,
    payload: SpellPayload,
    card_type: CardType,
) -> Result<(SpellCastOutcome, Vec<CardInstanceId>), DomainError> {
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
            player
                .receive_battlefield_card(payload.into_card_instance())
                .ok_or_else(|| {
                    DomainError::Game(GameError::InternalInvariantViolation(
                        "failed to move resolved permanent spell to the battlefield".to_string(),
                    ))
                })?;
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
