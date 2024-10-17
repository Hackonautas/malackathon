use axum::Json;
use tracing::info;

use crate::models::{Coords, OracleResponse, Reservoir};

/// Get all reservoirs
pub async fn all_reservoirs() -> Json<Vec<Reservoir>> {
    let stuff: OracleResponse<Reservoir> = reqwest::get(
        "https://gd419a46b456aec-db2.adb.eu-madrid-1.oraclecloudapps.com/ords/backo/v_listado_info/",
    )
    .await
    .unwrap()
    .json()
    .await
    .unwrap();
    Json(stuff.items)
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
