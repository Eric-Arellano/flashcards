use std::sync::Arc;

use axum::{routing, Router};

use crate::handlers;
use crate::State;

pub fn create(state: Arc<State>) -> Router {
    Router::new()
        .route(
            "/",
            routing::get({
                let state = Arc::clone(&state);
                move || handlers::card::get_all_cards(Arc::clone(&state))
            }),
        )
        .route(
            "/:id",
            routing::get({
                let state = Arc::clone(&state);
                move |path| handlers::card::get_card(path, Arc::clone(&state))
            }),
        )
}
