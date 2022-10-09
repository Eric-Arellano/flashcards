use serde::Serialize;

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

pub struct NoteBuilder {
    term: String,
    definitions: Vec<String>,
    examples: Vec<String>,
    notes: Vec<String>,
    clues: Vec<String>,
}

impl NoteBuilder {
    fn new(term: String) -> Self {
        Self {
            term,
            definitions: vec![],
            examples: vec![],
            notes: vec![],
            clues: vec![],
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

    pub fn build(self, id: NoteId) -> Note {
        let Self {
            term,
            definitions,
            examples,
            notes,
            clues,
        } = self;
        Note {
            id,
            term,
            definitions,
            examples,
            notes,
            clues,
        }
    }
}

impl Note {
    pub fn builder(term: String) -> NoteBuilder {
        NoteBuilder::new(term)
    }
}
