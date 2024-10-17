use axum::{
    routing::{get, post},
    Router,
};
use routes::{all_reservoirs, reservoir_by_id, reservoir_history, reservoirs_in_range};
use tracing::info;

mod models;
mod routes;
mod services;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    services::connect_to_db().await;

    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", get(root))
        .route("/reservoirs", get(all_reservoirs))
        .route("/reservoirs", post(reservoirs_in_range))
        .route("/reservoir/:id", get(reservoir_by_id))
        .route("/reservoir/:id/historical", get(reservoir_history));

    let port = std::env::var("PORT").unwrap_or_else(|_| "3000".to_string());

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port))
        .await
        .unwrap();

    info!("Listening on port {}", port);

    axum::serve(listener, app).await.unwrap();
}

async fn root() -> String {
    "hello".to_string()
}
