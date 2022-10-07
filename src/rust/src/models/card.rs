use crate::models::note::NoteId;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct CardId(pub u64);

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum CardKind {
    /// Show the user the term first.
    Term,
    /// Show the user the definition first.
    Definition,
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct Card {
    pub id: CardId,
    note_id: NoteId,
    kind: CardKind,
    // TODO: store review history.
}

impl Card {
    pub fn new(id: CardId, note_id: NoteId, kind: CardKind) -> Self {
        Self { id, note_id, kind }
    }
}
