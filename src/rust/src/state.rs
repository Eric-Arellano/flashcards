use std::collections::HashMap;
use std::sync::Mutex;

use crate::models::card::{Card, CardId, CardKind};
use crate::models::note::{CreateCardsKind, Note, NoteBuilder, NoteId};

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

    pub fn add_note(&self, note_builder: NoteBuilder) -> (Note, Vec<Card>) {
        // Note: this lock is held for the whole function because creating notes and cards should
        // be done atomically.
        let mut notes_by_id = self.notes_by_id.lock().unwrap();
        let note_id = match notes_by_id.keys().max() {
            None => NoteId(1),
            Some(NoteId(x)) => NoteId(x + 1),
        };
        let (note, create_cards_kind) = note_builder.build(note_id);
        notes_by_id.insert(note.id, note.clone());

        let mut cards_by_id = self.cards_by_id.lock().unwrap();
        let last_card_id: u64 = match cards_by_id.keys().max() {
            None => 0,
            Some(CardId(x)) => *x,
        };

        let cards = match create_cards_kind {
            CreateCardsKind::TermOnly => vec![ Card::new(CardId(last_card_id + 1), note_id, CardKind::Term)],
            CreateCardsKind::DefinitionOnly => vec![ Card::new(CardId(last_card_id + 1), note_id, CardKind::Definition)],
            CreateCardsKind::TermAndDefinition => vec![
                Card::new(CardId(last_card_id + 1), note_id, CardKind::Term),
                Card::new(CardId(last_card_id + 2), note_id, CardKind::Definition),
            ],
        };
        for card in &cards {
            cards_by_id.insert(card.id, card.clone());
        }

        (note, cards)
    }
}
