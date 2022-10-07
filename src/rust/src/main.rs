use axum::routing;

#[tokio::main]
async fn main() {
    let app = axum::Router::new().route("/", routing::get(|| async { "Flashcards!" }));
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
