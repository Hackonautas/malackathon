use axum::{routing::get, Router};
use tracing::info;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    info!("Hello, world!");

    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", get(root));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn root() -> String {
    "hello".to_string()
}
