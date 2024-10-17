use axum::{
    extract::{Path, State},
    Json,
};
use tracing::info;

use crate::models::{
    Coords, OracleResponse, OracleRet, Reservoir, ReservoirMax, ReservoirMean, ReservoirMin,
    ReservoirNameRequest,
};

const BASE_URL: &str = "https://gd419a46b456aec-db2.adb.eu-madrid-1.oraclecloudapps.com/ords/backo";

const PAGE_SIZE: usize = 20;

// 'f_get_maximo_historico_agua/'

fn get_url(endpoint: &str) -> String {
    format!("{}/{}", BASE_URL, endpoint)
}

/// Get all reservoirs
pub async fn all_reservoirs(page: Option<Path<usize>>) -> Json<Vec<Reservoir>> {
    let offset = PAGE_SIZE * page.map(|x| x.0).unwrap_or(0);
    let endpoint = format!("v_listado_info?limit={}&offset={}", PAGE_SIZE, offset);
    let res: OracleResponse<Reservoir> = reqwest::get(get_url(&endpoint))
        .await
        .unwrap()
        .json()
        .await
        .unwrap();
    Json(res.items)
}

/// Get reservoirs within a given range
pub async fn reservoirs_within_range(Json(coordinates): Json<Coords>) -> Json<Vec<Reservoir>> {
    info!("Received coordinates: {:?}", coordinates);
    Json(vec![])
}

/// Get reservoir by its name
pub async fn reservoir_by_name(Path(name): Path<String>) -> Json<Option<Reservoir>> {
    info!("Received name: {}", name);
    Json(None)
}

/// Get reservoir's water history
pub async fn reservoir_history(
    State(client): State<reqwest::Client>,
    Json(name): Json<ReservoirNameRequest>,
) -> Json<Vec<f32>> {
    let url = get_url("f_get_historico_agua/");

    let res = client
        .post(url)
        .json(&name)
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

#[axum::debug_handler]
pub async fn reservoir_max(
    State(client): State<reqwest::Client>,
    Json(name): Json<ReservoirNameRequest>,
) -> Json<ReservoirMax> {
    let endpoint = "f_get_maximo_historico_agua/";
    let url = get_url(endpoint);
    let res: OracleRet<f32> = client
        .post(url)
        .json(&name)
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();
    info!("Max water level for {}: {}", name.p_embalse_nombre, res.ret);
    Json(ReservoirMax {
        name: name.p_embalse_nombre,
        max: res.ret,
    })
}

#[axum::debug_handler]
pub async fn reservoir_min(
    State(client): State<reqwest::Client>,
    Json(name): Json<ReservoirNameRequest>,
) -> Json<ReservoirMin> {
    let endpoint = "f_get_minimo_historico_agua/";
    let url = get_url(endpoint);
    let res: OracleRet<f32> = client
        .post(url)
        .json(&name)
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();
    info!("Min water level for {}: {}", name.p_embalse_nombre, res.ret);
    Json(ReservoirMin {
        name: name.p_embalse_nombre,
        min: res.ret,
    })
}

#[axum::debug_handler]
pub async fn reservoir_mean(
    State(client): State<reqwest::Client>,
    Json(name): Json<ReservoirNameRequest>,
) -> Json<ReservoirMean> {
    let endpoint = "f_get_minimo_historico_agua/";
    let url = get_url(endpoint);
    let res: OracleRet<f32> = client
        .post(url)
        .json(&name)
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();
    info!("Min water level for {}: {}", name.p_embalse_nombre, res.ret);
    Json(ReservoirMean {
        name: name.p_embalse_nombre,
        mean: res.ret,
    })
}
