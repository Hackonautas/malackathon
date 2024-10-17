use axum::{
    routing::{get, post},
    Router,
};
use routes::{all_reservoirs, reservoir_by_name, reservoir_history, reservoirs_in_range};
use tower_http::cors::CorsLayer;
use tracing::info;

mod models;
mod routes;
mod services;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let port = std::env::var("PORT").unwrap_or_else(|_| "3000".to_string());

    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", get(root))
        .route("/reservoirs/:page", get(all_reservoirs))
        .route("/reservoirs", post(reservoirs_in_range))
        .route("/reservoir/:name", get(reservoir_by_name))
        .route("/reservoir/:id/historical", get(reservoir_history))
        .layer(CorsLayer::permissive());

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port))
        .await
        .unwrap();

    info!("Listening on port {}", port);

    axum::serve(listener, app).await.unwrap();
}

async fn root() -> String {
    "hello".to_string()
}
