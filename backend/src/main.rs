use axum::{
    http::{HeaderValue, Method},
    routing::{get, post},
    Router,
};
use routes::{
    all_reservoirs, reservoir_history, reservoir_max, reservoir_mean, reservoir_min,
    reservoirs_within_range,
};
use tower_http::cors::CorsLayer;
use tracing::info;

mod models;
mod routes;
mod services;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let port = std::env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    let addr = format!("http://localhost:{port}");

    let client = reqwest::Client::new();

    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", get(root))
        .route("/reservoirs/:page", get(all_reservoirs))
        .route("/reservoirs", post(reservoirs_within_range))
        .route("/reservoir/max", post(reservoir_max))
        .route("/reservoir/min", post(reservoir_min))
        .route("/reservoir/mean", post(reservoir_mean))
        .route("/reservoir/historical", post(reservoir_history))
        .with_state(client)
        .layer(
            CorsLayer::new()
                .allow_origin(addr.parse::<HeaderValue>().unwrap())
                .allow_methods([Method::GET]),
        );

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port))
        .await
        .unwrap();

    info!("Listening on port {}", port);

    axum::serve(listener, app).await.unwrap();
}

async fn root() -> String {
    "hello".to_string()
}
