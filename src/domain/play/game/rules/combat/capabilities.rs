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
pub const fn can_block_attacker(blocker: &CardInstance, attacker: &CardInstance) -> bool {
    if attacker_requires_aerial_blocking_capability(attacker) {
        blocker_can_block_aerial_attacker(blocker)
    } else {
        true
    }
}
