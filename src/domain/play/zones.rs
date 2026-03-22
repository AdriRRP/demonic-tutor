use {
    crate::domain::play::{cards::CardInstance, ids::CardInstanceId},
    rand::seq::SliceRandom,
    std::collections::VecDeque,
};

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Library(VecDeque<CardInstance>);

impl Library {
    #[must_use]
    pub fn new(cards: Vec<CardInstance>) -> Self {
        Self(VecDeque::from(cards))
    }

    pub fn draw_one(&mut self) -> Option<CardInstance> {
        self.0.pop_front()
    }

    pub fn draw(&mut self, n: usize) -> Option<Vec<CardInstance>> {
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

    pub fn receive(&mut self, cards: Vec<CardInstance>) {
        self.0.extend(cards);
    }

    pub fn shuffle(&mut self) {
        let mut rng = rand::make_rng::<rand::rngs::StdRng>();
        self.0.make_contiguous().shuffle(&mut rng);
    }

    pub fn iter(&self) -> impl Iterator<Item = &CardInstance> {
        self.0.iter()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
struct OrderedZone(Vec<CardInstance>);

impl OrderedZone {
    const fn empty() -> Self {
        Self(Vec::new())
    }

    fn receive(&mut self, cards: Vec<CardInstance>) {
        self.0.extend(cards);
    }

    fn add(&mut self, card: CardInstance) {
        self.0.push(card);
    }

    const fn len(&self) -> usize {
        self.0.len()
    }

    const fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    fn iter(&self) -> impl Iterator<Item = &CardInstance> {
        self.0.iter()
    }

    fn card(&self, card_id: &CardInstanceId) -> Option<&CardInstance> {
        self.0.iter().find(|card| card.id() == card_id)
    }

    fn contains(&self, card_id: &CardInstanceId) -> bool {
        self.card(card_id).is_some()
    }

    fn cards(&self) -> &[CardInstance] {
        &self.0
    }

    fn remove(&mut self, card_id: &CardInstanceId) -> Option<CardInstance> {
        self.0
            .iter()
            .position(|card| card.id() == card_id)
            .map(|index| self.0.remove(index))
    }

    fn drain_all(&mut self) -> Vec<CardInstance> {
        std::mem::take(&mut self.0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Hand(OrderedZone);

impl Hand {
    #[must_use]
    pub const fn new() -> Self {
        Self(OrderedZone::empty())
    }

    pub fn receive(&mut self, cards: Vec<CardInstance>) {
        self.0.receive(cards);
    }

    #[must_use]
    pub const fn len(&self) -> usize {
        self.0.len()
    }

    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn iter(&self) -> impl Iterator<Item = &CardInstance> {
        self.0.iter()
    }

    #[must_use]
    pub fn card(&self, card_id: &CardInstanceId) -> Option<&CardInstance> {
        self.0.card(card_id)
    }

    #[must_use]
    pub fn contains(&self, card_id: &CardInstanceId) -> bool {
        self.0.contains(card_id)
    }

    #[must_use]
    pub fn cards(&self) -> &[CardInstance] {
        self.0.cards()
    }

    /// Removes and returns all cards from the hand, leaving it empty.
    pub fn drain_all(&mut self) -> Vec<CardInstance> {
        self.0.drain_all()
    }

    #[must_use]
    pub fn remove(&mut self, card_id: &CardInstanceId) -> Option<CardInstance> {
        self.0.remove(card_id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Battlefield(Vec<CardInstance>);

impl Battlefield {
    #[must_use]
    pub const fn new() -> Self {
        Self(Vec::new())
    }

    pub fn add(&mut self, card: CardInstance) {
        self.0.push(card);
    }

    #[must_use]
    pub const fn len(&self) -> usize {
        self.0.len()
    }

    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn iter(&self) -> impl Iterator<Item = &CardInstance> {
        self.0.iter()
    }

    #[must_use]
    pub fn cards(&self) -> &[CardInstance] {
        &self.0
    }

    #[must_use]
    pub fn card(&self, card_id: &CardInstanceId) -> Option<&CardInstance> {
        self.0.iter().find(|card| card.id() == card_id)
    }

    #[must_use]
    pub fn contains(&self, card_id: &CardInstanceId) -> bool {
        self.card(card_id).is_some()
    }

    pub fn card_mut(&mut self, card_id: &CardInstanceId) -> Option<&mut CardInstance> {
        self.0.iter_mut().find(|c| c.id() == card_id)
    }

    #[must_use]
    pub fn remove(&mut self, card_id: &CardInstanceId) -> Option<CardInstance> {
        self.0
            .iter()
            .position(|card| card.id() == card_id)
            .map(|index| self.0.swap_remove(index))
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut CardInstance> {
        self.0.iter_mut()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Graveyard(OrderedZone);

impl Graveyard {
    #[must_use]
    pub const fn new() -> Self {
        Self(OrderedZone::empty())
    }

    pub fn add(&mut self, card: CardInstance) {
        self.0.add(card);
    }

    #[must_use]
    pub const fn len(&self) -> usize {
        self.0.len()
    }

    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn iter(&self) -> impl Iterator<Item = &CardInstance> {
        self.0.iter()
    }

    #[must_use]
    pub fn card(&self, card_id: &CardInstanceId) -> Option<&CardInstance> {
        self.0.card(card_id)
    }

    #[must_use]
    pub fn contains(&self, card_id: &CardInstanceId) -> bool {
        self.0.contains(card_id)
    }

    #[must_use]
    pub fn cards(&self) -> &[CardInstance] {
        self.0.cards()
    }

    #[must_use]
    pub fn remove(&mut self, card_id: &CardInstanceId) -> Option<CardInstance> {
        self.0.remove(card_id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Exile(Vec<CardInstanceId>);

impl Exile {
    #[must_use]
    pub const fn new() -> Self {
        Self(Vec::new())
    }

    pub fn add(&mut self, card_id: CardInstanceId) {
        self.0.push(card_id);
    }

    #[must_use]
    pub const fn len(&self) -> usize {
        self.0.len()
    }

    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn iter(&self) -> impl Iterator<Item = &CardInstanceId> {
        self.0.iter()
    }

    #[must_use]
    pub fn contains(&self, card_id: &CardInstanceId) -> bool {
        self.0.iter().any(|stored_id| stored_id == card_id)
    }

    #[must_use]
    pub fn card_id_at(&self, index: usize) -> Option<&CardInstanceId> {
        self.0.get(index)
    }

    #[must_use]
    pub fn remove(&mut self, card_id: &CardInstanceId) -> Option<CardInstanceId> {
        self.0
            .iter()
            .position(|stored_id| stored_id == card_id)
            .map(|index| self.0.remove(index))
    }
}
