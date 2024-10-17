use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Coords {
    lat: f64,
    lon: f64,
}

#[derive(Debug, Serialize)]
pub struct Reservoir {
    id: usize,
    code: usize,
    name: String,
    common_name: String,
    current: f32,
    capacity: f32,
    location: Coords,
    province: String,
    ccaa: String,
}
