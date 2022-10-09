use std::collections::{HashMap, HashSet};
use std::sync::Mutex;

use crate::models::card::{Card, CardId, CardKind};
use crate::models::note::{CreateCardsKind, Note, NoteBuilder, NoteId};

#[derive(Debug)]
pub struct State {
    pub cards_by_id: Mutex<HashMap<CardId, Card>>,
    pub notes_by_id: Mutex<HashMap<NoteId, Note>>,
    defined_terms: Mutex<HashSet<String>>,
}

impl State {
    pub fn new() -> Self {
        Self {
            cards_by_id: Mutex::new(HashMap::new()),
            notes_by_id: Mutex::new(HashMap::new()),
            defined_terms: Mutex::new(HashSet::new()),
        }
    }

    /// Save the note & its associated cards to the in-memory state.
    ///
    /// Returns None if the note was already created, based on the `term`.
    pub fn add_note(&self, note_builder: NoteBuilder) -> Option<(Note, Vec<Card>)> {
        // Note: the locks are held for the whole function because creating notes and cards should
        // be done atomically.
        let mut defined_terms = self.defined_terms.lock().unwrap();
        if defined_terms.contains(&note_builder.term) {
            return None;
        }
        defined_terms.insert(note_builder.term.clone());

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
            CreateCardsKind::TermOnly => {
                vec![Card::new(CardId(last_card_id + 1), note_id, CardKind::Term)]
            }
            CreateCardsKind::DefinitionOnly => vec![Card::new(
                CardId(last_card_id + 1),
                note_id,
                CardKind::Definition,
            )],
            CreateCardsKind::TermAndDefinition => vec![
                Card::new(CardId(last_card_id + 1), note_id, CardKind::Term),
                Card::new(CardId(last_card_id + 2), note_id, CardKind::Definition),
            ],
        };
        for card in &cards {
            cards_by_id.insert(card.id, card.clone());
        }

        Some((note, cards))
    }
}
