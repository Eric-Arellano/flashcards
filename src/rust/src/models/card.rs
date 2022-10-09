use crate::models::note::NoteId;

use serde::Serialize;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Serialize, Ord, PartialOrd)]
pub struct CardId(pub u64);

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CardKind {
    /// Show the user the term first.
    Term,
    /// Show the user the definition first.
    Definition,
}

#[derive(Clone, Debug, Hash, PartialEq, Eq, Serialize)]
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
