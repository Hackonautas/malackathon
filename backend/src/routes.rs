use axum::Json;

use crate::models::Reservoir;

pub async fn all_reservoirs() -> Json<Vec<Reservoir>> {
    Json(vec![])
}

pub async fn reservoir_by_id() -> Json<Option<Reservoir>> {
    Json(None)
}

pub async fn reservoir_history() -> Json<Vec<f32>> {
    Json(vec![])
}
