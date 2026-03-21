use super::super::spell_effects::supported_spell_rules;
use crate::domain::play::{
    cards::{CardInstance, CardType, SupportedSpellRules},
    game::model::{StackObject, StackObjectKind},
    game::SpellTarget,
    ids::{CardInstanceId, PlayerId, StackObjectId},
};

pub(super) struct ResolvedSpellObject {
    pub source_card_id: CardInstanceId,
    pub controller_id: PlayerId,
    pub stack_object_id: StackObjectId,
    pub card: CardInstance,
    pub card_type: CardType,
    pub mana_cost_paid: u32,
    pub supported_spell_rules: SupportedSpellRules,
    pub target: Option<SpellTarget>,
}

pub(super) fn extract_resolved_spell_object(stack_object: &StackObject) -> ResolvedSpellObject {
    let stack_object_id = stack_object.id().clone();
    let controller_id = stack_object.controller_id().clone();
    let source_card_id = stack_object.source_card_id().clone();

    let StackObjectKind::Spell(spell) = stack_object.kind().clone();
    let mana_cost_paid = spell.mana_cost_paid();
    let target = spell.target().cloned();
    let card = spell.into_card();
    let supported_spell_rules = supported_spell_rules(&card);
    let card_type = card.card_type().clone();

    ResolvedSpellObject {
        source_card_id,
        controller_id,
        stack_object_id,
        card,
        card_type,
        mana_cost_paid,
        supported_spell_rules,
        target,
    }
}
