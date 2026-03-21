use crate::domain::play::ids::PlayerId;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PriorityState {
    current_holder: PlayerId,
    passes_in_row: u8,
}

impl PriorityState {
    #[must_use]
    pub const fn opened(current_holder: PlayerId) -> Self {
        Self {
            current_holder,
            passes_in_row: 0,
        }
    }

    #[must_use]
    pub const fn after_first_pass(next_holder: PlayerId) -> Self {
        Self {
            current_holder: next_holder,
            passes_in_row: 1,
        }
    }

    #[must_use]
    pub const fn current_holder(&self) -> &PlayerId {
        &self.current_holder
    }

    #[must_use]
    pub const fn has_pending_pass(&self) -> bool {
        self.passes_in_row > 0
    }
}
