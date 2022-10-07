mod handlers;
mod models;
mod routes;
mod state;

use std::sync::Arc;

use models::note::{Note, NoteKind};
use state::State;

#[tokio::main]
async fn main() {
    let state = Arc::new(State::new());
    state.add_note(
        Note::builder("la palabra".to_owned())
            .definitions(&["'a word'".to_owned()])
            .examples(&["Hay dos palabras en la frase 'buenos dias'.".to_owned()])
            .build(),
        NoteKind::TermAndDefinition,
    );

    let app = routes::create(state);
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
