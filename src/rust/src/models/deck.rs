use std::collections::HashMap;

use crate::models::card::{Card, CardId};
use crate::models::note::{Note, NoteId};

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct DeckId(pub u64);

/// A collection of notes and cards.
#[derive(Clone, Debug)]
pub struct Deck {
    pub id: DeckId,
    pub cards_by_id: HashMap<CardId, Card>,
    pub notes_by_id: HashMap<NoteId, Note>,
}

impl Deck {
    pub fn new(id: DeckId) -> Self {
        Self {
            id,
            cards_by_id: HashMap::new(),
            notes_by_id: HashMap::new(),
        }
    }

    pub fn add(&mut self, note: Note, cards: Vec<Card>) {
        self.notes_by_id.insert(note.id, note);
        for card in cards {
            self.cards_by_id.insert(card.id, card);
        }
    }
}
