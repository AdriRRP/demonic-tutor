use {
    crate::domain::{cards::CardInstance, ids::CardInstanceId},
    rand::seq::SliceRandom,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Library(Vec<CardInstance>);

impl Library {
    #[must_use]
    pub const fn new(cards: Vec<CardInstance>) -> Self {
        Self(cards)
    }

    pub fn draw(&mut self, n: usize) -> Option<Vec<CardInstance>> {
        if self.0.len() >= n {
            Some(self.0.drain(0..n).collect())
        } else {
            None
        }
    }

    #[must_use]
    pub const fn len(&self) -> usize {
        self.0.len()
    }

    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn receive(&mut self, cards: Vec<CardInstance>) {
        self.0.extend(cards);
    }

    pub fn shuffle(&mut self) {
        let mut rng = rand::make_rng::<rand::rngs::StdRng>();
        self.0.shuffle(&mut rng);
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Hand(Vec<CardInstance>);

impl Hand {
    #[must_use]
    pub const fn new() -> Self {
        Self(Vec::new())
    }

    pub fn receive(&mut self, cards: Vec<CardInstance>) {
        self.0.extend(cards);
    }

    #[must_use]
    pub fn cards(&self) -> &[CardInstance] {
        &self.0
    }

    #[must_use]
    pub fn remove(&mut self, card_id: &CardInstanceId) -> Option<CardInstance> {
        self.0
            .iter()
            .position(|c| c.id() == card_id)
            .map(|i| self.0.remove(i))
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
    pub fn cards(&self) -> &[CardInstance] {
        &self.0
    }

    pub const fn cards_mut(&mut self) -> &mut Vec<CardInstance> {
        &mut self.0
    }

    pub fn card_mut(&mut self, card_id: &CardInstanceId) -> Option<&mut CardInstance> {
        self.0.iter_mut().find(|c| c.id() == card_id)
    }
}
