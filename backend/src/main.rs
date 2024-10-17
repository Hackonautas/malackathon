use axum::{routing::get, Router};
use routes::{all_reservoirs, reservoir_by_id, reservoir_history};

mod models;
mod routes;
mod services;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", get(root))
        .route("/reservoirs", get(all_reservoirs))
        .route("/reservoir/:id", get(reservoir_by_id))
        .route("/reservoir/:id/historical", get(reservoir_history));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}

async fn root() -> String {
    "hello".to_string()
}
