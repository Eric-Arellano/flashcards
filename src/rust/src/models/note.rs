use std::sync::Arc;

use crate::models::card::{Card, CardKind};

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

    pub fn build(self) -> Note {
        let Self {
            term,
            definitions,
            examples,
            notes,
            clues,
        } = self;
        Note {
            term,
            definitions,
            examples,
            notes,
            clues,
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum NoteKind {
    TermOnly,
    DefinitionOnly,
    TermAndDefinition,
}

impl Note {
    pub fn builder(term: String) -> NoteBuilder {
        NoteBuilder::new(term)
    }

    pub fn into_cards(self, kind: NoteKind) -> Vec<Card> {
        let note = Arc::new(self);
        match kind {
            NoteKind::TermOnly => vec![Card::new(note, CardKind::Term)],
            NoteKind::DefinitionOnly => vec![Card::new(note, CardKind::Definition)],
            NoteKind::TermAndDefinition => vec![
                Card::new(note.clone(), CardKind::Term),
                Card::new(note, CardKind::Definition),
            ],
        }
    }
}

#[cfg(test)]
mod test {
    use std::sync::Arc;

    use super::{Card, CardKind, Note, NoteKind};

    #[test]
    fn test_note_into_cards() {
        let note = Note::builder("term1".to_owned()).build();
        let note_arc = Arc::new(note.clone());
        assert_eq!(
            note.clone().into_cards(NoteKind::TermOnly),
            vec![Card::new(note_arc.clone(), CardKind::Term)]
        );
        assert_eq!(
            note.clone().into_cards(NoteKind::DefinitionOnly),
            vec![Card::new(note_arc.clone(), CardKind::Definition)]
        );
        assert_eq!(
            note.clone().into_cards(NoteKind::TermAndDefinition),
            vec![
                Card::new(note_arc.clone(), CardKind::Term),
                Card::new(note_arc.clone(), CardKind::Definition)
            ]
        );
    }
}
