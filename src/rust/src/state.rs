use std::sync::Mutex;

use crate::models::card::{Deck, Note};

#[derive(Debug)]
pub struct State {
    pub deck: Mutex<Deck>,
}

impl State {
    pub fn new() -> Self {
        Self {
            deck: Mutex::new(Deck::new()),
        }
    }

    pub fn add_note(&self, note: Note) {
        let mut deck = self.deck.lock().unwrap();
        for c in note.into_cards() {
            deck.add(c);
        }
    }
}
