use std::collections::HashMap;
use std::sync::Mutex;

use crate::models::card::{Card, CardId, CardKind};
use crate::models::note::{Note, NoteId};

#[derive(Debug)]
pub struct State {
    pub cards_by_id: Mutex<HashMap<CardId, Card>>,
    pub notes_by_id: Mutex<HashMap<NoteId, Note>>,
}

impl State {
    pub fn new() -> Self {
        Self {
            cards_by_id: Mutex::new(HashMap::new()),
            notes_by_id: Mutex::new(HashMap::new()),
        }
    }

    pub fn add_note(&self, note: Note) {
        let cards = vec![
            Card::new(CardId(1), note.id, CardKind::Term),
            Card::new(CardId(2), note.id, CardKind::Definition),
        ];

        let mut notes_by_id = self.notes_by_id.lock().unwrap();
        notes_by_id.insert(note.id, note);

        let mut cards_by_id = self.cards_by_id.lock().unwrap();
        for card in cards {
            cards_by_id.insert(card.id, card);
        }
    }
}
