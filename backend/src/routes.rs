use axum::{extract::Path, Json};
use tracing::info;

use crate::models::{Coords, OracleResponse, Reservoir, ReservoirHistoryRequest};

const BASE_URL: &str = "https://gd419a46b456aec-db2.adb.eu-madrid-1.oraclecloudapps.com/ords/backo";

// 'f_get_maximo_historico_agua/'

fn get_url(endpoint: &str) -> String {
    format!("{}/{}", BASE_URL, endpoint)
}

/// Get all reservoirs
pub async fn all_reservoirs() -> Json<Vec<Reservoir>> {
    let res: OracleResponse<Reservoir> = reqwest::get(get_url("v_listado_info/"))
        .await
        .unwrap()
        .json()
        .await
        .unwrap();
    Json(res.items)
}

/// Get reservoirs in range
pub async fn reservoirs_in_range(Json(coordinates): Json<Coords>) -> Json<Vec<Reservoir>> {
    info!("Received coordinates: {:?}", coordinates);
    Json(vec![])
}

/// Get reservoir by ID
pub async fn reservoir_by_name(Path(name): Path<String>) -> Json<Option<Reservoir>> {
    info!("Received name: {}", name);
    Json(None)
}

/// Get reservoir history
pub async fn reservoir_history(Path(name): Path<String>) -> Json<Vec<f32>> {
    let url = get_url("f_get_historico_agua/");
    let client = reqwest::ClientBuilder::new().build().unwrap();

    let res = client
        .post(url)
        .json(&ReservoirHistoryRequest {
            p_embalse_nombre: name,
        })
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    eprintln!("{res:?}");

    todo!()
    // Json(res.items)
}
