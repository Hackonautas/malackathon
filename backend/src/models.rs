use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Coords {
    lat: f64,
    lon: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OracleResponse<T> {
    pub items: Vec<T>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReservoirHistoryRequest {
    pub p_embalse_nombre: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Reservoir {
    // id: usize,
    #[serde(rename = "codigo")]
    code: usize,
    #[serde(rename = "nombre")]
    name: String,
    #[serde(rename = "embalse")]
    common_name: String,
    // current: f32,
    // #[serde(rename = "agua_total")]
    // capacity: f32,
    // location: Coords,
    x: f64,
    y: f64,
    #[serde(rename = "provincia")]
    province: String,
    ccaa: String,
}
