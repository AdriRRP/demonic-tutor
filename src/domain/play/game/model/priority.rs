//! Supports game model priority.

use crate::domain::play::ids::PlayerId;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PriorityState {
    current_holder: PlayerId,
    has_pending_pass: bool,
}

impl PriorityState {
    #[must_use]
    pub const fn opened(current_holder: PlayerId) -> Self {
        Self {
            current_holder,
            has_pending_pass: false,
        }
    }

    #[must_use]
    pub const fn after_first_pass(next_holder: PlayerId) -> Self {
        Self {
            current_holder: next_holder,
            has_pending_pass: true,
        }
    }

    #[must_use]
    pub const fn current_holder(&self) -> &PlayerId {
        &self.current_holder
    }

    #[must_use]
    pub const fn has_pending_pass(&self) -> bool {
        self.has_pending_pass
    }
}
