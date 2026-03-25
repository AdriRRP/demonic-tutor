//! Supports domain play zones.

use {
    crate::domain::play::ids::PlayerCardHandle,
    rand::seq::SliceRandom,
    std::collections::{HashMap, VecDeque},
};

#[derive(Debug, Clone, PartialEq, Eq)]
struct IndexedOrderedZone {
    slots: Vec<Option<OrderedHandleSlot>>,
    handle_to_slot: HashMap<PlayerCardHandle, usize>,
    visible_slots: Vec<Option<usize>>,
    slot_to_visible: HashMap<usize, usize>,
    free_slots: Vec<usize>,
    visible_tree: Vec<usize>,
    visible_live_until: usize,
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

impl Default for IndexedOrderedZone {
    fn default() -> Self {
        Self {
            slots: Vec::new(),
            handle_to_slot: HashMap::new(),
            visible_slots: Vec::new(),
            slot_to_visible: HashMap::new(),
            free_slots: Vec::new(),
            visible_tree: vec![0],
            visible_live_until: 0,
            head: None,
            tail: None,
            len: 0,
        }
    }
}

impl IndexedOrderedZone {
    fn visible_tree_prefix_sum(&self, end_exclusive: usize) -> usize {
        let mut tree_index = end_exclusive;
        let mut sum = 0usize;

        while tree_index != 0 {
            sum += self.visible_tree[tree_index];
            let lowbit = tree_index & tree_index.wrapping_neg();
            tree_index -= lowbit;
        }

        sum
    }

    fn visible_tree_push_present(&mut self) {
        let one_based_index = self.visible_tree.len();
        self.visible_tree.push(0);

        let lowbit = one_based_index & one_based_index.wrapping_neg();
        let covered_prefix = self.visible_tree_prefix_sum(one_based_index - 1);
        let previous_prefix = self.visible_tree_prefix_sum(one_based_index - lowbit);
        self.visible_tree[one_based_index] = covered_prefix - previous_prefix + 1;
    }

    fn compact_visible_storage(&mut self) {
        let mut next_visible_slots = Vec::with_capacity(self.len);
        let mut next_slot_to_visible = HashMap::with_capacity(self.len);
        let mut current = self.head;

        while let Some(slot_index) = current {
            let Some(slot) = self.slots.get(slot_index).and_then(Option::as_ref) else {
                break;
            };
            next_slot_to_visible.insert(slot_index, next_visible_slots.len());
            next_visible_slots.push(Some(slot_index));
            current = slot.next;
        }

        self.visible_slots = next_visible_slots;
        self.slot_to_visible = next_slot_to_visible;
        self.visible_tree = vec![0];
        for _ in 0..self.visible_slots.len() {
            self.visible_tree_push_present();
        }
        self.visible_live_until = self.visible_slots.len();
    }

    fn trim_visible_tail(&mut self) {
        while self.visible_live_until != 0
            && self.visible_slots[self.visible_live_until - 1].is_none()
        {
            self.visible_slots.pop();
            self.visible_tree.pop();
            self.visible_live_until -= 1;
        }
    }

    fn visible_tree_add(&mut self, index: usize, delta: isize) {
        let mut tree_index = index + 1;
        while tree_index < self.visible_tree.len() {
            if delta.is_positive() {
                self.visible_tree[tree_index] += delta.unsigned_abs();
            } else {
                self.visible_tree[tree_index] -= delta.unsigned_abs();
            }
            let lowbit = tree_index & tree_index.wrapping_neg();
            tree_index += lowbit;
        }
    }

    fn visible_index_at(&self, visible_position: usize) -> Option<usize> {
        if visible_position >= self.len {
            return None;
        }

        let mut bit = 1usize;
        while bit < self.visible_tree.len() {
            bit <<= 1;
        }
        bit >>= 1;

        let mut tree_index = 0usize;
        let mut remaining = visible_position + 1;

        while bit != 0 {
            let next = tree_index + bit;
            if next < self.visible_tree.len() && self.visible_tree[next] < remaining {
                tree_index = next;
                remaining -= self.visible_tree[next];
            }
            bit >>= 1;
        }

        Some(tree_index)
    }

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
        let visible_index = self.visible_slots.len();
        self.slot_to_visible.insert(slot_index, visible_index);
        self.visible_slots.push(Some(slot_index));
        self.visible_tree_push_present();
        self.visible_live_until = self.visible_slots.len();
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
        let visible_index = self.visible_index_at(index)?;
        let slot_index = self.visible_slots.get(visible_index).copied().flatten()?;
        self.slots.get(slot_index)?.as_ref().map(|slot| slot.handle)
    }

    fn drain_all(&mut self) -> Vec<PlayerCardHandle> {
        let handles = self.iter().copied().collect();
        self.slots.clear();
        self.handle_to_slot.clear();
        self.visible_slots.clear();
        self.slot_to_visible.clear();
        self.free_slots.clear();
        self.visible_tree = vec![0];
        self.visible_live_until = 0;
        self.head = None;
        self.tail = None;
        self.len = 0;
        handles
    }

    fn remove_preserving_order(&mut self, handle: PlayerCardHandle) -> Option<PlayerCardHandle> {
        let slot_index = self.handle_to_slot.remove(&handle)?;
        let slot = self.slots.get_mut(slot_index)?.take()?;
        let visible_index = self.slot_to_visible.remove(&slot_index)?;

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

        self.visible_slots[visible_index] = None;
        self.visible_tree_add(visible_index, -1);
        if visible_index + 1 == self.visible_live_until {
            self.trim_visible_tail();
        } else if self.visible_slots.len() > self.len.saturating_mul(2).saturating_add(8) {
            self.compact_visible_storage();
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

    #[must_use]
    pub fn peek_one(&self) -> Option<PlayerCardHandle> {
        self.0.front().copied()
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

    pub fn move_top_to_bottom(&mut self) -> Option<PlayerCardHandle> {
        let handle = self.0.pop_front()?;
        self.0.push_back(handle);
        Some(handle)
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

    #[test]
    fn hand_visible_indexing_stays_stable_after_repeated_removals_and_insertions() {
        let mut hand = Hand::new();
        let first = PlayerCardHandle::new(1);
        let second = PlayerCardHandle::new(2);
        let third = PlayerCardHandle::new(3);
        let fourth = PlayerCardHandle::new(4);
        let fifth = PlayerCardHandle::new(5);

        hand.receive(vec![first, second, third, fourth]);
        assert_eq!(hand.remove(second), Some(second));
        hand.receive(vec![fifth]);
        assert_eq!(hand.remove(third), Some(third));

        let visible: Vec<_> = hand.iter().copied().collect();
        assert_eq!(visible, vec![first, fourth, fifth]);
        assert_eq!(hand.handle_at(0), Some(first));
        assert_eq!(hand.handle_at(1), Some(fourth));
        assert_eq!(hand.handle_at(2), Some(fifth));
        assert_eq!(hand.handle_at(3), None);
    }
}
