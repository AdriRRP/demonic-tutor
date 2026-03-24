//! Supports public stack target materialization.

use crate::domain::play::{
    cards::SpellTargetKind,
    ids::{CardInstanceId, PlayerId},
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
