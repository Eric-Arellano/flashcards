use std::sync::Mutex;

use crate::models::card::{Card, CardId, CardKind};
use crate::models::deck::{Deck, DeckId};
use crate::models::note::Note;

#[derive(Debug)]
pub struct State {
    pub deck: Mutex<Deck>,
}

impl State {
    pub fn new() -> Self {
        Self {
            deck: Mutex::new(Deck::new(DeckId(1))),
        }
    }

    pub fn add_note(&self, note: Note) {
        let cards = vec![
            Card::new(CardId(1), note.id, CardKind::Term),
            Card::new(CardId(2), note.id, CardKind::Definition),
        ];
        let mut deck = self.deck.lock().unwrap();
        deck.add(note, cards);
    }
}
