//! Supports public stack target materialization.

use crate::domain::play::{
    cards::SpellTargetKind,
    ids::{CardInstanceId, PlayerId, StackObjectId},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SpellTarget {
    Player(PlayerId),
    Creature(CardInstanceId),
    Permanent(CardInstanceId),
    GraveyardCard(CardInstanceId),
    StackObject(StackObjectId),
}

impl SpellTarget {
    #[must_use]
    pub const fn kind(&self) -> SpellTargetKind {
        match self {
            Self::Player(_) => SpellTargetKind::Player,
            Self::Creature(_) => SpellTargetKind::Creature,
            Self::Permanent(_) => SpellTargetKind::Permanent,
            Self::GraveyardCard(_) => SpellTargetKind::GraveyardCard,
            Self::StackObject(_) => SpellTargetKind::StackSpell,
        }
    }
}
