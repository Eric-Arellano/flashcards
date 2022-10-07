mod models;
mod state;

use std::sync::Arc;

use axum::routing;

use crate::models::card::{Note, NoteKind};
use crate::state::State;

#[tokio::main]
async fn main() {
    let state = Arc::new(State::new());
    state.add_note(
        Note::builder("la palabra".to_owned(), NoteKind::TermAndDefinition)
            .definitions(&["'a word'".to_owned()])
            .examples(&["Hay dos palabras en la frase 'buenos dias'.".to_owned()])
            .build(),
    );

    let app = axum::Router::new().route(
        "/",
        routing::get({
            let state = Arc::clone(&state);
            move || get_deck(Arc::clone(&state))
        }),
    );
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn get_deck(state: Arc<State>) -> String {
    format!("{state:?}")
}
