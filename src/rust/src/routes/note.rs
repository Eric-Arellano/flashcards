use std::sync::Arc;

use axum::{routing, Router};

use crate::{handlers, State};

pub fn create(state: Arc<State>) -> Router {
    // POST /notes
    Router::new()
        .route(
            "/",
            routing::get({
                let state = Arc::clone(&state);
                move || handlers::note::get_all_notes(Arc::clone(&state))
            }),
        )
        .route(
            "/:id",
            routing::get({
                let state = Arc::clone(&state);
                move |path| handlers::note::get_note(path, Arc::clone(&state))
            }),
        )
}
