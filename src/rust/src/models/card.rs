use std::sync::Arc;

use crate::models::note::Note;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum CardKind {
    /// Show the user the term first.
    Term,
    /// Show the user the definition first.
    Definition,
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct Card {
    // The Arc ensures data integrity if a note has >1 associated card.
    note: Arc<Note>,
    kind: CardKind,
    // TODO: store review history.
}

impl Card {
    pub fn new(note: Arc<Note>, kind: CardKind) -> Self {
        Self { note, kind }
    }
}
