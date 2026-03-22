use crate::domain::play::{
    cards::{CardType, SpellCardSnapshot, SpellTargetKind, SupportedSpellRules},
    ids::{CardInstanceId, GameId, PlayerId, StackObjectId},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SpellTarget {
    Player(PlayerId),
    Creature(CardInstanceId),
}

impl SpellTarget {
    #[must_use]
    pub const fn kind(&self) -> SpellTargetKind {
        match self {
            Self::Player(_) => SpellTargetKind::Player,
            Self::Creature(_) => SpellTargetKind::Creature,
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

    pub fn next_id(&mut self, game_id: &GameId) -> StackObjectId {
        let id = StackObjectId::new(format!(
            "{}-stack-{}",
            game_id.as_str(),
            self.next_object_number
        ));
        self.next_object_number += 1;
        id
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
    id: StackObjectId,
    controller_id: PlayerId,
    kind: StackObjectKind,
}

impl StackObject {
    #[must_use]
    pub const fn new(id: StackObjectId, controller_id: PlayerId, kind: StackObjectKind) -> Self {
        Self {
            id,
            controller_id,
            kind,
        }
    }

    #[must_use]
    pub const fn id(&self) -> &StackObjectId {
        &self.id
    }

    #[must_use]
    pub const fn controller_id(&self) -> &PlayerId {
        &self.controller_id
    }

    #[must_use]
    pub const fn source_card_id(&self) -> &CardInstanceId {
        match &self.kind {
            StackObjectKind::Spell(spell) => spell.source_card_id(),
        }
    }

    #[must_use]
    pub const fn kind(&self) -> &StackObjectKind {
        &self.kind
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StackObjectKind {
    Spell(SpellOnStack),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpellOnStack {
    source_card_id: CardInstanceId,
    card: SpellCardSnapshot,
    card_type: CardType,
    supported_spell_rules: SupportedSpellRules,
    mana_cost_paid: u32,
    target: Option<SpellTarget>,
}

impl SpellOnStack {
    #[must_use]
    pub fn new(card: SpellCardSnapshot, mana_cost_paid: u32, target: Option<SpellTarget>) -> Self {
        Self {
            source_card_id: card.id().clone(),
            card_type: card.card_type().clone(),
            supported_spell_rules: card.supported_spell_rules(),
            card,
            mana_cost_paid,
            target,
        }
    }

    #[must_use]
    pub const fn source_card_id(&self) -> &CardInstanceId {
        &self.source_card_id
    }

    #[must_use]
    pub const fn card(&self) -> &SpellCardSnapshot {
        &self.card
    }

    #[must_use]
    pub const fn card_type(&self) -> &CardType {
        &self.card_type
    }

    #[must_use]
    pub const fn supported_spell_rules(&self) -> SupportedSpellRules {
        self.supported_spell_rules
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
    pub fn into_card(self) -> SpellCardSnapshot {
        self.card
    }

    #[must_use]
    pub fn into_target(self) -> Option<SpellTarget> {
        self.target
    }
}
