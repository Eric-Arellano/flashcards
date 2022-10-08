use std::sync::Arc;

use axum::extract::Path;
use axum::http::StatusCode;
use axum::response::ErrorResponse;
use axum::{response, Json};

use crate::models::card::{Card, CardId};

use crate::State;

pub async fn get_all_cards(state: Arc<State>) -> response::Result<Json<Vec<Card>>> {
    let deck = state
        .deck
        .lock()
        .map_err(|_| ErrorResponse::from(StatusCode::INTERNAL_SERVER_ERROR))?;
    Ok(Json(
        deck.cards_by_id.clone().into_values().into_iter().collect(),
    ))
}

pub async fn get_card(Path(id): Path<u64>, state: Arc<State>) -> response::Result<Json<Card>> {
    let deck = state
        .deck
        .lock()
        .map_err(|_| ErrorResponse::from(StatusCode::INTERNAL_SERVER_ERROR))?;
    deck.cards_by_id
        .get(&CardId(id))
        .map(|card| Json(card.clone()))
        .ok_or_else(|| ErrorResponse::from(StatusCode::NOT_FOUND))
}
