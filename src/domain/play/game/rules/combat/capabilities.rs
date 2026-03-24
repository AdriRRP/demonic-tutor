//! Supports rules combat capabilities.

use crate::domain::play::cards::{CardInstance, KeywordAbility};

#[must_use]
pub const fn attacker_requires_aerial_blocking_capability(attacker: &CardInstance) -> bool {
    attacker.has_keyword(KeywordAbility::Flying)
}

#[must_use]
pub const fn blocker_can_block_aerial_attacker(blocker: &CardInstance) -> bool {
    blocker.has_keyword(KeywordAbility::Flying) || blocker.has_keyword(KeywordAbility::Reach)
}

#[must_use]
pub const fn can_block_attacker_with_aerial_requirement(
    blocker: &CardInstance,
    attacker_requires_aerial_blocking: bool,
) -> bool {
    if attacker_requires_aerial_blocking {
        blocker_can_block_aerial_attacker(blocker)
    } else {
        true
    }
}
