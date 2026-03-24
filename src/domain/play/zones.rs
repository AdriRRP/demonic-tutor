//! Supports domain play zones.

use {
    crate::domain::play::ids::PlayerCardHandle,
    rand::seq::SliceRandom,
    std::collections::{HashMap, VecDeque},
};

#[derive(Debug, Clone, PartialEq, Eq, Default)]
struct IndexedOrderedZone {
    handles: Vec<Option<PlayerCardHandle>>,
    positions: HashMap<PlayerCardHandle, usize>,
    len: usize,
}

impl IndexedOrderedZone {
    fn receive_many(&mut self, handles: Vec<PlayerCardHandle>) {
        for handle in handles {
            self.push(handle);
        }
    }

    fn push(&mut self, handle: PlayerCardHandle) {
        let index = self.handles.len();
        self.positions.insert(handle, index);
        self.handles.push(Some(handle));
        self.len += 1;
    }

    const fn len(&self) -> usize {
        self.len
    }

    const fn is_empty(&self) -> bool {
        self.len == 0
    }

    fn iter(&self) -> impl Iterator<Item = &PlayerCardHandle> {
        self.handles.iter().filter_map(Option::as_ref)
    }

    fn contains(&self, handle: PlayerCardHandle) -> bool {
        self.positions.contains_key(&handle)
    }

    fn handle_at(&self, index: usize) -> Option<PlayerCardHandle> {
        self.handles.iter().filter_map(|handle| *handle).nth(index)
    }

    fn drain_all(&mut self) -> Vec<PlayerCardHandle> {
        self.positions.clear();
        self.len = 0;
        std::mem::take(&mut self.handles)
            .into_iter()
            .flatten()
            .collect()
    }

    fn remove_preserving_order(&mut self, handle: PlayerCardHandle) -> Option<PlayerCardHandle> {
        let index = self.positions.remove(&handle)?;
        let removed = self.handles.get_mut(index)?.take()?;
        self.len -= 1;
        Some(removed)
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
