use std::sync::Arc;

use axum::extract::Path;
use axum::http::{header, StatusCode};
use axum::response::{ErrorResponse, IntoResponse};
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
    let mut all_notes: Vec<Note> = notes_by_id.clone().into_values().into_iter().collect();
    all_notes.sort_by(|n1, n2| n1.id.cmp(&n2.id));
    Ok(Json(all_notes))
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

pub async fn create_note(Json(payload): Json<NoteBuilder>, state: Arc<State>) -> impl IntoResponse {
    match state.add_note(payload) {
        None => StatusCode::CONFLICT.into_response(),
        Some((note, cards)) => (
            StatusCode::CREATED,
            [(header::LOCATION, format!("/api/notes/{}", note.id.0))],
            Json(NoteCreateResult { note, cards }),
        )
            .into_response(),
    }
}

#[cfg(test)]
mod test {
    use std::sync::Arc;

    use axum::http::StatusCode;
    use hyper::Body;
    use serde_json::json;

    use crate::handlers::test::{make_get_request, make_post_request, send_request};
    use crate::models::note::{CreateCardsKind, Note};
    use crate::State;

    #[tokio::test]
    async fn test_get_all_cards() {
        let state = Arc::new(State::new());
        let app = crate::routes::create(state.clone());

        let (status, body) = send_request(app.clone(), make_get_request("/api/notes")).await;
        assert_eq!(status, StatusCode::OK);
        assert_eq!(body, json!([]));

        state.add_note(
            Note::builder("term1".to_owned(), CreateCardsKind::TermOnly)
                .definitions(&["def".to_owned()]),
        );
        state.add_note(
            Note::builder("term2".to_owned(), CreateCardsKind::TermOnly)
                .examples(&["ex".to_owned()]),
        );
        let (status, body) = send_request(app.clone(), make_get_request("/api/notes")).await;
        assert_eq!(status, StatusCode::OK);
        assert_eq!(
            body,
            json!(
                [
                    {"id": 1, "term": "term1", "definitions": ["def"], "examples": [], "notes": [], "clues": []},
                    {"id": 2, "term": "term2", "definitions": [], "examples": ["ex"], "notes": [], "clues": []},
                ]
            )
        );
    }

    #[tokio::test]
    async fn test_get_card() {
        let state = Arc::new(State::new());
        let app = crate::routes::create(state.clone());

        let (status, _) = send_request(app.clone(), make_get_request("/api/notes/1")).await;
        assert_eq!(status, StatusCode::NOT_FOUND);

        state.add_note(Note::builder("term1".to_owned(), CreateCardsKind::TermOnly));
        let (status, body) = send_request(app.clone(), make_get_request("/api/notes/1")).await;
        assert_eq!(status, StatusCode::OK);
        assert_eq!(
            body,
            json!({"id": 1, "term": "term1", "definitions": [], "examples": [], "notes": [], "clues": []})
        )
    }

    #[tokio::test]
    async fn test_create_card() {
        let state = Arc::new(State::new());
        let app = crate::routes::create(state.clone());

        let request_body = serde_json::to_string(
            &Note::builder("term".to_owned(), CreateCardsKind::TermAndDefinition)
                .definitions(&["def".to_owned()]),
        )
        .unwrap();
        let (status, body) = send_request(
            app.clone(),
            make_post_request("/api/notes", Body::from(request_body.clone())),
        )
        .await;
        assert_eq!(status, StatusCode::CREATED);
        assert_eq!(
            body,
            json!({
                "note": {"id": 1, "term": "term", "definitions": ["def".to_owned()], "examples": [], "notes": [], "clues": []},
                "cards": [
                    {"id": 1, "kind": "term", "note_id": 1},
                    {"id": 2, "kind": "definition", "note_id": 1},
                ]
            })
        );
        {
            let notes_by_id = state.notes_by_id.lock().unwrap();
            let cards_by_id = state.cards_by_id.lock().unwrap();
            assert_eq!(notes_by_id.len(), 1);
            assert_eq!(cards_by_id.len(), 2);
        }

        let (status, _) = send_request(
            app.clone(),
            make_post_request("/api/notes", Body::from(request_body)),
        )
        .await;
        assert_eq!(status, StatusCode::CONFLICT);
        {
            let notes_by_id = state.notes_by_id.lock().unwrap();
            let cards_by_id = state.cards_by_id.lock().unwrap();
            assert_eq!(notes_by_id.len(), 1);
            assert_eq!(cards_by_id.len(), 2);
        }
    }
}
