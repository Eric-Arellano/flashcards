use std::sync::Arc;

use axum::extract::Path;
use axum::http::{header, StatusCode};
use axum::response::ErrorResponse;
use axum::{response, Json};
use serde::Serialize;

use crate::models::card::Card;
use crate::models::note::{Note, NoteBuilder, NoteId};

use crate::State;

pub async fn get_all_notes(state: Arc<State>) -> response::Result<Json<Vec<Note>>> {
    let notes_by_id = state
        .notes_by_id
        .lock()
        .map_err(|_| ErrorResponse::from(StatusCode::INTERNAL_SERVER_ERROR))?;
    Ok(Json(
        notes_by_id.clone().into_values().into_iter().collect(),
    ))
}

pub async fn get_note(Path(id): Path<u64>, state: Arc<State>) -> response::Result<Json<Note>> {
    let notes_by_id = state
        .notes_by_id
        .lock()
        .map_err(|_| ErrorResponse::from(StatusCode::INTERNAL_SERVER_ERROR))?;
    notes_by_id
        .get(&NoteId(id))
        .map(|note| Json(note.clone()))
        .ok_or_else(|| ErrorResponse::from(StatusCode::NOT_FOUND))
}

#[derive(Serialize)]
pub struct NoteCreateResult {
    note: Note,
    cards: Vec<Card>,
}

pub async fn create_note(
    Json(payload): Json<NoteBuilder>,
    state: Arc<State>,
) -> impl response::IntoResponse {
    let (note, cards) = state.add_note(payload);
    (
        StatusCode::CREATED,
        [(header::LOCATION, format!("/api/notes/{}", note.id.0))],
        Json(NoteCreateResult { note, cards }),
    )
}
