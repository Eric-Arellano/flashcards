use std::sync::Arc;

use axum::{routing, Router};

use crate::handlers;
use crate::State;

pub fn create(state: Arc<State>) -> Router {
    // GET  /cards
    // GET  /cards/<id>
    Router::new().route(
        "/",
        routing::get({
            let state = Arc::clone(&state);
            move || handlers::card::get_all_cards(Arc::clone(&state))
        }),
    )
}
