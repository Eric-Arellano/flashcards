mod card;
mod note;

use std::sync::Arc;

use axum::Router;

use crate::State;

pub fn create(state: Arc<State>) -> Router {
    let api_routes = Router::new()
        .nest("/cards", card::create(state.clone()))
        .nest("/notes", note::create(state));
    Router::new().nest("/api", api_routes)
}
