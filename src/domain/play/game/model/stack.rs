//! Supports game model stack.

use crate::domain::play::{
    cards::{
        ActivatedAbilityEffect, ActivatedAbilityProfile, CardType, SpellPayload, SpellTargetKind,
        SupportedSpellRules,
    },
    ids::{CardInstanceId, GameId, PlayerId, StackObjectId},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SpellTarget {
    Player(PlayerId),
    Creature(CardInstanceId),
    GraveyardCard(CardInstanceId),
}

impl SpellTarget {
    #[must_use]
    pub const fn kind(&self) -> SpellTargetKind {
        match self {
            Self::Player(_) => SpellTargetKind::Player,
            Self::Creature(_) => SpellTargetKind::Creature,
            Self::GraveyardCard(_) => SpellTargetKind::GraveyardCard,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StackZone {
    objects: Vec<StackObject>,
    next_object_number: u32,
}

impl Default for StackZone {
    fn default() -> Self {
        Self::empty()
    }
}

impl StackZone {
    #[must_use]
    pub const fn empty() -> Self {
        Self {
            objects: Vec::new(),
            next_object_number: 1,
        }
    }

    pub const fn next_object_number(&mut self) -> u32 {
        let number = self.next_object_number;
        self.next_object_number += 1;
        number
    }

    #[must_use]
    pub fn object_id(&self, game_id: &GameId, object_number: u32) -> StackObjectId {
        StackObjectId::for_stack_object(game_id, object_number)
    }

    #[must_use]
    pub fn objects(&self) -> &[StackObject] {
        &self.objects
    }

    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.objects.is_empty()
    }

    #[must_use]
    pub const fn len(&self) -> usize {
        self.objects.len()
    }

    #[must_use]
    pub fn top(&self) -> Option<&StackObject> {
        self.objects.last()
    }

    pub fn push(&mut self, object: StackObject) {
        self.objects.push(object);
    }

    pub fn pop(&mut self) -> Option<StackObject> {
        self.objects.pop()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StackObject {
    number: u32,
    controller_id: PlayerId,
    kind: StackObjectKind,
}

impl StackObject {
    #[must_use]
    pub const fn new(number: u32, controller_id: PlayerId, kind: StackObjectKind) -> Self {
        Self {
            number,
            controller_id,
            kind,
        }
    }

    #[must_use]
    pub const fn number(&self) -> u32 {
        self.number
    }

    #[must_use]
    pub fn id(&self, game_id: &GameId) -> StackObjectId {
        StackObjectId::for_stack_object(game_id, self.number)
    }

    #[must_use]
    pub const fn controller_id(&self) -> &PlayerId {
        &self.controller_id
    }

    #[must_use]
    pub const fn source_card_id(&self) -> &CardInstanceId {
        match &self.kind {
            StackObjectKind::Spell(spell) => spell.source_card_id(),
            StackObjectKind::ActivatedAbility(ability) => ability.source_card_id(),
        }
    }

    #[must_use]
    pub const fn kind(&self) -> &StackObjectKind {
        &self.kind
    }

    #[must_use]
    pub fn into_kind(self) -> StackObjectKind {
        self.kind
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StackObjectKind {
    Spell(SpellOnStack),
    ActivatedAbility(ActivatedAbilityOnStack),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpellOnStack {
    payload: SpellPayload,
    mana_cost_paid: u32,
    target: Option<SpellTarget>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ActivatedAbilityOnStack {
    source_card_id: CardInstanceId,
    ability: ActivatedAbilityProfile,
}

impl ActivatedAbilityOnStack {
    #[must_use]
    pub const fn new(source_card_id: CardInstanceId, ability: ActivatedAbilityProfile) -> Self {
        Self {
            source_card_id,
            ability,
        }
    }

    #[must_use]
    pub const fn source_card_id(&self) -> &CardInstanceId {
        &self.source_card_id
    }

    #[must_use]
    pub const fn ability(&self) -> ActivatedAbilityProfile {
        self.ability
    }

    #[must_use]
    pub const fn effect(&self) -> ActivatedAbilityEffect {
        self.ability.effect()
    }
}

impl SpellOnStack {
    #[must_use]
    pub const fn new(
        payload: SpellPayload,
        mana_cost_paid: u32,
        target: Option<SpellTarget>,
    ) -> Self {
        Self {
            payload,
            mana_cost_paid,
            target,
        }
    }

    #[must_use]
    pub const fn source_card_id(&self) -> &CardInstanceId {
        self.payload.id()
    }

    #[must_use]
    pub const fn payload(&self) -> &SpellPayload {
        &self.payload
    }

    #[must_use]
    pub fn card_type(&self) -> &CardType {
        self.payload.card_type()
    }

    #[must_use]
    pub fn supported_spell_rules(&self) -> SupportedSpellRules {
        self.payload.supported_spell_rules()
    }

    #[must_use]
    pub const fn mana_cost_paid(&self) -> u32 {
        self.mana_cost_paid
    }

    #[must_use]
    pub const fn target(&self) -> Option<&SpellTarget> {
        self.target.as_ref()
    }

    #[must_use]
    pub fn into_payload(self) -> SpellPayload {
        self.payload
    }

    #[must_use]
    pub fn into_target(self) -> Option<SpellTarget> {
        self.target
    }
}
