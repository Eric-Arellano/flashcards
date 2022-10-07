use std::sync::Arc;

use axum::Router;

use crate::State;

pub fn create(_state: Arc<State>) -> Router {
    // GET  /decks
    // GET  /decks/<id>
    Router::new()
}
