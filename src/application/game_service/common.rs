//! Supports application game service common.

use crate::domain::play::events::DomainEvent;

#[derive(Default)]
pub(super) struct DomainEvents {
    items: Vec<DomainEvent>,
}

impl DomainEvents {
    pub(super) fn with<T>(event: T) -> Self
    where
        T: Into<DomainEvent>,
    {
        Self {
            items: vec![event.into()],
        }
    }

    pub(super) fn push<T>(&mut self, event: T)
    where
        T: Into<DomainEvent>,
    {
        self.items.push(event.into());
    }

    pub(super) fn push_optional<T>(&mut self, event: Option<T>)
    where
        T: Into<DomainEvent>,
    {
        if let Some(event) = event {
            self.push(event);
        }
    }

    pub(super) fn extend<T, I>(&mut self, events: I)
    where
        T: Into<DomainEvent>,
        I: IntoIterator<Item = T>,
    {
        self.items.extend(events.into_iter().map(Into::into));
    }

    pub(super) fn into_vec(self) -> Vec<DomainEvent> {
        self.items
    }
}
