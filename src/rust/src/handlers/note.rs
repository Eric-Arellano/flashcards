use std::sync::Arc;

use axum::extract::Path;
use axum::http::StatusCode;
use axum::response::ErrorResponse;
use axum::{response, Json};

use crate::models::note::{Note, NoteId};

use crate::State;

pub async fn get_all_notes(state: Arc<State>) -> response::Result<Json<Vec<Note>>> {
    let deck = state
        .deck
        .lock()
        .map_err(|_| ErrorResponse::from(StatusCode::INTERNAL_SERVER_ERROR))?;
    Ok(Json(
        deck.notes_by_id.clone().into_values().into_iter().collect(),
    ))
}

pub async fn get_note(Path(id): Path<u64>, state: Arc<State>) -> response::Result<Json<Note>> {
    let deck = state
        .deck
        .lock()
        .map_err(|_| ErrorResponse::from(StatusCode::INTERNAL_SERVER_ERROR))?;
    deck.notes_by_id
        .get(&NoteId(id))
        .map(|note| Json(note.clone()))
        .ok_or_else(|| ErrorResponse::from(StatusCode::NOT_FOUND))
}
