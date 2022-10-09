use std::collections::HashMap;
use std::sync::Mutex;

use crate::models::card::{Card, CardId, CardKind};
use crate::models::note::{Note, NoteBuilder, NoteId};

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

    pub fn add_note(&self, note_builder: NoteBuilder) {
        let note_id = {
            let mut notes_by_id = self.notes_by_id.lock().unwrap();
            let note_id = match notes_by_id.keys().max() {
                None => NoteId(1),
                Some(NoteId(x)) => NoteId(x + 1),
            };
            let note = note_builder.build(note_id);
            notes_by_id.insert(note.id, note);
            note_id
        };

        let mut cards_by_id = self.cards_by_id.lock().unwrap();
        let last_card_id: u64 = match cards_by_id.keys().max() {
            None => 0,
            Some(CardId(x)) => *x,
        };
        let cards = vec![
            Card::new(CardId(last_card_id + 1), note_id, CardKind::Term),
            Card::new(CardId(last_card_id + 2), note_id, CardKind::Definition),
        ];
        for card in cards {
            cards_by_id.insert(card.id, card);
        }
    }
}
