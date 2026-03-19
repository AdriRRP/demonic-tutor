use crate::domain::play::{
    cards::{CardInstance, CardType},
    ids::{CardInstanceId, GameId, PlayerId, StackObjectId},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SpellTarget {
    Player(PlayerId),
    Creature(CardInstanceId),
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
    source_card_id: CardInstanceId,
    kind: StackObjectKind,
}

impl StackObject {
    #[must_use]
    pub const fn new(
        id: StackObjectId,
        controller_id: PlayerId,
        source_card_id: CardInstanceId,
        kind: StackObjectKind,
    ) -> Self {
        Self {
            id,
            controller_id,
            source_card_id,
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
        &self.source_card_id
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
    card: CardInstance,
    mana_cost_paid: u32,
    target: Option<SpellTarget>,
}

impl SpellOnStack {
    #[must_use]
    pub const fn new(card: CardInstance, mana_cost_paid: u32, target: Option<SpellTarget>) -> Self {
        Self {
            card,
            mana_cost_paid,
            target,
        }
    }

    #[must_use]
    pub const fn card(&self) -> &CardInstance {
        &self.card
    }

    #[must_use]
    pub const fn card_type(&self) -> &CardType {
        self.card.card_type()
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
    pub fn into_card(self) -> CardInstance {
        self.card
    }

    #[must_use]
    pub fn into_target(self) -> Option<SpellTarget> {
        self.target
    }
}
