use std::collections::BTreeSet;
use std::sync::Arc;

pub enum NoteKind {
    TermOnly,
    DefinitionOnly,
    TermAndDefinition,
}

/// A Note holds all the information necessary to relate a term with its definition(s).
pub struct Note {
    /// The word or phrase.
    term: String,
    /// What the `term` means. Often, there will be more than one definition.
    definitions: Vec<String>,
    /// Examples using the `term`.
    examples: Vec<String>,
    /// E.g. grammar considerations.
    notes: Vec<String>,
    /// Any clues to be shown when only the definition is shown.
    clues: Vec<String>,
    /// What types of cards are associated with the note.
    kind: NoteKind,
    // TODO: store photos.
    // TODO: store created & modified datetimes.
}

pub enum CardKind {
    /// Show the user the term first.
    Term,
    /// Show the user the definition first.
    Definition,
}

pub struct Card {
    // The Arc ensures data integrity if a note has >1 associated card.
    pub note: Arc<Note>,
    pub kind: CardKind,
    // TODO: store review history.
}

/// A collection of cards.
///
/// This is intentionally kept simple because the app only supports a single
/// global deck, even though API routes are structured to allow adding more.
pub struct Deck {
    pub cards: BTreeSet<Card>,
}
