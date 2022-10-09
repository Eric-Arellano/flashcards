use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Serialize, Ord, PartialOrd)]
pub struct NoteId(pub u64);

/// A Note holds all the information necessary to relate a term with its definition(s).
#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize)]
pub struct Note {
    pub id: NoteId,
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
    // TODO: store photos.
    // TODO: store created & modified datetimes.
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CreateCardsKind {
    TermOnly,
    DefinitionOnly,
    TermAndDefinition,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash, Deserialize)]
pub struct NoteBuilder {
    term: String,
    definitions: Vec<String>,
    examples: Vec<String>,
    notes: Vec<String>,
    clues: Vec<String>,
    cards_kind: CreateCardsKind,
}

impl NoteBuilder {
    #[allow(dead_code)]
    fn new(term: String, cards_kind: CreateCardsKind) -> Self {
        Self {
            term,
            cards_kind,
            definitions: vec![],
            examples: vec![],
            notes: vec![],
            clues: vec![],
        }
    }

    #[allow(dead_code)]
    pub fn definitions(mut self, definitions: &[String]) -> Self {
        self.definitions = definitions.into();
        self
    }

    #[allow(dead_code)]
    pub fn examples(mut self, examples: &[String]) -> Self {
        self.examples = examples.into();
        self
    }

    #[allow(dead_code)]
    pub fn notes(mut self, notes: &[String]) -> Self {
        self.notes = notes.into();
        self
    }

    #[allow(dead_code)]
    pub fn clues(mut self, clues: &[String]) -> Self {
        self.clues = clues.into();
        self
    }

    pub fn build(self, id: NoteId) -> (Note, CreateCardsKind) {
        let Self {
            term,
            definitions,
            examples,
            notes,
            clues,
            cards_kind,
        } = self;
        let note = Note {
            id,
            term,
            definitions,
            examples,
            notes,
            clues,
        };
        (note, cards_kind)
    }
}

impl Note {
    #[allow(dead_code)]
    pub fn builder(term: String, cards_kind: CreateCardsKind) -> NoteBuilder {
        NoteBuilder::new(term, cards_kind)
    }
}
