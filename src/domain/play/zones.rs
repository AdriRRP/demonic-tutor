//! Supports domain play zones.

use {
    crate::domain::play::ids::PlayerCardHandle,
    rand::seq::SliceRandom,
    std::collections::{HashMap, VecDeque},
};

#[derive(Debug, Clone, PartialEq, Eq, Default)]
struct IndexedOrderedZone {
    slots: Vec<Option<OrderedHandleSlot>>,
    handle_to_slot: HashMap<PlayerCardHandle, usize>,
    free_slots: Vec<usize>,
    head: Option<usize>,
    tail: Option<usize>,
    len: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct OrderedHandleSlot {
    handle: PlayerCardHandle,
    prev: Option<usize>,
    next: Option<usize>,
}

struct IndexedOrderedZoneIter<'a> {
    zone: &'a IndexedOrderedZone,
    current: Option<usize>,
}

impl IndexedOrderedZone {
    fn receive_many(&mut self, handles: Vec<PlayerCardHandle>) {
        for handle in handles {
            self.push(handle);
        }
    }

    fn push(&mut self, handle: PlayerCardHandle) {
        let slot_index = if let Some(free_slot) = self.free_slots.pop() {
            free_slot
        } else {
            self.slots.push(None);
            self.slots.len() - 1
        };

        let slot = OrderedHandleSlot {
            handle,
            prev: self.tail,
            next: None,
        };

        if let Some(tail_index) = self.tail {
            if let Some(tail_slot) = self.slots[tail_index].as_mut() {
                tail_slot.next = Some(slot_index);
            }
        } else {
            self.head = Some(slot_index);
        }

        self.slots[slot_index] = Some(slot);
        self.tail = Some(slot_index);
        self.handle_to_slot.insert(handle, slot_index);
        self.len += 1;
    }

    const fn len(&self) -> usize {
        self.len
    }

    const fn is_empty(&self) -> bool {
        self.len == 0
    }

    fn iter(&self) -> impl Iterator<Item = &PlayerCardHandle> {
        IndexedOrderedZoneIter {
            zone: self,
            current: self.head,
        }
    }

    fn contains(&self, handle: PlayerCardHandle) -> bool {
        self.handle_to_slot.contains_key(&handle)
    }

    fn handle_at(&self, index: usize) -> Option<PlayerCardHandle> {
        let mut current = self.head?;
        let mut visible_index = 0;

        loop {
            let slot = self.slots.get(current)?.as_ref()?;
            if visible_index == index {
                return Some(slot.handle);
            }
            current = slot.next?;
            visible_index += 1;
        }
    }

    fn drain_all(&mut self) -> Vec<PlayerCardHandle> {
        let handles = self.iter().copied().collect();
        self.slots.clear();
        self.handle_to_slot.clear();
        self.free_slots.clear();
        self.head = None;
        self.tail = None;
        self.len = 0;
        handles
    }

    fn remove_preserving_order(&mut self, handle: PlayerCardHandle) -> Option<PlayerCardHandle> {
        let slot_index = self.handle_to_slot.remove(&handle)?;
        let slot = self.slots.get_mut(slot_index)?.take()?;

        if let Some(prev_index) = slot.prev {
            if let Some(prev_slot) = self.slots[prev_index].as_mut() {
                prev_slot.next = slot.next;
            }
        } else {
            self.head = slot.next;
        }

        if let Some(next_index) = slot.next {
            if let Some(next_slot) = self.slots[next_index].as_mut() {
                next_slot.prev = slot.prev;
            }
        } else {
            self.tail = slot.prev;
        }

        self.free_slots.push(slot_index);
        self.len -= 1;
        Some(slot.handle)
    }
}

impl<'a> Iterator for IndexedOrderedZoneIter<'a> {
    type Item = &'a PlayerCardHandle;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.current?;
        let slot = self.zone.slots.get(current)?.as_ref()?;
        self.current = slot.next;
        Some(&slot.handle)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Library(VecDeque<PlayerCardHandle>);

impl Library {
    #[must_use]
    pub fn new(handles: Vec<PlayerCardHandle>) -> Self {
        Self(VecDeque::from(handles))
    }

    pub fn draw_one(&mut self) -> Option<PlayerCardHandle> {
        self.0.pop_front()
    }

    pub fn draw(&mut self, n: usize) -> Option<Vec<PlayerCardHandle>> {
        if self.0.len() >= n {
            Some((0..n).filter_map(|_| self.0.pop_front()).collect())
        } else {
            None
        }
    }

    #[must_use]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn receive(&mut self, handles: Vec<PlayerCardHandle>) {
        self.0.extend(handles);
    }

    #[must_use]
    pub fn contains(&self, handle: PlayerCardHandle) -> bool {
        self.0.contains(&handle)
    }

    pub fn shuffle(&mut self) {
        let mut rng = rand::make_rng::<rand::rngs::StdRng>();
        self.0.make_contiguous().shuffle(&mut rng);
    }

    pub fn iter(&self) -> impl Iterator<Item = &PlayerCardHandle> {
        self.0.iter()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Hand {
    storage: IndexedOrderedZone,
}

impl Hand {
    #[must_use]
    pub fn new() -> Self {
        Self {
            storage: IndexedOrderedZone::default(),
        }
    }

    pub fn receive(&mut self, handles: Vec<PlayerCardHandle>) {
        self.storage.receive_many(handles);
    }

    #[must_use]
    pub const fn len(&self) -> usize {
        self.storage.len()
    }

    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.storage.is_empty()
    }

    pub fn iter(&self) -> impl Iterator<Item = &PlayerCardHandle> {
        self.storage.iter()
    }

    #[must_use]
    pub fn contains(&self, handle: PlayerCardHandle) -> bool {
        self.storage.contains(handle)
    }

    #[must_use]
    pub fn handle_at(&self, index: usize) -> Option<PlayerCardHandle> {
        self.storage.handle_at(index)
    }

    /// Removes and returns all card ids from the hand, leaving it empty.
    pub fn drain_all(&mut self) -> Vec<PlayerCardHandle> {
        self.storage.drain_all()
    }

    #[must_use]
    pub fn remove(&mut self, handle: PlayerCardHandle) -> Option<PlayerCardHandle> {
        self.storage.remove_preserving_order(handle)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Battlefield {
    handles: Vec<PlayerCardHandle>,
    positions: HashMap<PlayerCardHandle, usize>,
}

impl Battlefield {
    #[must_use]
    pub fn new() -> Self {
        Self {
            handles: Vec::new(),
            positions: HashMap::new(),
        }
    }

    pub fn add(&mut self, handle: PlayerCardHandle) {
        let index = self.handles.len();
        self.positions.insert(handle, index);
        self.handles.push(handle);
    }

    #[must_use]
    pub const fn len(&self) -> usize {
        self.handles.len()
    }

    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.handles.is_empty()
    }

    pub fn iter(&self) -> impl Iterator<Item = &PlayerCardHandle> {
        self.handles.iter()
    }

    #[must_use]
    pub fn contains(&self, handle: PlayerCardHandle) -> bool {
        self.positions.contains_key(&handle)
    }

    #[must_use]
    pub fn handle_at(&self, index: usize) -> Option<PlayerCardHandle> {
        self.handles.get(index).copied()
    }

    #[must_use]
    pub fn remove(&mut self, handle: PlayerCardHandle) -> Option<PlayerCardHandle> {
        let index = self.positions.remove(&handle)?;
        let removed = self.handles.swap_remove(index);
        if let Some(swapped_handle) = self.handles.get(index) {
            self.positions.insert(*swapped_handle, index);
        }
        Some(removed)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Graveyard {
    storage: IndexedOrderedZone,
}

impl Graveyard {
    #[must_use]
    pub fn new() -> Self {
        Self {
            storage: IndexedOrderedZone::default(),
        }
    }

    pub fn add(&mut self, handle: PlayerCardHandle) {
        self.storage.push(handle);
    }

    #[must_use]
    pub const fn len(&self) -> usize {
        self.storage.len()
    }

    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.storage.is_empty()
    }

    pub fn iter(&self) -> impl Iterator<Item = &PlayerCardHandle> {
        self.storage.iter()
    }

    #[must_use]
    pub fn contains(&self, handle: PlayerCardHandle) -> bool {
        self.storage.contains(handle)
    }

    #[must_use]
    pub fn handle_at(&self, index: usize) -> Option<PlayerCardHandle> {
        self.storage.handle_at(index)
    }

    #[must_use]
    pub fn remove(&mut self, handle: PlayerCardHandle) -> Option<PlayerCardHandle> {
        self.storage.remove_preserving_order(handle)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Exile {
    storage: IndexedOrderedZone,
}

impl Exile {
    #[must_use]
    pub fn new() -> Self {
        Self {
            storage: IndexedOrderedZone::default(),
        }
    }

    pub fn add(&mut self, handle: PlayerCardHandle) {
        self.storage.push(handle);
    }

    #[must_use]
    pub const fn len(&self) -> usize {
        self.storage.len()
    }

    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.storage.is_empty()
    }

    pub fn iter(&self) -> impl Iterator<Item = &PlayerCardHandle> {
        self.storage.iter()
    }

    #[must_use]
    pub fn contains(&self, handle: PlayerCardHandle) -> bool {
        self.storage.contains(handle)
    }

    #[must_use]
    pub fn handle_at(&self, index: usize) -> Option<PlayerCardHandle> {
        self.storage.handle_at(index)
    }

    #[must_use]
    pub fn remove(&mut self, handle: PlayerCardHandle) -> Option<PlayerCardHandle> {
        self.storage.remove_preserving_order(handle)
    }
}

#[cfg(test)]
mod tests {
    //! Verifies ordered zone removal preserves visible order without suffix shifts.

    use super::{Hand, PlayerCardHandle};

    #[test]
    fn hand_removal_preserves_visible_order() {
        let mut hand = Hand::new();
        let first = PlayerCardHandle::new(1);
        let second = PlayerCardHandle::new(2);
        let third = PlayerCardHandle::new(3);

        hand.receive(vec![first, second, third]);
        assert_eq!(hand.remove(second), Some(second));

        let visible: Vec<_> = hand.iter().copied().collect();
        assert_eq!(visible, vec![first, third]);
        assert_eq!(hand.handle_at(0), Some(first));
        assert_eq!(hand.handle_at(1), Some(third));
    }

    #[test]
    fn hand_reuses_removed_slots_without_changing_visible_order() {
        let mut hand = Hand::new();
        let first = PlayerCardHandle::new(1);
        let second = PlayerCardHandle::new(2);
        let third = PlayerCardHandle::new(3);
        let fourth = PlayerCardHandle::new(4);

        hand.receive(vec![first, second, third]);
        assert_eq!(hand.remove(second), Some(second));
        hand.receive(vec![fourth]);

        let visible: Vec<_> = hand.iter().copied().collect();
        assert_eq!(visible, vec![first, third, fourth]);
        assert_eq!(hand.handle_at(2), Some(fourth));
    }
}
