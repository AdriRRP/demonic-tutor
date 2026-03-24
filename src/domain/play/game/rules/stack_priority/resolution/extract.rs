//! Supports stack priority resolution extract.

use crate::domain::play::{
    cards::{ActivatedAbilityProfile, SpellPayload},
    errors::{DomainError, GameError},
    game::model::{StackObject, StackObjectKind},
    game::SpellTarget,
    ids::{CardInstanceId, PlayerId},
};

pub(super) struct ResolvedSpellObject {
    pub stack_object_number: u32,
    pub source_card_id: CardInstanceId,
    pub controller_id: PlayerId,
    pub payload: SpellPayload,
    pub mana_cost_paid: u32,
    pub target: Option<SpellTarget>,
}

pub(super) fn extract_resolved_spell_object(
    stack_object: StackObject,
) -> Result<ResolvedSpellObject, DomainError> {
    let stack_object_number = stack_object.number();
    let controller_id = stack_object.controller_id().clone();
    let StackObjectKind::Spell(spell) = stack_object.into_kind() else {
        return Err(DomainError::Game(GameError::InternalInvariantViolation(
            "spell extraction requires a spell stack object".to_string(),
        )));
    };
    let source_card_id = spell.source_card_id().clone();
    let mana_cost_paid = spell.mana_cost_paid();
    let target = spell.target().cloned();
    let payload = spell.into_payload();

    Ok(ResolvedSpellObject {
        stack_object_number,
        source_card_id,
        controller_id,
        payload,
        mana_cost_paid,
        target,
    })
}

pub(super) struct ResolvedActivatedAbility {
    pub stack_object_number: u32,
    pub source_card_id: CardInstanceId,
    pub controller_id: PlayerId,
    pub ability: ActivatedAbilityProfile,
}

pub(super) fn extract_resolved_activated_ability(
    stack_object: StackObject,
) -> Result<ResolvedActivatedAbility, DomainError> {
    let stack_object_number = stack_object.number();
    let controller_id = stack_object.controller_id().clone();
    let StackObjectKind::ActivatedAbility(ability) = stack_object.into_kind() else {
        return Err(DomainError::Game(GameError::InternalInvariantViolation(
            "activated-ability extraction requires an activated ability object".to_string(),
        )));
    };

    Ok(ResolvedActivatedAbility {
        stack_object_number,
        source_card_id: ability.source_card_id().clone(),
        controller_id,
        ability: ability.ability(),
    })
}
