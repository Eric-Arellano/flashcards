use std::sync::Arc;

use axum::extract::Path;
use axum::http::StatusCode;
use axum::response::ErrorResponse;
use axum::{response, Json};

use crate::models::card::{Card, CardId};

use crate::State;

pub async fn get_all_cards(state: Arc<State>) -> response::Result<Json<Vec<Card>>> {
    let cards_by_id = state
        .cards_by_id
        .lock()
        .map_err(|_| ErrorResponse::from(StatusCode::INTERNAL_SERVER_ERROR))?;
    let mut all_cards: Vec<Card> = cards_by_id.clone().into_values().into_iter().collect();
    all_cards.sort_by(|c1, c2| c1.id.cmp(&c2.id));
    Ok(Json(all_cards))
}

pub async fn get_card(Path(id): Path<u64>, state: Arc<State>) -> response::Result<Json<Card>> {
    let cards_by_id = state
        .cards_by_id
        .lock()
        .map_err(|_| ErrorResponse::from(StatusCode::INTERNAL_SERVER_ERROR))?;
    cards_by_id
        .get(&CardId(id))
        .map(|card| Json(card.clone()))
        .ok_or_else(|| ErrorResponse::from(StatusCode::NOT_FOUND))
}

#[cfg(test)]
mod test {
    use std::sync::Arc;

    use axum::http::StatusCode;
    use serde_json::json;

    use crate::handlers::test::{make_get_request, send_request};
    use crate::models::note::{CreateCardsKind, Note};
    use crate::State;

    #[tokio::test]
    async fn test_get_all_cards() {
        let state = Arc::new(State::new());
        let app = crate::routes::create(state.clone());

        let (status, body) = send_request(app.clone(), make_get_request("/api/cards")).await;
        assert_eq!(status, StatusCode::OK);
        assert_eq!(body, json!([]));

        state.add_note(Note::builder(
            "term1".to_owned(),
            CreateCardsKind::DefinitionOnly,
        ));
        state.add_note(Note::builder(
            "term2".to_owned(),
            CreateCardsKind::TermAndDefinition,
        ));
        let (status, body) = send_request(app.clone(), make_get_request("/api/cards")).await;
        assert_eq!(status, StatusCode::OK);
        assert_eq!(
            body,
            json!(
                [
                    {"id": 1, "kind": "definition", "note_id": 1},
                    {"id": 2, "kind": "term", "note_id": 2},
                    {"id": 3, "kind": "definition", "note_id": 2}
                ]
            )
        );
    }

    #[tokio::test]
    async fn test_get_card() {
        let state = Arc::new(State::new());
        let app = crate::routes::create(state.clone());

        let (status, _) = send_request(app.clone(), make_get_request("/api/cards/1")).await;
        assert_eq!(status, StatusCode::NOT_FOUND);

        state.add_note(Note::builder("term1".to_owned(), CreateCardsKind::TermOnly));
        let (status, body) = send_request(app.clone(), make_get_request("/api/cards/1")).await;
        assert_eq!(status, StatusCode::OK);
        assert_eq!(body, json!({"id": 1, "kind": "term", "note_id": 1}))
    }
}
