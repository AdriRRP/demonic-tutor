use {
    crate::domain::play::ids::CardInstanceId,
    rand::seq::SliceRandom,
    std::collections::{HashMap, VecDeque},
};

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Library(VecDeque<CardInstanceId>);

impl Library {
    #[must_use]
    pub fn new(card_ids: Vec<CardInstanceId>) -> Self {
        Self(VecDeque::from(card_ids))
    }

    pub fn draw_one(&mut self) -> Option<CardInstanceId> {
        self.0.pop_front()
    }

    pub fn draw(&mut self, n: usize) -> Option<Vec<CardInstanceId>> {
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

    pub fn receive(&mut self, card_ids: Vec<CardInstanceId>) {
        self.0.extend(card_ids);
    }

    pub fn shuffle(&mut self) {
        let mut rng = rand::make_rng::<rand::rngs::StdRng>();
        self.0.make_contiguous().shuffle(&mut rng);
    }

    pub fn iter(&self) -> impl Iterator<Item = &CardInstanceId> {
        self.0.iter()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Hand {
    card_ids: Vec<CardInstanceId>,
    positions: HashMap<CardInstanceId, usize>,
}

impl Hand {
    #[must_use]
    pub fn new() -> Self {
        Self {
            card_ids: Vec::new(),
            positions: HashMap::new(),
        }
    }

    pub fn receive(&mut self, card_ids: Vec<CardInstanceId>) {
        for card_id in card_ids {
            let index = self.card_ids.len();
            self.positions.insert(card_id.clone(), index);
            self.card_ids.push(card_id);
        }
    }

    #[must_use]
    pub const fn len(&self) -> usize {
        self.card_ids.len()
    }

    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.card_ids.is_empty()
    }

    pub fn iter(&self) -> impl Iterator<Item = &CardInstanceId> {
        self.card_ids.iter()
    }

    #[must_use]
    pub fn contains(&self, card_id: &CardInstanceId) -> bool {
        self.positions.contains_key(card_id)
    }

    #[must_use]
    pub fn card_id_at(&self, index: usize) -> Option<&CardInstanceId> {
        self.card_ids.get(index)
    }

    /// Removes and returns all card ids from the hand, leaving it empty.
    pub fn drain_all(&mut self) -> Vec<CardInstanceId> {
        self.positions.clear();
        std::mem::take(&mut self.card_ids)
    }

    #[must_use]
    pub fn remove(&mut self, card_id: &CardInstanceId) -> Option<CardInstanceId> {
        let index = self.positions.remove(card_id)?;
        let removed = self.card_ids.remove(index);
        for shifted_index in index..self.card_ids.len() {
            self.positions
                .insert(self.card_ids[shifted_index].clone(), shifted_index);
        }
        Some(removed)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Battlefield {
    card_ids: Vec<CardInstanceId>,
    positions: HashMap<CardInstanceId, usize>,
}

impl Battlefield {
    #[must_use]
    pub fn new() -> Self {
        Self {
            card_ids: Vec::new(),
            positions: HashMap::new(),
        }
    }

    pub fn add(&mut self, card_id: CardInstanceId) {
        let index = self.card_ids.len();
        self.positions.insert(card_id.clone(), index);
        self.card_ids.push(card_id);
    }

    #[must_use]
    pub const fn len(&self) -> usize {
        self.card_ids.len()
    }

    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.card_ids.is_empty()
    }

    pub fn iter(&self) -> impl Iterator<Item = &CardInstanceId> {
        self.card_ids.iter()
    }

    #[must_use]
    pub fn contains(&self, card_id: &CardInstanceId) -> bool {
        self.positions.contains_key(card_id)
    }

    #[must_use]
    pub fn card_id_at(&self, index: usize) -> Option<&CardInstanceId> {
        self.card_ids.get(index)
    }

    #[must_use]
    pub fn remove(&mut self, card_id: &CardInstanceId) -> Option<CardInstanceId> {
        let index = self.positions.remove(card_id)?;
        let removed = self.card_ids.swap_remove(index);
        if let Some(swapped_card_id) = self.card_ids.get(index) {
            self.positions.insert(swapped_card_id.clone(), index);
        }
        Some(removed)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Graveyard {
    card_ids: Vec<CardInstanceId>,
    positions: HashMap<CardInstanceId, usize>,
}

impl Graveyard {
    #[must_use]
    pub fn new() -> Self {
        Self {
            card_ids: Vec::new(),
            positions: HashMap::new(),
        }
    }

    pub fn add(&mut self, card_id: CardInstanceId) {
        let index = self.card_ids.len();
        self.positions.insert(card_id.clone(), index);
        self.card_ids.push(card_id);
    }

    #[must_use]
    pub const fn len(&self) -> usize {
        self.card_ids.len()
    }

    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.card_ids.is_empty()
    }

    pub fn iter(&self) -> impl Iterator<Item = &CardInstanceId> {
        self.card_ids.iter()
    }

    #[must_use]
    pub fn contains(&self, card_id: &CardInstanceId) -> bool {
        self.positions.contains_key(card_id)
    }

    #[must_use]
    pub fn card_id_at(&self, index: usize) -> Option<&CardInstanceId> {
        self.card_ids.get(index)
    }

    #[must_use]
    pub fn remove(&mut self, card_id: &CardInstanceId) -> Option<CardInstanceId> {
        let index = self.positions.remove(card_id)?;
        let removed = self.card_ids.remove(index);
        for shifted_index in index..self.card_ids.len() {
            self.positions
                .insert(self.card_ids[shifted_index].clone(), shifted_index);
        }
        Some(removed)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Exile {
    card_ids: Vec<CardInstanceId>,
    positions: HashMap<CardInstanceId, usize>,
}

impl Exile {
    #[must_use]
    pub fn new() -> Self {
        Self {
            card_ids: Vec::new(),
            positions: HashMap::new(),
        }
    }

    pub fn add(&mut self, card_id: CardInstanceId) {
        let index = self.card_ids.len();
        self.positions.insert(card_id.clone(), index);
        self.card_ids.push(card_id);
    }

    #[must_use]
    pub const fn len(&self) -> usize {
        self.card_ids.len()
    }

    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.card_ids.is_empty()
    }

    pub fn iter(&self) -> impl Iterator<Item = &CardInstanceId> {
        self.card_ids.iter()
    }

    #[must_use]
    pub fn contains(&self, card_id: &CardInstanceId) -> bool {
        self.positions.contains_key(card_id)
    }

    #[must_use]
    pub fn card_id_at(&self, index: usize) -> Option<&CardInstanceId> {
        self.card_ids.get(index)
    }

    #[must_use]
    pub fn remove(&mut self, card_id: &CardInstanceId) -> Option<CardInstanceId> {
        let index = self.positions.remove(card_id)?;
        let removed = self.card_ids.remove(index);
        for shifted_index in index..self.card_ids.len() {
            self.positions
                .insert(self.card_ids[shifted_index].clone(), shifted_index);
        }
        Some(removed)
    }
}
