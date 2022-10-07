use std::sync::Arc;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum NoteKind {
    TermOnly,
    DefinitionOnly,
    TermAndDefinition,
}

/// A Note holds all the information necessary to relate a term with its definition(s).
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
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

pub struct NoteBuilder {
    term: String,
    definitions: Vec<String>,
    examples: Vec<String>,
    notes: Vec<String>,
    clues: Vec<String>,
    kind: NoteKind,
}

impl NoteBuilder {
    fn new(term: String, kind: NoteKind) -> Self {
        Self {
            term,
            definitions: vec![],
            examples: vec![],
            notes: vec![],
            clues: vec![],
            kind,
        }
    }

    pub fn definitions(mut self, definitions: &[String]) -> Self {
        self.definitions = definitions.into();
        self
    }
    pub fn examples(mut self, examples: &[String]) -> Self {
        self.examples = examples.into();
        self
    }
    pub fn notes(mut self, notes: &[String]) -> Self {
        self.notes = notes.into();
        self
    }
    pub fn clues(mut self, clues: &[String]) -> Self {
        self.clues = clues.into();
        self
    }

    pub fn build(self) -> Note {
        let Self {
            term,
            definitions,
            examples,
            notes,
            clues,
            kind,
        } = self;
        Note {
            term,
            definitions,
            examples,
            notes,
            clues,
            kind,
        }
    }
}

impl Note {
    pub fn builder(term: String, kind: NoteKind) -> NoteBuilder {
        NoteBuilder::new(term, kind)
    }

    pub fn into_cards(self) -> Vec<Card> {
        let note_kind = self.kind;
        let note = Arc::new(self);
        match note_kind {
            NoteKind::TermOnly => vec![Card::new(note, CardKind::Term)],
            NoteKind::DefinitionOnly => vec![Card::new(note, CardKind::Definition)],
            NoteKind::TermAndDefinition => vec![
                Card::new(note.clone(), CardKind::Term),
                Card::new(note, CardKind::Definition),
            ],
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum CardKind {
    /// Show the user the term first.
    Term,
    /// Show the user the definition first.
    Definition,
}

#[derive(Clone, Debug, Hash)]
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

/// A collection of cards.
///
/// This is intentionally kept simple because the app only supports a single
/// global deck, even though API routes are structured to allow adding more.
#[derive(Clone, Debug, Hash)]
pub struct Deck {
    cards: Vec<Card>,
}

impl Deck {
    pub fn new() -> Self {
        Self { cards: vec![] }
    }

    pub fn add(&mut self, card: Card) {
        self.cards.push(card);
    }
}
