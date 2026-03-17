use crate::domain::play::ids::PlayerId;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PriorityState {
    current_holder: PlayerId,
    passes_in_row: u8,
}

impl PriorityState {
    #[must_use]
    pub const fn new(current_holder: PlayerId) -> Self {
        Self {
            current_holder,
            passes_in_row: 0,
        }
    }

    #[must_use]
    pub const fn new_with_passes(current_holder: PlayerId, passes_in_row: u8) -> Self {
        Self {
            current_holder,
            passes_in_row,
        }
    }

    #[must_use]
    pub const fn current_holder(&self) -> &PlayerId {
        &self.current_holder
    }

    #[must_use]
    pub const fn passes_in_row(&self) -> u8 {
        self.passes_in_row
    }
}
