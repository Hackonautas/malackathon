use axum::Json;
use tracing::info;

use crate::models::{Coords, Reservoir};

/// Get all reservoirs
pub async fn all_reservoirs() -> Json<Vec<Reservoir>> {
    Json(vec![])
}

/// Get reservoirs in range
pub async fn reservoirs_in_range(Json(coordinates): Json<Coords>) -> Json<Vec<Reservoir>> {
    info!("Received coordinates: {:?}", coordinates);
    Json(vec![])
}

/// Get reservoir by ID
pub async fn reservoir_by_id() -> Json<Option<Reservoir>> {
    Json(None)
}

/// Get reservoir history
pub async fn reservoir_history() -> Json<Vec<f32>> {
    Json(vec![])
}
