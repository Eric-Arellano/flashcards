use std::sync::Arc;

use axum::Router;

use crate::State;

pub fn create(_state: Arc<State>) -> Router {
    // GET  /notes
    // POST /notes
    // GET /notes/<id>
    Router::new()
}
