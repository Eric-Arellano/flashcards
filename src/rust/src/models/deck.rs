use crate::models::card::Card;

/// A collection of cards.
///
/// This is intentionally kept simple because the app only supports a single
/// global deck, even though API routes are structured to allow adding more.
#[derive(Clone, Debug, Hash)]
pub struct Deck {
    cards: Vec<Card>,
}

impl Deck {
    pub fn new() -> Self {
        Self { cards: vec![] }
    }

    pub fn add(&mut self, card: Card) {
        self.cards.push(card);
    }
}
